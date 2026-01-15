"""Logger Service - Logging with OpenTelemetry."""

from __future__ import annotations

import abc
import logging
import sys
import typing as T

import loguru
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


class PropagateHandler(logging.Handler):
    def emit(self, record: logging.LogRecord) -> None:
        logging.getLogger(record.name).handle(record)


class Service(abc.ABC, pdt.BaseModel, strict=True, frozen=True, extra="forbid"):
    """Base class for a global service."""

    @abc.abstractmethod
    def start(self) -> None:
        """Start the service."""

    def stop(self) -> None:
        """Stop the service."""


class LoggerService(Service):
    """Service for logging messages.

    https://loguru.readthedocs.io/en/stable/api/logger.html
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
        service_name = "Autogen Team"
        resource = Resource.create({"service.name": service_name})
        tracer_provider = TracerProvider(resource=resource)

        trace.set_tracer_provider(tracer_provider)
        otlp_trace_exporter = OTLPSpanExporter()
        tracer_provider.add_span_processor(BatchSpanProcessor(otlp_trace_exporter))

        logger_provider = LoggerProvider(resource=resource)
        set_logger_provider(logger_provider)
        otlp_log_exporter = OTLPLogExporter()
        logger_provider.add_log_record_processor(BatchLogRecordProcessor(otlp_log_exporter))

        otel_handler = LoggingHandler(level=logging.INFO, logger_provider=logger_provider)
        logging.getLogger().addHandler(otel_handler)

        logging.basicConfig(level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s")
        logger = logging.getLogger(service_name)
        logger.info("Logging started")
        loguru.logger.remove()
        config = self.model_dump()
        sinks = {"stderr": sys.stderr, "stdout": sys.stdout}
        config["sink"] = sinks.get(config["sink"], config["sink"])
        loguru.logger.add(**config)
        loguru.logger.add(PropagateHandler(), format="{message}")

    def logger(self) -> loguru.Logger:
        """Return the main logger."""
        return loguru.logger
