# Contract: Execution Security Policy & Sandbox Attestation

## 1. MCP Tool `launch_sandbox_pod` Input & Output Contract

### Request Parameters
```json
{
  "job_name": "sandbox-val-013-xxx",
  "cmd": "cargo test -- --nocapture",
  "language": "rust",
  "namespace": "factory-sandboxes",
  "trust_level": "untrusted",
  "environment": "production"
}
```

### Response Contract
```json
{
  "status": "success",
  "outcome": "passed",
  "command": "cargo test -- --nocapture",
  "exit_code": 0,
  "stdout": "running 1 test\ntest tests::it_works ... ok\ntest result: ok. 1 passed",
  "stderr": "",
  "duration_ms": 7520,
  "execution_environment": "gvisor",
  "attestation": {
    "cluster_id": "microk8s-server.internal.lgcorzo",
    "node_name": "server.internal.lgcorzo",
    "job_name": "sandbox-val-013-xxx",
    "pod_name": "sandbox-val-013-xxx-pod-yyy",
    "pod_uid": "78fd2bbe-c3c4-403c-82bc-37ed8afbbe09",
    "container_id": "containerd://b7a25261e029d68978bbf290df0a62fc819b3c51",
    "container_image_digest": "docker.io/library/rust@sha256:9f841bbe9e7d8e37ceb96ed907265a3a0df7f44e3737d0b100e7907a679acb36",
    "runtime_class_requested": "gvisor",
    "runtime_type_observed": "io.containerd.runsc.v1",
    "seccomp_profile": "RuntimeDefault",
    "capabilities_dropped": ["ALL"],
    "is_root": false,
    "exit_code": 0,
    "command_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    "observed_at": "2026-11-09T17:15:00Z"
  }
}
```

## 2. FinOps Transactional Admission Contract

### Trait Definition
```rust
pub trait FinOpsStopStore: Send + Sync {
    fn read_state(&self) -> Result<FinOpsStopState, String>;
    fn write_state(
        &self,
        halted: bool,
        reason: Option<String>,
        current_spend: f64,
        max_daily_budget: f64,
    ) -> Result<FinOpsStopState, String>;
    fn check_and_reserve_budget(
        &self,
        mission_id: &str,
        estimated_cost: f64,
    ) -> Result<BudgetReservationReceipt, String>;
}
```

