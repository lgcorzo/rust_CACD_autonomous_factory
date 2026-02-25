## 2026-02-03 - [Secrets Pickled in MLflow Adapters]

**Vulnerability:** Found `os.getenv` call inside `PythonModel.__init__` in `mlflow_adapter.py`. This resolves the secret value at save time and pickles it into the model artifact, effectively hardcoding the secret in the binary.
**Learning:** `PythonModel` subclasses are pickled by MLflow. Configuration dependent on environment variables must be resolved at load time (e.g., in `load_context` or `predict`), never in `__init__`.
**Prevention:** Remove environment variable resolution from `__init__`. Use `load_context` to read configuration from artifacts or resolve environment variables at runtime.

## 2026-02-04 - Information Exposure in Kafka Service

**Vulnerability:** The Kafka consumer service (`kafka_app.py`) was logging raw input messages which could contain PII, and returning raw exception messages to the output topic which could leak internal implementation details.
**Learning:** In event-driven architectures like Kafka, error handling often involves producing to an error topic or the same output topic. Great care must be taken to sanitize these error messages. Also, logging "raw" messages for debugging is a common privacy trap.
**Prevention:** Always catch exceptions at the top level of the message processor, log the full stack trace securely (server-side), but return/produce only generic error codes or messages to the downstream systems. Sanitize input logs to exclude data fields.

## 2026-02-09 - [Secret Leakage in Pickled MLflow Adapters]

**Vulnerability:** Found a hardcoded `model_config` dictionary in `mlflow_adapter.py` that captured the `LITELLM_API_KEY` environment variable value at initialization time. This caused the secret API key to be stored in plain text within the pickled model artifact.
**Learning:** When using MLflow's `PythonModel`, any instance attribute set in `__init__` is serialized (pickled) with the model. Reading secrets into instance attributes during `__init__` permanently bakes them into the artifact, leaking them to anyone with access to the model file.
**Prevention:** Never store environment-dependent configuration or secrets in `__init__` of a `PythonModel`. Always load configuration dynamically in `load_context` or `predict`, or use the context object provided by MLflow at runtime.
