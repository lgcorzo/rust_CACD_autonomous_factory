"""Manage global context during execution."""

# %% IMPORTS

from __future__ import annotations

import abc
import contextlib as ctx
import logging
import os
import sys
import typing as T
from typing import ClassVar

import loguru
import mlflow
import mlflow.tracking as mt
import pydantic as pdt
from opentelemetry import trace
from opentelemetry._logs import set_logger_provider
from opentelemetry.exporter.otlp.proto.http._log_exporter import OTLPLogExporter
from opentelemetry.exporter.otlp.proto.http.trace_exporter import OTLPSpanExporter
from opentelemetry.sdk._logs import LoggerProvider, LoggingHandler
from opentelemetry.sdk._logs.export import BatchLogRecordProcessor
from opentelemetry.sdk.resources import Resource
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchSpanProcessor
from plyer import notification

from autogen_team.io.osvariables import Env


# %% SERVICES
class PropagateHandler(logging.Handler):
    def emit(self, record: logging.LogRecord) -> None:
        logging.getLogger(record.name).handle(record)


class Service(abc.ABC, pdt.BaseModel, strict=True, frozen=True, extra="forbid"):
    """Base class for a global service.

    Use services to manage global contexts.
    e.g., logger object, mlflow client, spark context, ...
    """

    @abc.abstractmethod
    def start(self) -> None:
        """Start the service."""

    def stop(self) -> None:
        """Stop the service."""
        # does nothing by default


