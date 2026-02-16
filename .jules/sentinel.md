<<<<<<< HEAD
## 2026-02-03 - [Hardcoded Secrets in MLflow Adapters]
**Vulnerability:** Found a hardcoded fallback API key and internal cluster URL in `mlflow_adapter.py`.
**Learning:** Custom MLflow adapters (`PythonModel` subclasses) might contain copy-pasted debugging configurations or hardcoded environments that were accidentally committed.
**Prevention:** Ensure all `PythonModel` implementations load configuration strictly from artifacts or environment variables, never from hardcoded dictionaries in `__init__`.
=======
## 2026-02-04 - Information Exposure in Kafka Service
**Vulnerability:** The Kafka consumer service (`kafka_app.py`) was logging raw input messages which could contain PII, and returning raw exception messages to the output topic which could leak internal implementation details.
**Learning:** In event-driven architectures like Kafka, error handling often involves producing to an error topic or the same output topic. Great care must be taken to sanitize these error messages. Also, logging "raw" messages for debugging is a common privacy trap.
**Prevention:** Always catch exceptions at the top level of the message processor, log the full stack trace securely (server-side), but return/produce only generic error codes or messages to the downstream systems. Sanitize input logs to exclude data fields.
>>>>>>> 36194fc (feat: Sanitize Kafka service logs and error responses)

## 2026-02-16 - [Secret Leak via Pickle Serialization]
**Vulnerability:** `CustomSaver.Adapter` was capturing `os.getenv("LITELLM_API_KEY")` in `__init__`. Since MLflow uses pickle to serialize the model object, the evaluated secret value was being persisted in the model artifact.
**Learning:** Initializing configuration in `__init__` for `PythonModel` subclasses is dangerous because it captures the *current* environment state into the artifact.
**Prevention:** Defer all environment-dependent configuration to `load_context` or `predict`, which run at serving time.
