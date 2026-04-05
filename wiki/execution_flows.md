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
    participant OC as OpenCode Worker
    participant MCP as Rust MCP Server
    participant R2R as R2R Graph RAG
    participant LLM as LiteLLM (Gemini 2.5)

    n8n->>Jira: Poll: JQL "Active" tasks
    Jira-->>n8n: Return Task (DA-123)
    n8n->>Kafka: Publish "mission-input"
    n8n->>Jira: Transition to "In Progress"

    Kafka->>Hatchet: Trigger autonomous-mission
    Hatchet->>OC: Step 1: Plan Mission
    OC->>MCP: invoke("retrieve_context", goal)
    MCP->>R2R: Semantic Search
    R2R-->>MCP: Contextual Code Patterns
    OC->>MCP: invoke("plan_mission", goal + context)
    MCP->>LLM: Decompose into Task DAG
    LLM-->>MCP: Task DAG JSON
    MCP-->>OC: Task List

    rect rgb(240, 240, 240)
    Note over Hatchet,OC: Parallel Task Execution
    Hatchet->>OC: Step 2: Implement Task A (e.g. JWT utils)
    OC->>MCP: invoke("execute_code", task_A)
    MCP-->>OC: Code Changes & Results
    Hatchet->>OC: Step 3: Implement Task B (e.g. middleware)
    OC->>MCP: invoke("execute_code", task_B)
    MCP-->>OC: Code Changes & Results
    end

    Hatchet->>OC: Step 4: Run Tests
    OC->>MCP: invoke("run_tests", changes)
    MCP-->>OC: Success (10 tests passed)

    Hatchet->>OC: Step 5: Security Review
    OC->>MCP: invoke("security_review", code)
    MCP-->>OC: Approved (No vulnerabilities)

    Hatchet->>GIT: Create Pull Request (feature/da-123)
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
    participant Agent as OpenCode Agent
    participant MCP as Rust MCP Server (Axum)
    participant Sandbox as Sandbox/Firecrate
    
    Agent->>MCP: GET /sse (Establish Connection)
    MCP-->>Agent: 200 OK (Connection: keep-alive)
    MCP-->>Agent: event: endpoint { session_id: "uuid-123" }
    
    Note over Agent,MCP: Persistent Session Established
    
    Agent->>MCP: POST /mcp?session_id=uuid-123 (JSON-RPC: call_tool)
    MCP->>Sandbox: Execute Isolated Command
    Sandbox-->>MCP: Stdout/Stderr
    MCP-->>Agent: event: message { jsonrpc: "2.0", result: { content: [...] } }
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

## 🏗️ Verification Loop

No code reaches the `main` branch without surviving the **Verification Triad**:

1. **Logical Verification**:
    - **Coder Agent**: Implements the logic.
    - **Tester Agent**: Verifies that the logic works.
2. **Architectural Verification**:
    - **Reviewer Agent**: Checks for design patterns and system alignment.
3. **Security Verification**:
    - **SecurityReviewTool**: Automated scanning of code for SQL injections, hardcoded secrets, and unsafe dependencies.

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
