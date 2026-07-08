# TACTICAL-DESIGN: Dark Gravity Components

This document details the **Tactical Design** of the autonomous factory, mapping the domain logic to specific software components, crates, and data schemas.

---

## Workspace Components

| Crate | Layer | Responsibility |
| :--- | :--- | :--- |
| `factory-core` | **Domain** | Pure logic: `Mission`, `Task`, `MissionStatus`, `TaskStatus`, `SecurityValidator` trait, `SecurityBounds` trait, `FactoryError` |
| `factory-application` | **Application** | Hatchet Workflows (6-phase DAG), Agent Logic (Rustant, ZeroClaw) |
| `factory-infrastructure` | **Infrastructure** | Clients: Kafka, R2R GraphRAG, S3, Jira, OpenZiti, MCP, Vault |
| `factory-mcp-server` | **Interface** | Axum-based MCP Server with SSE/HTTP transport, 8 tools |
| `factory-cli` | **Interface** | Hatchet worker CLI entry point |

---

## CRG-Verified MCP Tool Inventory

Based on `code-review-graph` analysis of `factory-mcp-server/src/tools/`:

| Tool | File | Lines | Functions | Status |
| :--- | :--- | :--- | :--- | :--- |
| `plan_mission` | `tools/plan_mission.rs` | 19-85 | `new`, `name`, `description`, `input_schema`, `call` | Implemented |
| `execute_code` | `tools/execute_code.rs` | 12-64 | `new`, `name`, `description`, `input_schema`, `call` | Implemented |
| `run_tests` | `tools/run_tests.rs` | 12-63 | `new`, `name`, `description`, `input_schema`, `call` | Implemented |
| `retrieve_context` | `tools/retrieve_context.rs` | 12-56 | `new`, `name`, `description`, `input_schema`, `call` | Implemented |
| `index_code` | `tools/index_code.rs` | 11-80 | `new`, `name`, `description`, `input_schema`, `call` | Implemented |
| `security_review` | `tools/security_review.rs` | — | — | Implemented |
| `search_jira` | `tools/search_jira.rs` | 12-56 | `new`, `name`, `description`, `input_schema`, `call` | Implemented |
| `update_mission_status` | `tools/update_mission_status.rs` | — | — | Implemented |

Each tool implements the `Tool` trait with `name()`, `description()`, `input_schema()`, and `call()` methods.

### C4 Component Diagram: MCP Server

```mermaid
C4Component
    title Dark Gravity Autonomous Factory - MCP Server Components

    Container_Boundary(mcp_server, "MCP Server (factory-mcp-server)") {
        Component(protocol, "Protocol Handler", "Axum SSE/HTTP", "Handles JSON-RPC 2.0 requests")
        
        Container_Boundary(tools_boundary, "MCP Tools") {
            Component(plan_mission, "Plan Mission Tool", "Rust", "Uses R2R context for planning")
            Component(execute_code, "Execute Code Tool", "Rust", "Runs code in sandbox")
            Component(run_tests, "Run Tests Tool", "Rust", "Validates tests in sandbox")
            Component(retrieve_context, "Retrieve Context Tool", "Rust", "Fetches graphRAG context")
            Component(index_code, "Index Code Tool", "Rust", "Indexes code into R2R")
            Component(security_review, "Security Review Tool", "Rust", "Audits code")
            Component(search_jira, "Search Jira Tool", "Rust", "Queries Jira API")
            Component(update_status, "Update Mission Status Tool", "Rust", "Updates Hatchet/DB")
        }
        
        Component(sandbox_driver, "Sandbox Driver", "Trait", "Interface for Firecracker/Subprocess")
    }

    Container(app, "Application / Agents", "Rust", "Initiates tool calls")
    Container_Ext(r2r, "R2R GraphRAG", "Knowledge Base")
    Container_Ext(k8s, "Kubernetes / gVisor", "Sandbox Engine")

    Rel(app, protocol, "JSON-RPC Call", "HTTP/SSE")
    
    Rel(protocol, plan_mission, "Routes Request")
    Rel(protocol, execute_code, "Routes Request")
    Rel(protocol, run_tests, "Routes Request")
    Rel(protocol, retrieve_context, "Routes Request")
    Rel(protocol, index_code, "Routes Request")
    Rel(protocol, security_review, "Routes Request")
    Rel(protocol, search_jira, "Routes Request")
    Rel(protocol, update_status, "Routes Request")
    
    Rel(plan_mission, r2r, "Search")
    Rel(retrieve_context, r2r, "Fetch")
    Rel(index_code, r2r, "Upsert")
    
    Rel(execute_code, sandbox_driver, "Invoke")
    Rel(run_tests, sandbox_driver, "Invoke")
    Rel(sandbox_driver, k8s, "Execute")
```

---

## Communication Patterns

