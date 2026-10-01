---
iso_doc_type: "Description"
iso_viewpoint: "ContextView"
type: "architecture"
title: "Strategic Design — Dark Gravity CA/CD Autonomous Factory"
description: "ISO 42010 ContextView / ISO 15289 Description documentation for Strategic Design — Dark Gravity CA/CD Autonomous Factory."
tags: ['iso42010', 'okf', 'context_view', 'business']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# Strategic Design — Dark Gravity CA/CD Autonomous Factory

> **Purpose**: Present the top-down architectural view using C4 model levels 1–2, Onion Architecture layering, and Bounded Context mapping.

---

## 1. C4 Level 1: System Context Diagram

The System Context diagram shows the Dark Gravity Factory as a single system interacting with human actors and external systems.

```mermaid
C4Context
    title Dark Gravity Factory — System Context (C1)

    Person(po, "Product Owner", "Creates Epics tagged autonomous-plan")
    Person(techlead, "Tech Lead", "Approves Spec-Kit task decomposition")
    Person(architect, "Architect", "Resolves Agent-Stuck deadlocks via HITL Vertex 3")
    Person(reviewer, "Senior Reviewer", "Reviews design and merges PR/MR - HITL Vertex 4")
    Person(devops, "DevOps Engineer", "Monitors cluster health, troubleshoots deployments")

    System(factory, "Dark Gravity Factory", "CA/CD Autonomous Agent Platform: Ingests issues, plans via SDD, codes in sandboxes, validates with TDD+SAST, delivers PR/MR")

    System_Ext(github, "GitHub", "Source repos, PRs, Actions CI, Webhooks")
    System_Ext(gitlab, "GitLab", "MRs, CI Pipelines, Webhooks")
    System_Ext(jira, "Jira Cloud", "Issue tracking, Sprint boards, JQL search")
    System_Ext(fluxcd, "FluxCD", "GitOps Kubernetes reconciliation")
    System_Ext(hatchet, "Hatchet", "DAG workflow orchestration engine")
    System_Ext(kafka, "Kafka KRaft", "Event streaming bus, mission topics")
    System_Ext(litellm, "LiteLLM Proxy", "Multi-provider LLM routing: Azure OpenAI, Ollama")
    System_Ext(ziti, "OpenZiti", "Zero Trust overlay network")
    System_Ext(vault, "HashiCorp Vault", "Secret management, credential rotation")
    System_Ext(r2r, "R2R GraphRAG", "Context-aware code knowledge retrieval")
    System_Ext(sentry, "Sentry", "Error tracking and crash reporting")

    Rel(po, factory, "Creates autonomous missions via labeled issues")
    Rel(techlead, factory, "Approves decomposed task plans")
    Rel(architect, factory, "Overrides stuck agents, tunes parameters")
    Rel(reviewer, factory, "Reviews and merges PRs - human gate")
    Rel(devops, factory, "Configures K8s, monitors health")

    Rel(factory, github, "Polls issues/PRs, pushes code, reads CI status")
    Rel(factory, gitlab, "Polls MRs, pushes fixes, reads pipelines")
    Rel(factory, jira, "Searches issues, syncs SDD tasks")
    Rel(factory, hatchet, "Dispatches 6-phase mission DAGs")
    Rel(factory, kafka, "Publishes/consumes mission and telemetry events")
    Rel(factory, litellm, "Routes LLM inference with FinOps tagging")
    Rel(factory, ziti, "Encrypted agent-to-service tunnels")
    Rel(factory, vault, "Retrieves secrets, rotates credentials")
    Rel(factory, r2r, "Retrieves contextual code knowledge")
    Rel(factory, sentry, "Reports errors and crash telemetry")
    Rel(fluxcd, factory, "Deploys factory manifests to K8s cluster")
```

---

## 2. C4 Level 2: Container Diagram

The Container diagram decomposes the factory into its 5 Rust crates (deployable containers) and their Kubernetes namespace topology.

