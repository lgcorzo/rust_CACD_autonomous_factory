# 🔄 Execution Flows: Dark Gravity CA/CD

## 🗺️ Mission End-to-End Sequence Diagram

This diagram visualizes a full cycle, from the initial Jira ticket to the final PR submission.

```mermaid
sequenceDiagram
    autonumber
    participant Jira as Jira Cloud
    participant n8n as n8n Poller
    participant Kafka as Kafka
    participant Hatchet as Hatchet Engine
    participant RU as Rustant (Architect)
    participant ZC as ZeroClaw (Executor)
    participant MCP as Rust MCP Server
    participant R2R as R2R Graph RAG
    participant LLM as LiteLLM (mnimax2.5)

    n8n->>Jira: Poll: JQL "Active" tasks
    Jira-->>n8n: Return Task (DA-123)
    n8n->>Kafka: Publish "mission-input"
    n8n->>Jira: Transition to "In Progress"

    Kafka->>Hatchet: Trigger autonomous-mission
    
    rect rgb(230, 242, 255)
    Note over Hatchet,RU: Phase 1: Planning
    Hatchet->>RU: Action: plan_mission
    RU->>Kafka: publish_thought("analyzing requirements")
    RU->>MCP: invoke("context_pruning", goal)
    MCP->>R2R: Vector Search
    R2R-->>MCP: Pruned Context
    RU->>LLM: Decompose into Strategy
    LLM-->>RU: Strategy JSON
    end

    rect rgb(230, 255, 230)
    Note over Hatchet,ZC: Phase 2 & 3: Execution & Validation
    Hatchet->>ZC: Action: code_task
    ZC->>Kafka: publish_thought("implementing module X")
    ZC->>MCP: invoke("execute_code", logic)
    MCP->>ZC: Isolated Results
    Hatchet->>ZC: Action: validate_task
    ZC->>MCP: invoke("run_tests", logic)
    end

    rect rgb(255, 245, 230)
    Note over Hatchet,RU: Phase 4: Review
    Hatchet->>RU: Action: review_mission
    RU->>Kafka: publish_thought("auditing security tokens")
    RU->>MCP: invoke("security_review", artifacts)
    end

    Hatchet->>GIT: Create PR (mission-da-123)
    Hatchet->>Kafka: Publish "mission-complete"
    Kafka->>n8n: Notify completion (DA-123)
    n8n->>Jira: Transition to "Done" + Post PR Link
```

---

## 🌩️ SSE Transport Flow (Unified Communication)

The factory uses a persistent **SSE (Server-Sent Events)** stream for bidirectional tool execution. This ensures long-running tasks (like code generation or test suites) do not timeout and provide real-time feedback.

```mermaid
sequenceDiagram
    autonumber
    participant Agent as Rustant/ZeroClaw
    participant MCP as Rust MCP Server (Axum)
    participant Sandbox as Sandbox/Firecracker
    
    Agent->>MCP: GET /sse (Establish Connection)
    MCP-->>Agent: 200 OK (Connection: keep-alive)
    MCP-->>Agent: event: endpoint { uri: "http://.../mcp" }
    
    Note over Agent,MCP: Persistent Session Established
    
    Agent->>MCP: POST /mcp (JSON-RPC: call_tool)
    MCP->>Sandbox: Execute Micro-VM isolated task
    Sandbox-->>MCP: Stdout/Stderr
    MCP-->>Agent: event: message { jsonrpc: "2.0", result: [...] }
```

---

## 🛠️ Tool Execution Internal Flow

The `ExecuteCodeTool` is the most active tool in the system. It handles code generation, refinement, and initial validation.

```mermaid
graph TD
    Start[Agent Invokes execute_code] --> Extract[Extract Task Details]
    Extract --> GenCode[Generate Refined Code]
    GenCode --> Write[Apply Changes to Workspace]
    Write --> Validate{Syntax Check}
    Validate -->|Fail| Refine[Refine Code with Error]
    Refine --> GenCode
    Validate -->|Pass| Success[Return Results to Agent]
```

---

## 🏗️ Verification Triad (Phase 12 Integration)

No code reaches the `main` branch without surviving the **Verification Triad**:

1. **Logical Verification (ZeroClaw)**:
    - **Executor**: Implements and validates logic in a Firecracker micro-VM.
2. **Architectural Verification (Rustant)**:
    - **Architect**: Checks for alignment with the R2R-retrieved patterns.
3. **Security Verification (Rustant Tooling)**:
    - **SecurityReview**: Automated scanning for vulnerabilities and compliance.

---

## 🌩️ KEDA Autoscaling

The factory scales horizontally based on mission demand (Kafka lag).

```mermaid
graph LR
    K[Kafka mission-input] -->|Queue Lag| KE[KEDA ScaledObject]
    KE -->|Scale| D[OpenCode Deployment]
    D -->|N replicas| H[Hatchet Workers]
```

> [!NOTE]
> This ensures zero idling resource overhead while maintaining high throughput for large bursts of missions.
