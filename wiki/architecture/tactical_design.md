---
iso_doc_type: "Description"
iso_viewpoint: "ArchitectureDescription"
type: "architecture"
title: "Tactical Design — Dark Gravity CA/CD Autonomous Factory"
description: "ISO 42010 ArchitectureDescription / ISO 15289 Description documentation for Tactical Design — Dark Gravity CA/CD Autonomous Factory."
tags: ['iso42010', 'okf', 'architecture_description', 'c4']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# Tactical Design — Dark Gravity CA/CD Autonomous Factory

> **Purpose**: C4 Level 3 (Component) diagrams showing internal components per crate, module dependencies, DAG phase mapping, and MCP tool registry.

---

## 1. C4 Level 3: Component Diagrams

### 1.1 factory-core Components

```mermaid
graph TB
    subgraph "factory-core (Domain Layer)"
        LIB["lib.rs: Domain Entities"]
        ERR["error.rs: FactoryError"]
        EXEC["executor.rs: CodeSurgeryExecutor trait"]
        SEC["security.rs: SecurityValidator, SandboxConstraint, SastScanResult, JitToken"]
        NHI["security/nhi.rs: Ed25519 NHI Credential Manager"]
        CFG["config.rs: AgentModelConfig"]
        PROTO["proto/v1: gRPC Protobuf definitions"]
    end

    LIB --> ERR
    LIB --> EXEC
    LIB --> SEC
    SEC --> NHI
    LIB --> CFG
    LIB --> PROTO

    style LIB fill:#4CAF50,stroke:#2E7D32,color:#fff
```

**Key domain entities in `lib.rs`**:

| Entity | Description |
|:---|:---|
| `Mission` | Complete unit of autonomous work with `id`, `tasks`, `status` |
| `Task` | Individual work unit with `assigned_agent`, `dependencies`, `status` |
| `PRDirective` | Parsed bot command: `Spec`, `Refine`, `Retry`, `Status`, `Validate`, `Interact` |
| `PolledIssueEvent` | Issue detected by outbound poller |
| `PRCommentEvent` | PR comment with extracted directive |
| `PipelineFailureEvent` | CI/CD failure for remediation |
| `ErrorCategory` | Classification: `CodeCompilation`, `LintViolation`, `TestFailure`, etc. |
| `FinOpsTag` | Cost attribution virtual headers |
| `SddMissionPlan` | Structured plan from RustantAgent for Hatchet |
| `SddTaskItem` | Individual SDD task parsed from `tasks.md` |

---

### 1.2 factory-application Components

```mermaid
graph TB
    subgraph "factory-application (Application Layer)"
        APPLIB["lib.rs: Agent trait"]
        
        subgraph "agents/"
            AMOD["mod.rs: Agent Registry"]
            RUST["rustant.rs: RustantAgent - Planner"]
            ZC["zeroclaw.rs: ZeroClawAgent - Developer"]
            AUD["auditor.rs: AuditorAgent - Security"]
            FIN["finops.rs: FinOpsAgent - Cost Control"]
            QA["qa_observer.rs: QAObserverAgent - Quality"]
            DOC["doc_agent.rs: DocumentationAgent - Wiki"]
        end
        
        subgraph "workflows/"
            WMOD["mod.rs: Workflow Registry"]
            AM["autonomous_mission.rs: 6-Phase DAG"]
            CB["circuit_breaker.rs: Aethelgard"]
            DT["develop_task.rs: TDD Execution"]
            PR["pipeline_remediation.rs: CI Fix"]
            CC["comment_control.rs: PR Directives"]
            DR["deep_research.rs: Knowledge Search"]
        end
        
        subgraph "bridge/"
            BMOD["mod.rs: Bridge Registry"]
            ADK["adk_driver.rs: Google ADK Integration"]
            KB["kafka_bridge.rs: Event Bridge"]
            SEM["semantica_bridge.rs: Semantic Search"]
            BST["state.rs: Checkpoint State"]
        end
        
        POLL["poller_service.rs: PollerDaemonService"]
        TEL["telemetry_export.rs: Metrics Export"]
        GLV["gitlab_verifier.rs: GitlabVerifier"]
        OSR["utils/osr.rs: OSR Calculator"]
    end

    APPLIB --> AMOD
    APPLIB --> WMOD
    APPLIB --> BMOD
    AMOD --> RUST
    AMOD --> ZC
    AMOD --> AUD
    AMOD --> FIN
    AMOD --> QA
    AMOD --> DOC

    style APPLIB fill:#2196F3,stroke:#1565C0,color:#fff
```

---

### 1.3 factory-infrastructure Components

```mermaid
graph TB
    subgraph "factory-infrastructure (Infrastructure Layer)"
        ILIB["lib.rs: Adapter Traits"]
        
        subgraph "Platform Adapters"
            GH["github.rs: GitHub REST API"]
            GL["gitlab.rs: GitLab REST API"]
            JR["jira.rs: Jira Cloud API"]
        end
        
        subgraph "Messaging"
            KF["kafka.rs: KRaft Producer/Consumer"]
        end
        
        subgraph "Security"
            ZT["ziti.rs: OpenZiti Client"]
            VT["vault.rs: Vault Secret Manager"]
            SV["security_validator.rs: Ed25519 Validator"]
        end
        
        subgraph "Observability"
            SN["sentry.rs: Error Tracking"]
            S3["s3.rs: Artifact Storage"]
        end
        
        subgraph "AI/ML"
            R2R["r2r.rs: GraphRAG Client"]
            MPC["mcp_client.rs: MCP Protocol Client"]
            SEM["semantica.rs: Semantic Search"]
        end
        
        subgraph "Execution"
            AE["aethalgard.rs: Circuit Breaker"]
            CS["cursor_store.rs: Polling State"]
            GP["git_poller.rs: Repository Change Detection"]
            PC["pipeline_classifier.rs: Error Taxonomy"]
        end
    end

    ILIB --> GH
    ILIB --> GL
    ILIB --> KF
    ILIB --> ZT
    ILIB --> MPC

    style ILIB fill:#FF9800,stroke:#E65100,color:#fff
```