```mermaid
C4Container
    title Dark Gravity Factory — Container Diagram (C2)

    Person(human, "Human Operator", "HITL governance vertices 1-4")

    System_Boundary(factory, "Dark Gravity Factory") {
        Container(core, "factory-core", "Rust Crate", "Domain layer: Mission, Task, Error, Security, NHI, PRDirective, Pipeline entities")
        Container(app, "factory-application", "Rust Crate", "Application layer: 6 Agents, Workflows, Poller, Bridge, Telemetry")
        Container(infra, "factory-infrastructure", "Rust Crate", "Infrastructure layer: GitHub/GitLab/Jira clients, Kafka, Ziti, Vault, R2R, Sentry adapters")
        Container(mcp, "factory-mcp-server", "Rust Crate + Axum", "Interface layer: MCP JSON-RPC server, 15+ tools, sandbox orchestration, feedback routes")
        Container(cli, "factory-cli", "Rust Binary", "CLI layer: trigger_mission, run_functional_suite, trigger_deep_search")
    }

    System_Ext(github, "GitHub")
    System_Ext(gitlab, "GitLab")
    System_Ext(hatchet, "Hatchet")
    System_Ext(kafka, "Kafka KRaft")
    System_Ext(litellm, "LiteLLM")
    System_Ext(ziti, "OpenZiti")

    Rel(human, cli, "Triggers missions via CLI")
    Rel(human, mcp, "Interacts via MCP protocol")
    Rel(cli, app, "Invokes application workflows")
    Rel(mcp, app, "Exposes agent capabilities as MCP tools")
    Rel(app, core, "Uses domain entities and traits")
    Rel(app, infra, "Calls infrastructure adapters")
    Rel(infra, github, "REST API: issues, PRs, Actions")
    Rel(infra, gitlab, "REST API: MRs, pipelines")
    Rel(infra, kafka, "Produces/consumes mission events")
    Rel(infra, ziti, "Zero Trust tunnels")
    Rel(app, hatchet, "Dispatches DAG workflows")
    Rel(app, litellm, "LLM inference with FinOps tags")
```

### Kubernetes Namespace Topology

```mermaid
graph TB
    subgraph "K8s Cluster"
        subgraph "ns: agents"
            MCP["factory-mcp-server Pod"]
            CLI["factory-cli Jobs"]
            Sandbox["gVisor Sandbox Pods"]
        end
        subgraph "ns: orchestrators"
            Hatchet["Hatchet Engine"]
            Poller["Poller Daemon"]
        end
        subgraph "ns: llm-apps"
            LiteLLM["LiteLLM Proxy"]
            R2R["R2R GraphRAG"]
        end
        subgraph "ns: confluent"
            Kafka["Kafka KRaft Broker"]
        end
        subgraph "ns: networking"
            Ziti["OpenZiti Controller + Router"]
        end
        subgraph "ns: secrets"
            Vault["HashiCorp Vault"]
            SealedSecrets["Bitnami Sealed Secrets"]
        end
    end

    MCP --> Hatchet
    MCP --> Sandbox
    Poller --> Kafka
    Hatchet --> Kafka
    MCP --> LiteLLM
    MCP --> R2R
    MCP --> Ziti
    MCP --> Vault
```

---

## 3. Onion Architecture Layer Model

The factory follows a strict **Onion Architecture** where dependencies point inward. No inner layer references an outer layer.

```mermaid
graph TB
    subgraph "Layer 4: Interface"
        CLI_L["factory-cli"]
        MCP_L["factory-mcp-server"]
    end
    subgraph "Layer 3: Infrastructure"
        INFRA_L["factory-infrastructure"]
    end
    subgraph "Layer 2: Application"
        APP_L["factory-application"]
    end
    subgraph "Layer 1: Domain - Core"
        CORE_L["factory-core"]
    end

    CLI_L --> APP_L
    MCP_L --> APP_L
    APP_L --> CORE_L
    APP_L --> INFRA_L
    INFRA_L --> CORE_L

    style CORE_L fill:#4CAF50,stroke:#2E7D32,color:#fff
    style APP_L fill:#2196F3,stroke:#1565C0,color:#fff
    style INFRA_L fill:#FF9800,stroke:#E65100,color:#fff
    style CLI_L fill:#9C27B0,stroke:#6A1B9A,color:#fff
    style MCP_L fill:#9C27B0,stroke:#6A1B9A,color:#fff
```

