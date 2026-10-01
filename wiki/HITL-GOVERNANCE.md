# HITL Governance — Dark Gravity CA/CD Autonomous Factory

> **Purpose**: Formal specification of the 4-Vertex Human-in-the-Loop governance mesh ensuring human oversight at critical decision points in the autonomous pipeline.

---

## 1. Governance Model Overview

The Dark Gravity Factory implements a **4-Vertex HITL Mesh** — four strategically placed human decision gates that prevent fully autonomous operation while maximizing agent acceleration.

> **Core Principle**: Agents are empowered to create branches, commit code, push upstream, create PRs/MRs, and resolve CI/CD failures. **Under no circumstances may an agent automatically merge a PR or MR into `main`** or default branches. The merge process **MUST be a manual human action**.

```mermaid
graph LR
    V1["Vertex 1:<br/>Strategic Injection<br/>(Product Owner)"]
    V2["Vertex 2:<br/>Sprint Mobilization<br/>(Tech Lead)"]
    V3["Vertex 3:<br/>Exception Override<br/>(Architect)"]
    V4["Vertex 4:<br/>Categorical Scrutiny<br/>(Senior Reviewer)"]

    V1 -->|"Epic approved"| AGENT_PLAN["Agent Planning"]
    AGENT_PLAN -->|"Tasks decomposed"| V2
    V2 -->|"Tasks approved"| AGENT_CODE["Agent Coding"]
    AGENT_CODE -->|"Deadlock detected"| V3
    V3 -->|"Override applied"| AGENT_CODE
    AGENT_CODE -->|"PR/MR created"| V4
    V4 -->|"Merge approved"| DEPLOY["Deployment"]

    style V1 fill:#4CAF50,stroke:#2E7D32,color:#fff
    style V2 fill:#2196F3,stroke:#1565C0,color:#fff
    style V3 fill:#FF9800,stroke:#E65100,color:#fff
    style V4 fill:#9C27B0,stroke:#6A1B9A,color:#fff
```

---

## 2. Vertex Specifications

### Vertex 1: Strategic Injection

| Attribute | Value |
|:---|:---|
| **Human Role** | Product Owner |
| **Trigger Condition** | Creation of Epic tagged `autonomous-plan` / `autonomous-mission` / `dark-gravity` |
| **Enforcement Mechanism** | GitHub/GitLab label requirements; Poller only ingests issues with required labels |
| **Decision** | Whether a mission should be autonomously executed |
| **Output** | `PolledIssueEvent` dispatched to ingestion pipeline |

### Vertex 2: Sprint Mobilization

| Attribute | Value |
|:---|:---|
| **Human Role** | Tech Lead |
| **Trigger Condition** | RustantAgent generates `SddMissionPlan` with decomposed tasks |
| **Enforcement Mechanism** | Hatchet DAG `Plan` phase waits for approval signal; `/speckit-tasks` generates tasks requiring human review |
| **Decision** | Whether the task decomposition is correct and safe to execute |
| **Output** | Approved `SddTaskItem` list dispatched to `Code` phase |

### Vertex 3: Exception Override

| Attribute | Value |
|:---|:---|
| **Human Role** | Architect |
| **Trigger Condition** | Aethelgard circuit breaker trips: 3 consecutive failures with stagnant diff hash |
| **Enforcement Mechanism** | `Agent-Stuck` state triggers Slack/Jira alert; DAG pauses until human override |
| **Decision** | Override parameters, abort mission, or reassign to different agent strategy |
| **Output** | Resumed execution with tuned parameters OR mission abortion |

### Vertex 4: Categorical Scrutiny

| Attribute | Value |
|:---|:---|
| **Human Role** | Senior Reviewer |
| **Trigger Condition** | PR/MR created by agent after successful validation + review phases |
| **Enforcement Mechanism** | Branch protection rules require human approval; agents cannot merge |
| **Decision** | Final design review, security posture validation, merge approval |
| **Output** | Merged PR/MR → FluxCD deployment |

---

## 3. Governance State Machine

```mermaid
stateDiagram-v2
    [*] --> EpicCreated: PO creates tagged Epic (Vertex 1)
    EpicCreated --> Ingested: Poller detects required labels
    Ingested --> Planning: RustantAgent plans mission
    Planning --> TasksReady: SddMissionPlan generated
    TasksReady --> HumanApproval_V2: Vertex 2 - Tech Lead Review
    
    HumanApproval_V2 --> Coding: Tasks approved
    HumanApproval_V2 --> Rejected: Tasks rejected
    
    Coding --> Validation: ZeroClaw executes TDD
    Validation --> Review: Tests pass + SAST pass
    Validation --> AgentStuck: 3 retries exhausted
    
    AgentStuck --> HumanOverride_V3: Vertex 3 - Architect
    HumanOverride_V3 --> Coding: Override with new params
    HumanOverride_V3 --> Aborted: Mission rejected
    
    Review --> PRCreated: RustantAgent creates PR/MR
    PRCreated --> HumanMerge_V4: Vertex 4 - Senior Reviewer
    HumanMerge_V4 --> Delivered: PR merged (human action)
    HumanMerge_V4 --> Coding: Changes requested
    
    Delivered --> [*]
    Rejected --> [*]
    Aborted --> [*]
```

---

## 4. Documentation Sync Pipeline

The wiki documentation is synchronized via the `.github/workflows/docs-to-wiki.yml` pipeline:

```mermaid
sequenceDiagram
    participant Dev as Developer/Agent
    participant Git as Git Repository
    participant CI as GitHub Actions
    participant Wiki as GitHub Wiki

    Dev->>Git: Push to wiki/ directory
    Git->>CI: Trigger docs-to-wiki.yml
    CI->>CI: Validate markdown links
    CI->>CI: Check Mermaid syntax
    CI->>CI: Calculate OSR (< 5% gate)
    
    alt OSR < 5%
        CI->>Wiki: Sync wiki/ → GitHub Wiki
        CI-->>Dev: ✅ Wiki updated
    else OSR >= 5%
        CI-->>Dev: ❌ OSR gate failed
    end
```

---

> *Related: [Security Architecture](SECURITY-ARCHITECTURE.md) · [Business Context](BUSINESS-CONTEXT.md) · [Compliance & Audit](COMPLIANCE-AUDIT.md)*
