# AGENT-SPECIFICATIONS: Autonomous Workforce

This document specifies the roles, responsibilities, and tooling for the **Dark Gravity** autonomous agents.

---

## Rustant (Planner Agent)

The "Captain" of the mission. Governs the **Intelligence Context**.

### Responsibilities
- **Context-Driven Planning**: Queries R2R GraphRAG via `R2rClient` for semantic context retrieval, then calls `plan_mission` via MCP to plan the mission.
- **Security Review**: Audits code via `security_review` MCP tool using LLM-as-a-Judge.
- **Review**: Reviews execution results and provides feedback for iteration.

### Implementation
- **File**: `crates/factory-application/src/agents/rustant.rs` (lines 12-78)
- **Key Methods**: `new()`, `plan_mission()`, `review_mission()`
- **Dependencies**: `McpClient`, `R2rClient`
- **Agent trait**: Implements `Agent` (`name`, `execute`)

#### Rustant Mission Planning Sequence
```mermaid
sequenceDiagram
    participant Hatchet as Workflow Engine
    participant RU as RustantAgent
    participant R2R as R2rClient
    participant LLM as LiteLLM Gateway
    participant MCP as McpServer

    Hatchet->>RU: execute(task_description)
    activate RU
    RU->>R2R: search("Context for task...")
    R2R-->>RU: GraphRAG nodes & edges
    RU->>MCP: call_tool("plan_mission", payload)
    MCP->>LLM: generate_text()
    LLM-->>MCP: execution plan JSON
    MCP-->>RU: Result
    RU-->>Hatchet: MissionPlan
    deactivate RU
```

---

## ZeroClaw (Executor Agent)

The "Muscle" of the system. Operates within the **Execution Context**.

### Responsibilities
- **Code Implementation**: Translates tasks into source code via `execute_code` MCP tool.
- **Verification**: Runs test suites via `run_tests` inside isolated sandbox.
- **Self-Correction**: Iterates on code based on test failure feedback.

### Implementation
- **File**: `crates/factory-application/src/agents/zeroclaw.rs` (lines 11-98)
- **Key Methods**: `new()`, `execute_task()`, `validate_mission()`, `introspect_k8s()`
- **Dependencies**: `McpClient`
- **Sandbox Drivers**: `SubprocessDriver` (local), `FirecrackerDriver` (micro-VM via KVM)
- **Agent trait**: Implements `Agent` (`name`, `execute`)

#### ZeroClaw Execution Sequence
```mermaid
sequenceDiagram
    participant Hatchet as Workflow Engine
    participant ZC as ZeroClawAgent
    participant MCP as McpServer
    participant Sandbox as SandboxDriver
    
    Hatchet->>ZC: execute(task_description)
    activate ZC
    loop Iterative Execution
        ZC->>MCP: call_tool("execute_code", source)
        MCP->>Sandbox: execute(language, source)
        Sandbox-->>MCP: stdout, stderr
        MCP-->>ZC: Result
        
        ZC->>MCP: call_tool("run_tests", config)
        MCP->>Sandbox: execute("cargo test")
        Sandbox-->>MCP: Test pass/fail
        MCP-->>ZC: Result
    end
    ZC-->>Hatchet: Execution Result
    deactivate ZC
```

---

## DevOps Agent (Aethelgard Loop)

> **Status: Planned (Not yet implemented)**

The "Immune System". Governs the **Remediation Context**.

### Responsibilities
- **CI/CD Auto-Remediation**: Parses pipeline failures, queries R2R GraphRAG for historical fixes, directs Developer Agent to apply patches.
- **Circuit Breaker**: Maximum 3 consecutive auto-remediation attempts before escalating.
- **Backlog Automation**: Auto-grades severity and creates backlog issues.

---

## Documentation Agent (Superpowers)

> **Status: Active (Partially implemented)**

The "Memory Keeper". Manages the **Infrastructure Context** for documentation.

