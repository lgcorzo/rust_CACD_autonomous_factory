---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-application::agents::zeroclaw — ZeroClawAgent (Developer)"
source_path: "crates/factory-application/src/agents/zeroclaw.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-application::agents::zeroclaw — ZeroClawAgent (Developer)."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-application::agents::zeroclaw — ZeroClawAgent (Developer)

> **Source**: `crates/factory-application/src/agents/zeroclaw.rs`
> **Layer**: Application

---

## TDD Loop State Machine

```mermaid
stateDiagram-v2
    [*] --> LoadCheckpoint
    LoadCheckpoint --> SASTPreScan
    SASTPreScan --> SandboxLaunch: score >= 8.0
    SASTPreScan --> Rejected: score < 8.0
    SandboxLaunch --> ExecuteCode
    ExecuteCode --> RunTests
    RunTests --> TestsPassed: All green
    RunTests --> TestsFailed: Red
    TestsFailed --> ExecuteCode: Retry < 3
    TestsFailed --> CircuitBreakerTrip: Retry == 3
    TestsPassed --> ValidateSandbox
    ValidateSandbox --> SaveCheckpoint
    SaveCheckpoint --> [*]
    CircuitBreakerTrip --> Aethalgard
    Aethalgard --> HumanOverride: HITL Vertex 3
    Rejected --> [*]
```

## Sandbox Execution Sequence

```mermaid
sequenceDiagram
    participant ZC as ZeroClawAgent
    participant MCP as MCP Server
    participant K8s as Kubernetes
    participant Pod as gVisor Pod

    ZC->>MCP: launch_sandbox_pod(constraints)
    MCP->>K8s: Create Job with gVisor runtime
    K8s->>Pod: Start containers (app + sidecar)
    Pod-->>MCP: Pod ready

    ZC->>MCP: execute_code(code_patch)
    MCP->>Pod: Write code to scratch workspace
    Pod-->>MCP: Compilation result

    ZC->>MCP: run_tests(test_command)
    MCP->>Pod: Execute test suite
    Pod-->>MCP: Test results (pass/fail)

    ZC->>ZC: validate_sandbox_constraints(app, sidecar)
    Note right of ZC: App: 30 MiB / 0.25 CPU<br/>Sidecar: 20 MiB / 0.10 CPU<br/>No network egress
```

## Tool Authorization

| Tool | Authorized |
|:---|:---:|
| `launch_sandbox_pod` | ✅ |
| `execute_code` | ✅ |
| `run_tests` | ✅ |
| `security_review` | ✅ |
| `sync_bridge_state` | ✅ |
| `introspect_k8s` | ✅ |
| `plan_mission` | ❌ |
| `retrieve_context` | ❌ |

---

> *Related: [RustantAgent](rustant.md) · [AuditorAgent](auditor.md)*
