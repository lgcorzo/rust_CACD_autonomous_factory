# API & Schema Contracts: Secure gVisor Execution & Evidence Governance

**Feature**: `012-secure-gvisor-execution`
**Date**: 2026-11-08

## 1. Sandbox Execution Tool Contract (`execute_code` & `run_tests`)

### Input Schema
```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "ExecuteCodeRequest",
  "type": "object",
  "properties": {
    "code": {
      "type": "string",
      "description": "Code snippet or test command to execute"
    },
    "language": {
      "type": "string",
      "enum": ["rust", "python", "shell"]
    },
    "timeout_seconds": {
      "type": "integer",
      "default": 60,
      "minimum": 5,
      "maximum": 300
    },
    "memory_limit_mb": {
      "type": "integer",
      "default": 512,
      "minimum": 256,
      "maximum": 2048
    }
  },
  "required": ["code", "language"]
}
```

### Output Schema
```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "ExecuteCodeResponse",
  "type": "object",
  "properties": {
    "status": {
      "type": "string",
      "enum": ["success", "failure", "timeout", "security_violation", "infrastructure_error"]
    },
    "outcome": {
      "type": "string",
      "enum": ["passed", "failed", "not_executed", "infrastructure_error", "timeout"]
    },
    "exit_code": {
      "type": ["integer", "null"]
    },
    "stdout": {
      "type": "string"
    },
    "stderr": {
      "type": "string"
    },
    "duration_ms": {
      "type": "integer"
    },
    "execution_environment": {
      "type": "string",
      "enum": ["gvisor-runsc", "k8s-hardened-container"]
    },
    "attestation": {
      "type": "object",
      "properties": {
        "runtime_class": { "type": "string" },
        "non_root_uid": { "type": "integer" },
        "read_only_rootfs": { "type": "boolean" },
        "capabilities_dropped": { "type": "array", "items": { "type": "string" } }
      },
      "required": ["runtime_class", "non_root_uid", "read_only_rootfs"]
    }
  },
  "required": ["status", "outcome", "execution_environment", "duration_ms"]
}
```

---

## 2. Evidence Validation CLI Contract (`factory-cli validate-evidence`)

### Command Invocation
```bash
factory-cli validate-evidence --evidence-file <PATH> --target-commit <SHA> --strict-attestation
```

### Exit Codes
- `0`: Evidence is fresh, matches target commit, has non-mock verified attestation, passes all gates.
- `1`: Stale evidence / commit SHA mismatch (`EVIDENCE_STALE_COMMIT`).
- `2`: Illegal state jump (e.g. `OPEN` -> `VERIFIED`).
- `3`: Mock/Synthetic attestation rejected under strict attestation mode.
- `4`: Implementer and Verifier identity collision.
