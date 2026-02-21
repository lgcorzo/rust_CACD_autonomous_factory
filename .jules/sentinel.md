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

## 2026-02-05 - [Path Traversal in MCP Tools]
**Vulnerability:** Found unvalidated file path operations in `run_tests` and `execute_code` MCP tools, allowing agents to overwrite arbitrary files outside the sandbox using `../` sequences.
**Learning:** Tools designed for AI agents often manipulate files based on model output. If the model hallucinates or is maliciously prompted to use relative paths like `../../etc/passwd`, standard `os.path.join` does not prevent directory traversal.
**Prevention:** Always use a secure path joining utility (like `safe_join` implementing `os.path.commonpath`) when handling file paths from untrusted sources (including LLMs) to enforce sandbox boundaries.