class LoggerService(Service):
    """Service for logging messages.

    https://loguru.readthedocs.io/en/stable/api/logger.html

    Parameters:
        sink (str): logging output.
        level (str): logging level.
        format (str): logging format.
        colorize (bool): colorize output.
        serialize (bool): convert to JSON.
        backtrace (bool): enable exception trace.
        diagnose (bool): enable variable display.
        catch (bool): catch errors during log handling.
    """

    sink: str = "stderr"
    level: str = "DEBUG"
    format: str = (
        "<green>[{time:YYYY-MM-DD HH:mm:ss.SSS}]</green>"
        "<level>[{level}]</level>"
        "<cyan>[{name}:{function}:{line}]</cyan>"
        " <level>{message}</level>"
    )
    colorize: bool = True
    serialize: bool = False
    backtrace: bool = True
    diagnose: bool = False
    catch: bool = True

    def start(self) -> None:
        # Define the service name
        service_name = "Autogen Team"
        # Define the resource with service.name
        resource = Resource.create({"service.name": service_name})
        # Tracing setup
        tracer_provider = TracerProvider(resource=resource)

        trace.set_tracer_provider(tracer_provider)
        otlp_trace_exporter = OTLPSpanExporter()
        tracer_provider.add_span_processor(BatchSpanProcessor(otlp_trace_exporter))

        # Logging setup
        logger_provider = LoggerProvider(resource=resource)
        set_logger_provider(logger_provider)
        otlp_log_exporter = OTLPLogExporter()
        logger_provider.add_log_record_processor(BatchLogRecordProcessor(otlp_log_exporter))

        # Attach OpenTelemetry handler to Python's logging
        otel_handler = LoggingHandler(level=logging.INFO, logger_provider=logger_provider)
        logging.getLogger().addHandler(otel_handler)

        # Root logger configuration
        logging.basicConfig(level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s")
        logger = logging.getLogger(service_name)
        logger.info("Logging started")
        loguru.logger.remove()
        config = self.model_dump()
        # use standard sinks or keep the original
        sinks = {"stderr": sys.stderr, "stdout": sys.stdout}
        config["sink"] = sinks.get(config["sink"], config["sink"])
        loguru.logger.add(**config)
        loguru.logger.add(PropagateHandler(), format="{message}")

    def logger(self) -> loguru.Logger:
        """Return the main logger.

        Returns:
            loguru.Logger: the main logger.
        """
        return loguru.logger


class AlertsService(Service):
    """Service for sending notifications.

    Require libnotify-bin on Linux systems.

    In production, use with Slack, Discord, or emails.

    https://plyer.readthedocs.io/en/latest/api.html#plyer.facades.Notification

    Parameters:
        enable (bool): use notifications or print.
        app_name (str): name of the application.
        timeout (int | None): timeout in secs.
    """

    enable: bool = True
    app_name: str = "autogen_team"
    timeout: int | None = None

    def start(self) -> None:
        pass

    def notify(self, title: str, message: str) -> None:
        """Send a notification to the system.

        Args:
            title (str): title of the notification.
            message (str): message of the notification.
        """
        if self.enable:
            try:
                notification.notify(title=title, message=message, app_name=self.app_name, timeout=self.timeout)
            except Exception:
                print(f"[{self.app_name}] {title}: {message} (Notification ignored: No usable implementation)")
        else:
            print(f"[{self.app_name}] {title}: {message}")


class MlflowService(Service):
    """Service for Mlflow tracking and registry.

    Parameters:
        tracking_uri (str): the URI for the Mlflow tracking server.
        registry_uri (str): the URI for the Mlflow model registry.
        experiment_name (str): the name of tracking experiment.
        registry_name (str): the name of model registry.
        autolog_disable (bool): disable autologging.
        autolog_disable_for_unsupported_versions (bool): disable autologging for unsupported versions.
        autolog_exclusive (bool): If True, enables exclusive autologging.
        autolog_log_input_examples (bool): If True, logs input examples during autologging.
        autolog_log_model_signatures (bool): If True, logs model signatures during autologging.
        autolog_log_models (bool): If True, enables logging of models during autologging.
        autolog_log_datasets (bool): If True, logs datasets used during autologging.
        autolog_silent (bool): If True, suppresses all Mlflow warnings during autologging.
    """

    class RunConfig(pdt.BaseModel, strict=True, frozen=True, extra="forbid"):
        """Run configuration for Mlflow tracking.

        Parameters:
            name (str): name of the run.
            description (str | None): description of the run.
            tags (dict[str, T.Any] | None): tags for the run.
            log_system_metrics (bool | None): enable system metrics logging.
        """

        name: str
        description: str | None = None
        tags: dict[str, T.Any] | None = None
        log_system_metrics: bool | None = True

    # server uri
    env: ClassVar[Env] = Env()
    tracking_uri: str = env.mlflow_tracking_uri
    registry_uri: str = env.mlflow_registry_uri
    experiment_name: str = env.mlflow_experiment_name
    registry_name: str = env.mlflow_registered_model_name
    # autolog
    autolog_disable: bool = False
    autolog_disable_for_unsupported_versions: bool = False
    autolog_exclusive: bool = False
    autolog_log_input_examples: bool = True
    autolog_log_model_signatures: bool = True
    autolog_log_models: bool = False
    autolog_log_datasets: bool = False
    autolog_silent: bool = False

    def start(self) -> None:
        # server uri
        mlflow.set_tracking_uri(uri=self.tracking_uri)
        mlflow.set_registry_uri(uri=self.registry_uri)
        # experiment
        mlflow.set_experiment(experiment_name=self.experiment_name)
        # autolog
        mlflow.autolog(
            disable=self.autolog_disable,
            disable_for_unsupported_versions=self.autolog_disable_for_unsupported_versions,
            exclusive=self.autolog_exclusive,
            log_input_examples=self.autolog_log_input_examples,
            log_model_signatures=self.autolog_log_model_signatures,
            log_datasets=self.autolog_log_datasets,
            silent=self.autolog_silent,
        )
        # S3 credentials
        os.environ["AWS_ACCESS_KEY_ID"] = os.getenv("AWS_ACCESS_KEY_ID", self.env.aws_access_key_id)
        os.environ["AWS_SECRET_ACCESS_KEY"] = os.getenv("AWS_SECRET_ACCESS_KEY", self.env.aws_secret_access_key)
        os.environ["MLFLOW_S3_ENDPOINT_URL"] = os.getenv("MLFLOW_S3_ENDPOINT_URL", self.env.mlflow_s3_endpoint_url)
        os.environ["MLFLOW_S3_IGNORE_TLS"] = os.getenv(
            "MLFLOW_S3_IGNORE_TLS", str(self.env.mlflow_s3_ignore_tls).lower()
        )

    @ctx.contextmanager
    def run_context(self, run_config: RunConfig) -> T.Generator[mlflow.ActiveRun, None, None]:
        """Yield an active Mlflow run and exit it afterwards.

        Args:
            run (str): run parameters.

        Yields:
            T.Generator[mlflow.ActiveRun, None, None]: active run context. Will be closed as the end of context.
        """
        with mlflow.start_run(
            run_name=run_config.name,
            tags=run_config.tags,
            description=run_config.description,
            log_system_metrics=run_config.log_system_metrics,
        ) as run:
            yield run

    def client(self) -> mt.MlflowClient:
        """Return a new Mlflow client.

        Returns:
            MlflowClient: the mlflow client.
        """
        return mt.MlflowClient(tracking_uri=self.tracking_uri, registry_uri=self.registry_uri)
