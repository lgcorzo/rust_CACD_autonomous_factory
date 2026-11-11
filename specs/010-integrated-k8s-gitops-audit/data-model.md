# Phase 1: Domain Entities & Architecture Model

**Feature**: `010-integrated-k8s-gitops-audit`  
**Date**: `2026-11-02`

---

## 1. Domain Entities & Type Definitions

### 1.1 Sandbox Execution Outcome (`crates/factory-mcp-server/src/tools/run_tests.rs`)
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TestExecutionOutcome {
    Passed,
    Failed,
    NotExecuted,
    InfrastructureError,
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestExecutionReport {
    pub outcome: TestExecutionOutcome,
    pub command: String,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
    pub execution_environment: String, // e.g., "k8s-gvisor"
    pub evidence_hash: Option<String>,
}
```

### 1.2 Sandbox Runtime Configuration (`crates/factory-mcp-server/src/sandbox.rs`)
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SandboxEnvironmentMode {
    Production,
    Development,
    Test,
}

#[derive(Debug, Clone)]
pub struct SandboxSecurityPolicy {
    pub mode: SandboxEnvironmentMode,
    pub require_gvisor: bool,
    pub namespace: String,
    pub execution_timeout_secs: u64,
    pub cpu_limit: String,
    pub memory_limit: String,
}

impl Default for SandboxSecurityPolicy {
    fn default() -> Self {
        Self {
            mode: SandboxEnvironmentMode::Production,
            require_gvisor: true,
            namespace: "factory-sandboxes".to_string(),
            execution_timeout_secs: 120,
            cpu_limit: "250m".to_string(),
            memory_limit: "64Mi".to_string(),
        }
    }
}
```

### 1.3 FinOps Admission Gate (`crates/factory-application/src/agents/finops.rs`)
```rust
pub trait MissionAdmissionGate: Send + Sync {
    fn can_admit_mission(&self, mission_id: &str) -> Result<(), factory_core::error::FactoryError>;
}

impl MissionAdmissionGate for FinOpsAgent {
    fn can_admit_mission(&self, mission_id: &str) -> Result<(), factory_core::error::FactoryError> {
        if self.is_halted() {
            return Err(factory_core::error::FactoryError::FinOps(format!(
                "Mission admission denied for '{}': FinOps budget hard-stop latch is active",
                mission_id
            )));
        }
        Ok(())
    }
}
```

---

## 2. Kubernetes GitOps Workload Specification (`gitops_internal_lgcorzo`)

This example uses `development`, the namespace name selected by the checked-in admission-policy binding. The repository defines no `Namespace` object, so existence is **TO CONFIRM**.

```yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: sandbox-job-example
  namespace: development
spec:
  ttlSecondsAfterFinished: 300
  backoffLimit: 0
  template:
    metadata:
      labels:
        app.kubernetes.io/component: sandbox-workload
        app.kubernetes.io/part-of: dark-gravity
    spec:
      runtimeClassName: gvisor
      restartPolicy: Never
      automountServiceAccountToken: false
      securityContext:
        runAsNonRoot: true
        runAsUser: 65534
        runAsGroup: 65534
        seccompProfile:
          type: RuntimeDefault
      containers:
      - name: sandbox
        image: python:3.11.11-slim
        command: ["/bin/sh", "-c"]
        args: ["python3 -c 'print(\"secure sandbox\")'"]
        securityContext:
          allowPrivilegeEscalation: false
          readOnlyRootFilesystem: true
          capabilities:
            drop:
            - ALL
        resources:
          limits:
            memory: "30Mi"
            cpu: "250m"
            ephemeral-storage: "100Mi"
          requests:
            memory: "16Mi"
            cpu: "50m"
            ephemeral-storage: "50Mi"
```
