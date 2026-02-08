## 2026-02-03 - [Hardcoded Secrets in MLflow Adapters]
**Vulnerability:** Found a hardcoded fallback API key and internal cluster URL in `mlflow_adapter.py`.
**Learning:** Custom MLflow adapters (`PythonModel` subclasses) might contain copy-pasted debugging configurations or hardcoded environments that were accidentally committed.
**Prevention:** Ensure all `PythonModel` implementations load configuration strictly from artifacts or environment variables, never from hardcoded dictionaries in `__init__`.

## 2026-02-04 - Information Exposure in Kafka Service
**Vulnerability:** The Kafka consumer service (`kafka_app.py`) was logging raw input messages which could contain PII, and returning raw exception messages to the output topic which could leak internal implementation details.
**Learning:** In event-driven architectures like Kafka, error handling often involves producing to an error topic or the same output topic. Great care must be taken to sanitize these error messages. Also, logging "raw" messages for debugging is a common privacy trap.
**Prevention:** Always catch exceptions at the top level of the message processor, log the full stack trace securely (server-side), but return/produce only generic error codes or messages to the downstream systems. Sanitize input logs to exclude data fields.

## 2026-02-05 - [Secret Leakage via Pickle in MLflow Adapters]
**Vulnerability:** `CustomSaver.Adapter` was initializing `self.model_config` with `os.getenv("LITELLM_API_KEY")` in `__init__`. This caused the *value* of the API key to be pickled into the model artifact, leaking the secret.
**Learning:** Even using `os.getenv` in `__init__` is dangerous for objects that will be pickled (like MLflow models), as it freezes the environment state at creation time into the artifact.
**Prevention:** Do not capture environment variables in `__init__` of `PythonModel`. Access them dynamically in `load_context` or `predict` methods, or ensure they are not stored in instance attributes.
