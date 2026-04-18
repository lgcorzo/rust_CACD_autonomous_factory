# 🔄 Execution Flows: Dark Gravity CA/CD

## 🗺️ Mission End-to-End Sequence Diagram

This diagram visualizes a full cycle, from the initial GitHub Issue to the final PR submission, orchestrated via the **6-Phase Hatchet DAG**.

```mermaid
sequenceDiagram
    autonumber
    participant GH as GitHub Issue
    participant n8n as n8n Poller
    participant Kafka as Kafka (mission-events)
    participant Hatchet as Hatchet Engine
    participant RU as Rustant (Planner)
    participant ZC as ZeroClaw (Executor)
    participant LLM as LiteLLM (minimax-m2.7:cloud)
    participant R2R as R2R (Context)

    GH->>n8n: New Issue (label:mission)
    n8n->>Kafka: Publish "mission-input"
    n8n->>GH: Add Label "in-progress"

    Kafka->>Hatchet: Trigger 6-Phase DAG
    
    rect rgb(230, 242, 255)
    Note over Hatchet,RU: Phase 1: Planning (Decomposition)
    Hatchet->>RU: Action: plan_mission
    RU->>R2R: Vector Search (Patterns)
    RU->>LLM: Generate Strategy JSON
    end

    rect rgb(230, 255, 230)
    Note over Hatchet,ZC: Phase 2-4: Implementation & Test
    Hatchet->>ZC: Action: execute_code
    ZC->>ZC: Unit Test Loop
    Hatchet->>ZC: Action: validate_integration
    end

    rect rgb(255, 245, 230)
    Note over Hatchet,RU: Phase 5-6: Review & Delivery
    Hatchet->>RU: Action: security_review
    Hatchet->>GH: Create Pull Request
    n8n->>GH: Close Issue / Add PR Link
    end
```

---

## 🛡️ Project Aethelgard: Self-Healing Loop

Project Aethelgard implements an autonomous remediation loop for Kubernetes infrastructure errors, triggered by Cloud-native alerts (FluxCD).

```mermaid
sequenceDiagram
    autonumber
    participant Flux as FluxCD / K8s
    participant RW as Remediator Webhook
    participant ML as MLflow (Track)
    participant Jules as Jules (Remediator)
    participant Cluster as K8s Cluster

    Flux->>RW: Alert: ImagePullBackOff / OOMKill
    RW->>ML: Start Experiment Run
    RW->>Jules: Trigger Remediation(alert_context)
    Jules->>Jules: Classify Error
    Jules->>Jules: Generate Fix (Patch/PR)
    Jules->>Cluster: Apply Patch / Create PR
    Cluster-->>Jules: Success/Failure
    Jules->>ML: Log Metric (remediation_success)
```

---

## 👻 Direct PR Correction Sequence (GravityRunner)

This flow illustrates how `GravityRunner` bypasses the full sandbox lifecycle for trivial fixes or real-time PR adjustments.

```mermaid
sequenceDiagram
    autonumber
    participant ZC as ZeroClaw (Executor)
    participant GR as GravityRunner (Runner)
    participant GH as GitHub API / Actions
    
    ZC->>ZC: Identifies trivial lint/format error
    ZC->>GR: Trigger: fix_pr_commit(branch, patch)
    GR->>GH: POST /repos/:owner/:repo/contents/:path
    GH-->>GR: 201 Created (Commit Added)
    
    Note over GR,GH: Trigger Validation Workflow
    GR->>GH: POST /repos/:owner/:repo/actions/workflows/:id/dispatches
    GH->>GR: Execute Workflow on Self-Hosted Runner
    GR-->>GH: Report Status (Check Run)
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
| **SecurityVerification**: Automated scanning for vulnerabilities and compliance.

---

## 📈 MLOps 2026: The "Experiment" Trace

The factory treats every mission lifecycle as an **MLflow Run**.

```mermaid
graph LR
    S[Start Mission] --> Run[Start MLflow Run]
    Run --> P[Phase 1: Planning]
    P --> C[Phase 2: Coding]
    C --> V[Phase 3: Validation]
    V --> R[Phase 4: Review]
    R --> D[Phase 5: Delivery]
    D --> Log[Log Metrics & Artifacts]
    Log --> End[End Experiment]
    
    P -.->|Log Token Usage| ML((MLflow))
    C -.->|Log Accuracy| ML
    V -.->|Log Latency| ML
    D -.->|Log PR URL| ML
```
