# OpenCode Integration in CA/CD Factory

This document explains how the OpenCode agent is integrated and executed within the project's Autonomous Agent Factory.

## Architecture & Integration (UML)

The `OpenCode` agent acts as a native Hatchet Worker. It connects to our consolidated Python MCP server over SSE to execute domain-specific tools. KEDA monitors event queues and scales the OpenCode pods accordingly.

```mermaid
classDiagram
    direction TB
    class HatchetEngine {
        +TriggerWorkflow()
        +MaintainState()
        +RetryTasks()
    }
    class OpenCodeWorker {
        +Image: ghcr.io/anomalyco/opencode
        +Model: Gemini 2.5 Pro
        +ConnectHatchet()
        +InvokeMCP()
    }
    class MCPServer {
        +plan_mission()
        +execute_code()
        +run_tests()
        +security_review()
        +retrieve_context()
        +index_code()
    }
    class KEDA {
        +MonitorKafka()
        +ScalePods()
    }
    class Kafka {
        +mission-input
        +agent-thought
    }

    Kafka <-- KEDA : Monitors lag
    KEDA --> OpenCodeWorker : Scales replicas (0 to N)
    HatchetEngine "1" *-- "many" OpenCodeWorker : Orchestrates & Assigns Tasks
    OpenCodeWorker --> MCPServer : SSE Transport Tool Calls
```

## Execution Flow

When a mission is triggered, Hatchet establishes the durable state. In the `plan` step, an OpenCode worker generates a Directed Acyclic Graph (DAG) using the Planner Agent's capabilities via MCP.

**Critical Control Flow:** Hatchet then natively parses this DAG (via `fan_out_tasks`) to spawn dedicated parallel child workflows for each coding task. OpenCode workers process these individual child tasks, invoking the testing and reviewing tools before Hatchet aggregates the final outcome.

```mermaid
sequenceDiagram
    autonumber
    participant K as Kafka (mission-input)
    participant H as Hatchet Engine (Orchestrator)
    participant O as OpenCode Worker Node(s)
    participant M as MCP Server (llmops-python-package)
    participant E as Firecracker MicroVM / E2B

    K->>H: Trigger `cacd-mission` workflow
    H->>O: Assign 'plan' step

    Note over O,M: OpenCode connects to MCP via SSE
    O->>M: Tool Call: `retrieve_context(goal)`
    M-->>O: R2R pgvector codebase context

    O->>M: Tool Call: `plan_mission(goal, context)`
    M-->>O: Task DAG Returned
    O-->>H: Return DAG Outcome

    Note over H,O: Hatchet uses dynamic fan-out to orchestrate the DAG.
    H->>H: Parse DAG & Spawn Child Workflows

    par For each task in DAG
        H->>O: Assign 'execute task' step
        O->>M: Tool Call: `execute_code()`
        M->>E: Send generated code to Secure Sandbox
        E-->>M: Evaluation & Runtime results
        M-->>O: Code completion status

        O->>M: Tool Call: `run_tests()`
        M->>E: pytest isolated run
        E-->>M: Test reports
        M-->>O: Test Pass/Fail
        O-->>H: Child task complete
    end

    H->>O: Assign 'aggregate_and_review' step
    O->>M: Tool Call: `security_review()`
    M-->>O: Security Assessment Pass
    O-->>H: Review complete

    H->>K: Publish to `mission-artifact` (Done)
```

## Local Development Usage

To run the full stack locally, connecting the OpenCode worker to the local Python MCP Server:

1. **Start the MCP Server:**
   This provisions the `llmops-factory` via stdio or SSE.

   ```bash
   poetry run invoke projects.mcp
   ```

2. **Configure OpenCode:**
   Ensure your local OpenCode instance is configured to point to the local MCP server via your `opencode.json` configuration file, utilizing the `http://localhost:8000/sse` endpoint for tool access.
3. **Trigger Mission:**
   Publish an event to the Hatchet backend or the Kafka `mission-input` topic to instantiate an OpenCode worker response.