| Layer | Crate | Responsibility | Dependencies |
|:---|:---|:---|:---|
| **Domain (Core)** | `factory-core` | Entities (`Mission`, `Task`, `PRDirective`), Error types, Security traits, NHI credentials | `serde`, `chrono`, `uuid`, `thiserror`, `ed25519-dalek`, `zeroize` |
| **Application** | `factory-application` | Agents (`Rustant`, `ZeroClaw`, etc.), Workflows, Poller, Bridge, Telemetry | `factory-core`, `factory-infrastructure`, `async-trait`, `regex` |
| **Infrastructure** | `factory-infrastructure` | Platform adapters: GitHub, GitLab, Jira, Kafka, Ziti, Vault, R2R, Sentry, MCP Client | `factory-core`, `reqwest`, `rdkafka`, `kube`, `k8s-openapi` |
| **Interface (MCP)** | `factory-mcp-server` | MCP JSON-RPC server, tool registration, sandbox pod management, feedback routes | `factory-core`, `factory-application`, `factory-infrastructure`, `axum`, `tower` |
| **Interface (CLI)** | `factory-cli` | Binary entry points: `trigger_mission`, `run_functional_suite`, `trigger_deep_search` | `factory-core`, `factory-application`, `clap`, `tokio` |

---

## 4. Bounded Context Map

The factory decomposes into 4 strategic bounded contexts with explicit integration patterns:

```mermaid
graph LR
    subgraph "BC1: Agent Execution"
        Rustant["RustantAgent"]
        ZeroClaw["ZeroClawAgent"]
        Auditor["AuditorAgent"]
        FinOps["FinOpsAgent"]
        QAObs["QAObserverAgent"]
        DocAgent["DocumentationAgent"]
    end

    subgraph "BC2: Mission Orchestration"
        Poller["PollerDaemonService"]
        DAG["Hatchet 6-Phase DAG"]
        CB["Aethelgard Circuit Breaker"]
        SDD["Spec-Kit SDD Pipeline"]
    end

    subgraph "BC3: Infrastructure Integration"
        GH["GitHub Adapter"]
        GL["GitLab Adapter"]
        JR["Jira Adapter"]
        KF["Kafka Producer/Consumer"]
        MCPClient["MCP Client"]
    end

    subgraph "BC4: Security and Identity"
        NHI["Ed25519 NHI Issuer"]
        SAST["SAST Scanner"]
        Sandbox["gVisor Sandbox"]
        Validator["SecurityValidator"]
        JIT["JitToken Manager"]
    end

    BC1_note["Anti-Corruption Layer"] --> BC3
    Rustant --> DAG
    ZeroClaw --> Sandbox
    ZeroClaw --> SAST
    Poller --> GH
    Poller --> GL
    Poller --> JR
    DAG --> KF
    NHI --> Validator

    style BC1_note fill:#fff,stroke:#999
```

### Context Integration Patterns

| Source BC | Target BC | Pattern | Mechanism |
|:---|:---|:---|:---|
| Agent Execution → Infrastructure | Anti-Corruption Layer | Agents call infrastructure adapters via trait abstractions (`McpClient`, `R2rClient`, `AethalgardClient`) |
| Mission Orchestration → Agent Execution | Published Events | Hatchet DAG dispatches task events consumed by agents |
| Mission Orchestration → Infrastructure | Shared Kernel | `PolledIssueEvent`, `PRCommentEvent` domain entities shared between poller and adapters |
| Security/Identity → All | Conformist | All contexts conform to `SecurityValidator` trait and `SandboxConstraint` policies |

---

> *Related: [Business Context](business_context.md) · [Tactical Design](tactical_design.md) · [Agent Specifications](agent_specifications.md)*