---

### 1.4 factory-mcp-server Components

```mermaid
graph TB
    subgraph "factory-mcp-server (Interface Layer)"
        MLIB["lib.rs: MCP Server Core"]
        MAIN["main.rs: Axum HTTP Server"]
        PROT["protocol.rs: JSON-RPC Protocol"]
        SBX["sandbox.rs: gVisor Pod Lifecycle"]
        FB["feedback_route.rs: Webhook Handler"]
        GHW["github_webhook.rs: GitHub Event Handler"]
        SCR["scratch.rs: Temp Workspace"]
        
        subgraph "skills/"
            SMOD["mod.rs: Skill Registry"]
            SCTX["context.rs: Spec-Kit Context"]
        end
        
        subgraph "tools/"
            TMOD["mod.rs: Tool Dispatch"]
            BRIDGE["bridge.rs: State Bridge"]
            EXEC["execute_code.rs"]
            IDX["index_code.rs: R2R Ingestion"]
            LAUNCH["launch_sandbox_pod.rs: K8s Job"]
            PLAN["plan_mission.rs"]
            RET["retrieve_context.rs: GraphRAG Query"]
            RUN["run_tests.rs"]
            SJIRA["search_jira.rs"]
            SREV["security_review.rs: Semgrep SAST"]
            SKIT["spec_kit_tool.rs: SDD Bridge"]
            SK2I["spec_kit_tasks_to_issues.rs"]
            UMS["update_mission_status.rs"]
            DRT["deep_research_tool.rs"]
            GFS["get_factory_status.rs"]
            IKT["inspect_kafka_topic.rs"]
            LMB["list_minio_buckets.rs"]
            LMO["list_minio_objects.rs"]
        end
    end

    MAIN --> MLIB
    MLIB --> PROT
    MLIB --> TMOD
    MLIB --> SMOD
    TMOD --> LAUNCH
    TMOD --> EXEC
    TMOD --> RUN
    TMOD --> SREV
    TMOD --> SKIT
    LAUNCH --> SBX

    style MLIB fill:#9C27B0,stroke:#6A1B9A,color:#fff
```

---

### 1.5 factory-cli Components

```mermaid
graph TB
    subgraph "factory-cli (CLI Layer)"
        CMAIN["main.rs: CLI Entry Point"]
        TM["bin/trigger_mission.rs: Manual Mission Trigger"]
        RFS["bin/run_functional_suite.rs: E2E Test Runner"]
        TDS["bin/trigger_deep_search.rs: Knowledge Search"]
    end

    CMAIN --> TM
    CMAIN --> RFS
    CMAIN --> TDS

    style CMAIN fill:#9C27B0,stroke:#6A1B9A,color:#fff
```

---

## 2. DAG Phase-to-Agent Mapping

The 6-phase Hatchet DAG maps each phase to its responsible agent:

| Phase | Agent | Input | Output | HITL Gate |
|:---|:---|:---|:---|:---|
| **1. Ingestion** | PollerDaemonService | Issue/PR/Pipeline event | `PolledIssueEvent` or `PRCommentEvent` | Vertex 1 (PO creates Epic) |
| **2. Plan** | RustantAgent | Event + R2R context | `SddMissionPlan` with tasks | Vertex 2 (Tech Lead approves) |
| **3. Code** | ZeroClawAgent | `SddTaskItem` list | Code patches in sandbox | — |
| **4. Validation** | ZeroClawAgent + AuditorAgent | Code patches | Test results + SAST scan | Vertex 3 (Architect on deadlock) |
| **5. Review** | RustantAgent | Validation results | Review decision | — |
| **6. Delivery** | PollerDaemonService | Approved changes | PR/MR creation | Vertex 4 (Reviewer merges) |

---

## 3. MCP Tool Registry

The `factory-mcp-server` exposes 15+ MCP tools:

| Tool | Category | Description |
|:---|:---|:---|
| `launch_sandbox_pod` | Execution | Creates gVisor-sandboxed K8s pod |
| `execute_code` | Execution | Runs code in sandbox |
| `run_tests` | Validation | Executes test suites |
| `security_review` | Security | SAST scan via Semgrep |
| `spec_kit_tool` | Planning | Spec-Kit SDD bridge |
| `spec_kit_tasks_to_issues` | Planning | Converts tasks to GitHub issues |
| `plan_mission` | Planning | Generates mission plan |
| `retrieve_context` | Knowledge | R2R GraphRAG query |
| `index_code` | Knowledge | Ingests code into R2R |
| `search_jira` | Integration | JQL issue search |
| `bridge` | State | Checkpoint state management |
| `update_mission_status` | Telemetry | Mission status tracking |
| `deep_research_tool` | Knowledge | Deep web/code research |
| `get_factory_status` | Observability | Factory health check |
| `inspect_kafka_topic` | Observability | Kafka topic inspection |

---

> *Related: [Strategic Design](strategic_design.md) · [Agent Specifications](agent_specifications.md) · [Experiment Lifecycle](runtime_sequences.md)*