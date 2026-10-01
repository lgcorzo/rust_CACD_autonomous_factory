# factory-application::workflows::develop_task — TDD Task Development

> **Source**: `crates/factory-application/src/workflows/develop_task.rs`
> **Layer**: Application

---

## TDD Task Development Sequence

```mermaid
sequenceDiagram
    participant Hatchet as Hatchet DAG
    participant ZC as ZeroClawAgent
    participant MCP as MCP Server
    participant SBX as gVisor Sandbox

    Hatchet->>ZC: Execute SddTaskItem
    ZC->>MCP: Load checkpoint (bridge state)
    ZC->>MCP: launch_sandbox_pod(constraints)
    MCP->>SBX: Create gVisor pod

    loop TDD Red-Green-Refactor
        ZC->>MCP: execute_code(code_patch)
        MCP->>SBX: Apply patch
        ZC->>MCP: run_tests(test_command)
        MCP->>SBX: Execute tests
        SBX-->>MCP: Test results
        MCP-->>ZC: Pass/Fail

        alt Tests Pass
            ZC->>ZC: Refactor phase
        else Tests Fail (retry < 3)
            ZC->>ZC: Modify code patch
        else Tests Fail (retry == 3)
            ZC->>ZC: Escalate to Aethelgard
        end
    end

    ZC->>MCP: Save checkpoint
```

---

> *Related: [circuit_breaker.rs](crates_factory-application_src_workflows_circuit_breaker.md) · [ZeroClawAgent](crates_factory-application_src_agents_zeroclaw.md)*