### Inbound (Missions)
- **Adapter**: Confluent Kafka (`mission-input` topic).
- **Trigger**: Hatchet Engine observes Kafka stream.

### Internal (Agent Coordination)
- **Protocol**: MCP tools via `McpClient` (HTTP or SSE transport).
- **Mesh**: OpenZiti dark network overlay (mTLS) — zero public ports.
- **Memory**: R2R GraphRAG for semantic codebase context.
- **Telemetry**: Agent thoughts published to Kafka (`agent-thought` topic).

### Outbound (Delivery)
- **Adapter**: GitHub App (planned) via `create_pull_request`.
- **Protocol**: REST API.

---

## 6-Phase Hatchet DAG

The mission lifecycle is orchestrated by Hatchet Engine as a durable DAG:

```mermaid
sequenceDiagram
    participant Hatchet as "Hatchet Engine"
    participant RU as "Rustant (Planner Agent)"
    participant ZC as "ZeroClaw (Executor Agent)"
    participant MCP as "MCP Server"
    participant R2R as "R2R GraphRAG"
    participant Sandbox as "gVisor Sandbox"
    participant GH as "GitHub API"

    Hatchet->>RU: Phase 1: Trigger Planning
    activate RU
    RU->>MCP: Call retrieve_context
    MCP->>R2R: Query semantic context
    R2R-->>MCP: Context results
    MCP-->>RU: Return context
    RU->>MCP: Call plan_mission (LLM)
    MCP-->>RU: Plan generated
    RU-->>Hatchet: Commit Plan Checkpoint
    deactivate RU

    Hatchet->>ZC: Phase 2: Trigger Implementation
    activate ZC
    ZC->>MCP: Call execute_code
    MCP->>Sandbox: Spin up gVisor container & run code
    Sandbox-->>MCP: Code result (stdout/stderr)
    MCP-->>ZC: Result
    ZC-->>Hatchet: Commit Implementation Checkpoint
    deactivate ZC

    Hatchet->>ZC: Phase 3: Trigger Validation
    activate ZC
    ZC->>MCP: Call run_tests (cargo test)
    MCP->>Sandbox: Run in isolated VM
    Sandbox-->>MCP: Test results
    MCP-->>ZC: Pass/Fail
    ZC-->>Hatchet: Commit Validation Checkpoint
    deactivate ZC

    Hatchet->>RU: Phase 4: Trigger Security Review
    activate RU
    RU->>MCP: Call security_review (LLM-as-a-Judge)
    MCP-->>RU: Audit Result (Pass/Fail)
    RU-->>Hatchet: Commit Review Checkpoint
    deactivate RU

    Hatchet->>GH: Phase 5: Delivery
    GH-->>Hatchet: PR Created URL
```

---

## Data Model (factory-core)

```
Mission { id, name, description, created_at, tasks, status }
Task { id, mission_id, description, assigned_agent, dependencies, status }
MissionStatus: Pending | Running | Completed | Failed
TaskStatus: Queued | Active | Finished | Blocked
Metadata { timestamp, model_version, extra }
Inputs { jira_key, goal, target, constraints }
Outputs { pr_url, summary, artifact_paths }
Targets { agent_config, sandbox_config, expected_quality }
SHAPValues { values (HashMap) }
FeatureImportances { features (Vec) }
SecurityValidator: validate_signature() / audit_content() -> AuditResult
SecurityBounds: validate_token() / issue_jit_token() -> JitToken
VerifiableCredential: sign() / verify() (W3C JSON-LD with Ed25519)
```

---

## Sandbox Architecture

```mermaid
classDiagram
    class SandboxDriver {
        <<trait>>
        +execute(language, code) ExecutionResult
    }
    class SubprocessDriver {
        +execute(language, code) ExecutionResult
    }
    class GvisorK8sDriver {
        +execute(language, code) ExecutionResult
    }
    class NativeSurgerySandboxDriver {
        +execute_surgery(language, code) ExecutionResult
    }
    class ExecutionResult {
        +stdout: String
        +stderr: String
        +exit_code: i32
    }
    SandboxDriver <|-- SubprocessDriver
    SandboxDriver <|-- GvisorK8sDriver
    SandboxDriver <|-- NativeSurgerySandboxDriver
```

Both drivers are implemented in `crates/factory-mcp-server/src/sandbox.rs`.

---

## MCP Protocol

Communication follows the JSON-RPC 2.0 standard over SSE/HTTP, defined in `crates/factory-mcp-server/src/protocol.rs`:

- **`JsonRpcRequest`**: `{ id, method, params }`
- **`JsonRpcResponse`**: `{ id, result }`
- **`JsonRpcError`**: `{ code, message, data? }`
- **`McpTool`**: `{ name, description, input_schema }`
- **`CallToolResult`**: `{ content, is_error? }`
- **`McpContent`**: `{ type (text), text }`

---

*Last updated: 2026-07-08 — Verified against actual codebase via CRG analysis*