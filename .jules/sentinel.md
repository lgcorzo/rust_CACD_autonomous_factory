## 2026-02-03 - [Hardcoded Secrets in MLflow Adapters]
**Vulnerability:** Found a hardcoded fallback API key and internal cluster URL in `mlflow_adapter.py`.
**Learning:** Custom MLflow adapters (`PythonModel` subclasses) might contain copy-pasted debugging configurations or hardcoded environments that were accidentally committed.
**Prevention:** Ensure all `PythonModel` implementations load configuration strictly from artifacts or environment variables, never from hardcoded dictionaries in `__init__`.

## 2026-02-04 - Information Exposure in Kafka Service
**Vulnerability:** The Kafka consumer service (`kafka_app.py`) was logging raw input messages which could contain PII, and returning raw exception messages to the output topic which could leak internal implementation details.
**Learning:** In event-driven architectures like Kafka, error handling often involves producing to an error topic or the same output topic. Great care must be taken to sanitize these error messages. Also, logging "raw" messages for debugging is a common privacy trap.
**Prevention:** Always catch exceptions at the top level of the message processor, log the full stack trace securely (server-side), but return/produce only generic error codes or messages to the downstream systems. Sanitize input logs to exclude data fields.

## 2026-02-19 - [Secret Capture via Pickle in MLflow Adapters]
**Vulnerability:** `CustomSaver.Adapter` was initializing `self.model_config` using `os.getenv("LITELLM_API_KEY")` in `__init__`. When the model was saved (pickled), the *value* of the environment variable at save time was captured and stored in the model artifact, leaking the secret.
**Learning:** Initializing object state with environment variables in `__init__` is dangerous for picklable objects (like MLflow models), as it captures the build-time environment.
**Prevention:** Never read secrets into `self` attributes in `__init__`. Use `load_context` to read from artifacts, or read environment variables lazily at runtime (e.g., inside `predict`).
