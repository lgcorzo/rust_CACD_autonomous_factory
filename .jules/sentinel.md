## 2026-02-03 - [Hardcoded Secrets in MLflow Adapters]
**Vulnerability:** Found a hardcoded fallback API key and internal cluster URL in `mlflow_adapter.py`.
**Learning:** Custom MLflow adapters (`PythonModel` subclasses) might contain copy-pasted debugging configurations or hardcoded environments that were accidentally committed.
**Prevention:** Ensure all `PythonModel` implementations load configuration strictly from artifacts or environment variables, never from hardcoded dictionaries in `__init__`.

## 2026-02-04 - Information Exposure in Kafka Service
**Vulnerability:** The Kafka consumer service (`kafka_app.py`) was logging raw input messages which could contain PII, and returning raw exception messages to the output topic which could leak internal implementation details.
**Learning:** In event-driven architectures like Kafka, error handling often involves producing to an error topic or the same output topic. Great care must be taken to sanitize these error messages. Also, logging "raw" messages for debugging is a common privacy trap.
**Prevention:** Always catch exceptions at the top level of the message processor, log the full stack trace securely (server-side), but return/produce only generic error codes or messages to the downstream systems. Sanitize input logs to exclude data fields.

## 2026-02-12 - [Path Traversal in MCP Tools]
**Vulnerability:** MCP tools `execute_code` and `run_tests` were vulnerable to path traversal attacks via `os.path.join`. An attacker (or hallucinating LLM) could specify paths like `../evil.py` to write files outside the sandbox.
**Learning:** `os.path.join` does not prevent path traversal. Always use `os.path.abspath` and verify the prefix using `os.path.commonpath` or similar when handling untrusted paths.
**Prevention:** Introduced `autogen_team.core.security.safe_join` to safely resolve paths within a base directory. Updated MCP tools to use this utility.
