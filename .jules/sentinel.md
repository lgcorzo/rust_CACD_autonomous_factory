## 2026-02-03 - [Secrets Pickled in MLflow Adapters]

**Vulnerability:** Found `os.getenv` call inside `PythonModel.__init__` in `mlflow_adapter.py`. This resolves the secret value at save time and pickles it into the model artifact, effectively hardcoding the secret in the binary.
**Learning:** `PythonModel` subclasses are pickled by MLflow. Configuration dependent on environment variables must be resolved at load time (e.g., in `load_context` or `predict`), never in `__init__`.
**Prevention:** Remove environment variable resolution from `__init__`. Use `load_context` to read configuration from artifacts or resolve environment variables at runtime.

## 2026-02-03 - [Hardcoded Secrets in MLflow Adapters]

**Vulnerability:** Found a hardcoded fallback API key and internal cluster URL in `mlflow_adapter.py`.
**Learning:** Custom MLflow adapters (`PythonModel` subclasses) might contain copy-pasted debugging configurations or hardcoded environments that were accidentally committed.
**Prevention:** Ensure all `PythonModel` implementations load configuration strictly from artifacts or environment variables, never from hardcoded dictionaries in `__init__`.

## 2026-02-04 - Information Exposure in Kafka Service

**Vulnerability:** The Kafka consumer service (`kafka_app.py`) was logging raw input messages which could contain PII, and returning raw exception messages to the output topic which could leak internal implementation details.
**Learning:** In event-driven architectures like Kafka, error handling often involves producing to an error topic or the same output topic. Great care must be taken to sanitize these error messages. Also, logging "raw" messages for debugging is a common privacy trap.
**Prevention:** Always catch exceptions at the top level of the message processor, log the full stack trace securely (server-side), but return/produce only generic error codes or messages to the downstream systems. Sanitize input logs to exclude data fields.

## 2026-02-05 - [Path Traversal in MCP Tools]

**Vulnerability:** Found unvalidated file path operations in `run_tests` and `execute_code` MCP tools, allowing agents to overwrite arbitrary files outside the sandbox using `../` sequences.
**Learning:** Tools designed for AI agents often manipulate files based on model output. If the model hallucinates or is maliciously prompted to use relative paths like `../../etc/passwd`, standard `os.path.join` does not prevent directory traversal.
**Prevention:** Always use a secure path joining utility (like `safe_join` implementing `os.path.commonpath`) when handling file paths from untrusted sources (including LLMs) to enforce sandbox boundaries.

## 2026-02-09 - [Secret Leakage in Pickled MLflow Adapters]

**Vulnerability:** Found a hardcoded `model_config` dictionary in `mlflow_adapter.py` that captured the `LITELLM_API_KEY` environment variable value at initialization time. This caused the secret API key to be stored in plain text within the pickled model artifact.
**Learning:** When using MLflow's `PythonModel`, any instance attribute set in `__init__` is serialized (pickled) with the model. Reading secrets into instance attributes during `__init__` permanently bakes them into the artifact, leaking them to anyone with access to the model file.
**Prevention:** Never store environment-dependent configuration or secrets in `__init__` of a `PythonModel`. Always load configuration dynamically in `load_context` or `predict`, or use the context object provided by MLflow at runtime.

## 2026-02-14 - [Insecure HTTPS Warning Suppression]

**Vulnerability:** Found `urllib3.disable_warnings(urllib3.exceptions.InsecureRequestWarning)` in `kafka_app.py`. This globally suppressed warnings for invalid SSL certificates, potentially masking MITM attacks.
**Learning:** Suppressing warnings at the module level affects the entire application lifecycle and can hide critical security misconfigurations in production environments.
**Prevention:** Never suppress security warnings globally. If necessary for development, scope suppressions narrowly or use environment-specific configurations.

## 2026-02-17 - [Sensitive Data in PythonModel Adapter]

**Vulnerability:** The `CustomSaver.Adapter` class in `mlflow_adapter.py` was capturing the `LITELLM_API_KEY` environment variable in its `__init__` method, causing the secret to be pickled into the MLflow model artifact.
**Learning:** Even unused code in `__init__` can be dangerous if it captures secrets into the object state, as pickling serializes the entire object state.
**Prevention:** Removed the `self.model_config` assignment. Always verify that `PythonModel` subclasses do not store secrets in `self`.

## 2026-03-18 - [Information Exposure via Exception Stack Traces]
**Vulnerability:** The MCP server's `handle_call_tool` function (`mcp_server.py`) was returning raw exception strings (`str(e)`) and full stack traces (`traceback.format_exc()`) to the client when a tool failed.
**Learning:** Returning full tracebacks in API or server responses exposes internal implementation details (file paths, dependency versions, internal logic) which can be leveraged by attackers.
**Prevention:** Always log the full stack trace securely on the server side using `logger.exception()`, and return a sanitized, generic error message (e.g., "An internal error occurred") to the client.