### Responsibilities
- **CRG Wiki Generation**: Uses `code-review-graph wiki` to generate auto-documented wiki pages from AST analysis.
- **Graphify Integration**: Maintains `graphify-out/` with code structure graphs and reports.
- **Wiki Refinement**: Uses Superpowers skills to keep documentation accurate.

### Superpowers Skills Loaded
| Skill | Purpose |
| :--- | :--- |
| `writing-plans` | Decompose documentation into atomic tasks |
| `subagent-driven-development` | Execute each subagent with focused context |
| `verification-before-completion` | Verify before marking tasks complete |

---

## Agent Interface

All agents implement the `Agent` trait in `crates/factory-application/src/agents/`:

```mermaid
classDiagram
    class Agent {
        <<trait>>
        +name() String
        +execute(task_description: String) Value
    }
    class DocumentationAgent {
        <<active>>
        +run_post_merge_pipeline()
        +extract_code_deltas()
        +generate_hazitek_report()
    }
    class RustantAgent {
        <<active>>
        +plan_mission()
        +review_mission()
    }
    class ZeroClawAgent {
        <<active>>
        +execute_task()
        +validate_mission()
    }
    class AuditorAgent {
        <<planned>>
        +audit_mission_logs()
    }
    class QAObserverAgent {
        <<planned>>
        +poll_sentry()
    }
    class FinOpsAgent {
        <<planned>>
        +track_anomaly()
    }
    class DevOpsAgent {
        <<planned>>
        +remediate_ci_failure()
    }
    
    Agent <|-- DocumentationAgent
    Agent <|-- RustantAgent
    Agent <|-- ZeroClawAgent
    Agent <|-- AuditorAgent
    Agent <|-- QAObserverAgent
    Agent <|-- FinOpsAgent
    Agent <|-- DevOpsAgent
```

```rust
#[async_trait]
pub trait Agent: Send + Sync {
    fn name(&self) -> String;
    async fn execute(&self, task_description: &str) -> anyhow::Result<Value>;
}
```

---

## Agentic Interaction Flow

```mermaid
sequenceDiagram
    participant Hatchet as Hatchet DAG
    participant RU as Rustant
    participant ZC as ZeroClaw
    participant MCP as Factory MCP Server
    
    Hatchet->>RU: Trigger Mission Planning
    RU->>MCP: retrieve_context & plan_mission
    MCP-->>RU: Plan Context
    RU-->>Hatchet: Mission Plan
    
    Hatchet->>ZC: Trigger Execution
    ZC->>MCP: execute_code (Sandbox)
    MCP-->>ZC: Code Diff
    ZC->>MCP: run_tests
    MCP-->>ZC: Test Results
    ZC-->>Hatchet: Implementation artifacts
    
    Hatchet->>RU: Trigger Review
    RU->>MCP: security_review
    MCP-->>RU: Audit Result
    RU-->>Hatchet: Final Status
```

---

## LLM Configuration

All agents route through the **LiteLLM Gateway**:

| Model | Provider | Tool Calling |
| :--- | :--- | :--- |
| `gemma4:12b` | LiteLLM (OpenAI-compatible) | Yes |
| `ollama/qwen2.5-coder:7b` | LiteLLM (Ollama) | Yes |
| `ollama/qwen2.5:7b` | LiteLLM (Ollama) | No |

---

## CRG-Verified Agent Dependencies

Based on `code-review-graph` edge analysis:

- **Rustant** → Outgoing edges: `r2r_client.search()`, `mcp_client.call_tool_json()`, `tracing::info`
- **ZeroClaw** → Outgoing edges: `mcp_client.call_tool_json()`, `tracing::info`
- **Mission Workflow** → Orchestrates: Rustant (plan) → ZeroClaw (code) → Rustant (review) → Delivery
- **Task Workflow** → Individual task execution with `StepCheckpoint` recovery

---

*Last updated: 2026-06-23 — Verified against actual codebase via CRG analysis*