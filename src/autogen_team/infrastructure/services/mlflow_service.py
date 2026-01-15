"""MLflow Service - Tracking and Registry."""

from __future__ import annotations

import contextlib as ctx
import os
import typing as T
from typing import ClassVar

import mlflow
import mlflow.tracking as mt
import pydantic as pdt

from autogen_team.infrastructure.io.osvariables import Env
from .logger_service import Service


class MlflowService(Service):
    """Service for Mlflow tracking and registry."""

    class RunConfig(pdt.BaseModel, strict=True, frozen=True, extra="forbid"):
        """Run configuration for Mlflow tracking."""

        name: str
        description: str | None = None
        tags: dict[str, T.Any] | None = None
        log_system_metrics: bool | None = True

    env: ClassVar[Env] = Env()
    tracking_uri: str = env.mlflow_tracking_uri
    registry_uri: str = env.mlflow_registry_uri
    experiment_name: str = env.mlflow_experiment_name
    registry_name: str = env.mlflow_registered_model_name
    autolog_disable: bool = False
    autolog_disable_for_unsupported_versions: bool = False
    autolog_exclusive: bool = False
    autolog_log_input_examples: bool = True
    autolog_log_model_signatures: bool = True
    autolog_log_models: bool = False
    autolog_log_datasets: bool = False
    autolog_silent: bool = False

    def start(self) -> None:
        mlflow.set_tracking_uri(uri=self.tracking_uri)
        mlflow.set_registry_uri(uri=self.registry_uri)
        mlflow.set_experiment(experiment_name=self.experiment_name)
        mlflow.autolog(
            disable=self.autolog_disable,
            disable_for_unsupported_versions=self.autolog_disable_for_unsupported_versions,
            exclusive=self.autolog_exclusive,
            log_input_examples=self.autolog_log_input_examples,
            log_model_signatures=self.autolog_log_model_signatures,
            log_datasets=self.autolog_log_datasets,
            silent=self.autolog_silent,
        )
        os.environ["AWS_ACCESS_KEY_ID"] = os.getenv("AWS_ACCESS_KEY_ID", self.env.aws_access_key_id)
        os.environ["AWS_SECRET_ACCESS_KEY"] = os.getenv("AWS_SECRET_ACCESS_KEY", self.env.aws_secret_access_key)
        os.environ["MLFLOW_S3_ENDPOINT_URL"] = os.getenv("MLFLOW_S3_ENDPOINT_URL", self.env.mlflow_s3_endpoint_url)
        os.environ["MLFLOW_S3_IGNORE_TLS"] = os.getenv(
            "MLFLOW_S3_IGNORE_TLS", str(self.env.mlflow_s3_ignore_tls).lower()
        )

    @ctx.contextmanager
    def run_context(self, run_config: RunConfig) -> T.Generator[mlflow.ActiveRun, None, None]:
        """Yield an active Mlflow run and exit it afterwards."""
        with mlflow.start_run(
            run_name=run_config.name,
            tags=run_config.tags,
            description=run_config.description,
            log_system_metrics=run_config.log_system_metrics,
        ) as run:
            yield run

    def client(self) -> mt.MlflowClient:
        """Return a new Mlflow client."""
        return mt.MlflowClient(tracking_uri=self.tracking_uri, registry_uri=self.registry_uri)
