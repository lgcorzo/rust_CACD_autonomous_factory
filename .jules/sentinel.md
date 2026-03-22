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

## 2024-05-15 - [Path Traversal in MCP Tools]
**Vulnerability:** The MCP tools `execute_code` and `run_tests` process a `files_changed` payload containing `action` (e.g. `create`, `delete`) and `path`. In both tools, the `action == "delete"` block did not adequately prevent path traversal because either the path validation was skipped (via an early `continue`) or exceptions were incorrectly handled/omitted for the given operation. This allowed path traversal (e.g., `../../../tmp/pwned.txt`) to bypass the `safe_join` check effectively.
**Learning:** File path validation using `safe_join` must occur *before* evaluating any conditional logic based on the action being performed (create, read, update, delete). Furthermore, missing `try...except` blocks around `safe_join` calls inside loops can cause uncaught errors or unintended flow.
**Prevention:** Always validate and normalize paths from untrusted input before using them in any OS-level function or conditional logic blocks, and catch exceptions appropriately per-item.

## 2026-03-12 - [Information Exposure via Tracebacks in MCP Server]
**Vulnerability:** The `handle_call_tool` function in the MCP server (`mcp_server.py`) was catching all exceptions and returning the raw error message along with the full stack trace (`traceback.format_exc()`) to the client. This exposes internal application details and potentially sensitive execution context.
**Learning:** Returning raw stack traces and unhandled exception details directly to API clients or external systems violates the principle of "Fail securely". This information can be leveraged by attackers to map internal application structure, discover library versions, or uncover configuration details.
**Prevention:** Always log full exception details (including tracebacks) server-side using secure logging frameworks (e.g., `loguru`). Return generic error messages to external clients to prevent information leakage, ensuring the application fails securely without exposing its internals.

## 2026-03-22 - [Information Exposure via Exception Details in MCP Tools]
**Vulnerability:** The `execute_code` and `run_tests` MCP tools were capturing and returning `e.strerror` from `OSError` exceptions during file operations (e.g., creating/writing files). This exposed raw OS-level error messages to the client, which could leak information about the underlying filesystem structure, permissions, or system state.
**Learning:** Returning raw OS exception strings directly to clients violates the principle of failing securely. Attackers can use these details to map out the filesystem or understand the sandbox environment's constraints.
**Prevention:** Always return generic failure messages (e.g., "Operation failed") to clients when handling low-level exceptions like `OSError` or `IOError`. Log the detailed exception server-side for debugging purposes.
