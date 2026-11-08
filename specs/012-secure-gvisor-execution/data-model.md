# Data Model & State Transitions: Secure gVisor Execution & Evidence Governance

**Feature**: `012-secure-gvisor-execution`
**Date**: 2026-11-08

## 1. Entities

### SandboxExecutionRequest
Represents an incoming code execution, test execution, or compilation task initiated by an agent or workflow step.
- `task_id`: String (UUID or slug)
- `code_type`: Enum (`Rust`, `Python`, `ShellCommand`)
- `payload`: String (code snippet or command string)
- `timeout_seconds`: u64 (default 60s)
- `memory_limit_mb`: u32 (minimum 512, default 512)
- `memory_request_mb`: u32 (default 64)
- `environment_profile`: Enum (`GvisorK8s`, `SecureContainerK8s`) - *Subprocess strictly prohibited for untrusted code*

### SandboxExecutionResult
Result emitted after execution inside the isolated container.
- `task_id`: String
- `status`: Enum (`Success`, `Failure`, `Timeout`, `SecurityPolicyViolation`, `InfrastructureError`)
- `exit_code`: Option<i32>
- `stdout`: String
- `stderr`: String
- `duration_ms`: u64
- `attestation`: SandboxAttestation

### SandboxAttestation
Cryptographic and environmental proof of isolation.
- `runtime_class`: String (e.g. `"gvisor"`)
- `runtime_type`: String (e.g. `"io.containerd.runsc.v1"`)
- `pod_name`: Option<String>
- `container_sha`: String (immutable digest)
- `seccomp_profile`: String (`RuntimeDefault`)
- `is_root`: bool (MUST be `false`)
- `capabilities_dropped`: Vec<String> (`["ALL"]`)

### FinOpsSharedStopState
Shared persistent state governing budget admissions.
- `version`: u64 (monotonic increment)
- `current_spend_usd`: f64
- `budget_limit_usd`: f64
- `hard_stop_active`: bool
- `hard_stop_reason`: Option<String>
- `last_updated_at`: String (ISO 8601)
- `locking_lease_id`: Option<String>

### VerificationEvidenceRecord
Data structure for evidence certification.
- `evidence_id`: String (UUID)
- `commit_sha`: String (40-char hex Git commit SHA)
- `task_id`: String
- `milestone`: String (e.g., `"DG-RVP-001"`, `"DG-SEC-002"`)
- `state`: Enum (`OPEN`, `IMPLEMENTED`, `TESTED`, `VERIFIED`)
- `attestation_type`: Enum (`GvisorVerified`, `ContainerVerified`, `MockInterfaceOnly`)
- `implementer_id`: String
- `verifier_id`: String (MUST be different from implementer_id)
- `created_at`: String (ISO 8601)
- `expires_at`: String (ISO 8601)
- `artifact_hashes`: HashMap<String, String> (path -> SHA256)

---

## 2. State Machine Transitions

### Evidence Lifecycle
```
 [ OPEN ] 
     │
     ▼ (Developer / Agent delivers implementation code)
 [ IMPLEMENTED ]
     │
     ▼ (Automated tests run in gVisor sandbox pod)
 [ TESTED ]
     │
     ▼ (Evidence Gate confirms commit SHA freshness + non-mock attestation + verifier separation)
 [ VERIFIED ]
```

Invalid Transitions Rejected by Gate:
- `OPEN` -> `VERIFIED` (Illegal transition)
- `IMPLEMENTED` -> `VERIFIED` (Requires `TESTED` state with test results)
- Any state -> `VERIFIED` if `attestation_type == MockInterfaceOnly`
- Any state -> `VERIFIED` if `commit_sha != HEAD_SHA`
