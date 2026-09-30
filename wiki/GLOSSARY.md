# Glossary — Dark Gravity CA/CD Autonomous Factory

> **Purpose**: Ubiquitous language dictionary for the Dark Gravity project. Defines all domain terms, agent names, protocol acronyms, and architectural concepts used throughout this wiki.

---

## A

| Term | Definition |
|:---|:---|
| **ADK** | Agent Development Kit — Google's framework for building AI agents; used by `AdkDriver` to orchestrate multi-step agent reasoning |
| **Aethelgard** | Circuit breaker and anti-deadlock subsystem that monitors agent retry loops and trips after 3 consecutive failures with stagnant diff hashes |
| **Agent** | An autonomous software entity implementing the `Agent` trait (`name()` + `execute()`) that performs a specific role in the factory pipeline |
| **Agent-Stuck** | State entered when an agent's circuit breaker trips after exceeding the maximum retry count; requires HITL Vertex 3 (Architect Override) |
| **AST** | Abstract Syntax Tree — the parsed tree representation of Rust source code; used for OSR calculation and documentation symbol extraction |
| **AuditorAgent** | Factory agent responsible for security review, SAST scanning, and cryptographic credential verification |

## B

| Term | Definition |
|:---|:---|
| **Bounded Context** | DDD concept defining clear boundaries between subsystems; Dark Gravity has 4: Agent Execution, Mission Orchestration, Infrastructure Integration, Security/Identity |

## C

| Term | Definition |
|:---|:---|
| **CA/CD** | Continuous Agentic / Continuous Deployment — the autonomous software engineering paradigm where AI agents plan, code, test, and deploy with human governance gates |
| **C4 Model** | Architecture visualization framework with 4 levels: Context (C1), Container (C2), Component (C3), Code (C4) |
| **Circuit Breaker** | See **Aethelgard**; prevents infinite retry loops by tripping after 3 failed attempts with no code progress |
| **CodeSurgeryExecutor** | Trait in `factory-core` for applying surgical file patches (`SurgicalPatch`) and verifying syntax |
| **CompliancePacket** | Audit evidence record containing AST deltas, compute hours, FinOps spend, and Ed25519 NHI credentials |

## D

| Term | Definition |
|:---|:---|
| **DAG** | Directed Acyclic Graph — the 6-phase mission execution graph orchestrated by Hatchet: Ingestion → Plan → Code → Validation → Review → Delivery |
| **Dark Gravity** | The CA/CD Autonomous Agent Factory project name |
| **DDD** | Domain-Driven Design — architectural methodology organizing code around business domains with explicit bounded contexts |
| **DocAgent** | DocumentationAgent — factory agent that generates wiki content, calculates OSR, and triggers doc-sync workflows |

## E

| Term | Definition |
|:---|:---|
| **Ed25519** | Elliptic curve digital signature algorithm used for Non-Human Identity (NHI) credential issuance and verification |
| **ErrorCategory** | Enum in `factory-core` classifying CI/CD pipeline errors: `CodeCompilation`, `LintViolation`, `TestFailure`, `SecurityAudit`, `InfrastructureBuild`, `InfrastructureTransient`, `Unknown` |

## F

| Term | Definition |
|:---|:---|
| **FactoryError** | Central error type in `factory-core::error` with variants: `Config`, `Network`, `Internal`, `Security`, `Agent`, `Mission`, `Unexpected`, `Storage`, `IoError`, `RemediationError` |
| **FinOps** | Financial Operations — practice of tracking and optimizing cloud spend; implemented via `FinOpsTag` virtual headers and `DailyBudgetConfig` hardstops |
| **FinOpsAgent** | Factory agent tracking token consumption, budget enforcement, and spend velocity alerts |
| **FinOpsTag** | Struct with `team`, `epic`, `microservice`, `environment`, `cost_center` fields; serialized as `x-vtags-*` HTTP headers for LLM request attribution |
| **FluxCD** | GitOps controller that deploys the factory Kubernetes manifests from Git repositories |

## G

| Term | Definition |
|:---|:---|
| **gVisor** | Google's container sandbox runtime providing kernel-level isolation for agent execution pods |
| **GitOps** | Infrastructure-as-code deployment methodology where Git is the single source of truth |

## H

| Term | Definition |
|:---|:---|
| **Hatchet** | Workflow orchestration engine that executes the 6-phase mission DAG with step-level retry and timeout management |
| **Hazitek 2026** | Basque Country R&D innovation grant program (via SPRI) funding the Dark Gravity project |
| **HITL** | Human-in-the-Loop — governance model ensuring humans retain oversight at 4 critical vertices |
| **HITL Vertex 1** | Strategic Injection — Product Owner creates Epics tagged `autonomous-plan` |
| **HITL Vertex 2** | Sprint Mobilization — Tech Lead approves decomposed Spec-Kit tasks |
| **HITL Vertex 3** | Exception Override — Architect resolves Agent-Stuck deadlocks |
| **HITL Vertex 4** | Categorical Scrutiny — Senior Reviewer reviews design & merges PR/MR (agents strictly forbidden from merging) |

## J

| Term | Definition |
|:---|:---|
| **JitToken** | Just-In-Time ephemeral security token with automatic memory zeroization (`zeroize::ZeroizeOnDrop`) |

## K

| Term | Definition |
|:---|:---|
| **Kafka KRaft** | Apache Kafka running in KRaft mode (no ZooKeeper) for event streaming between factory subsystems |
| **kube-rs** | Rust Kubernetes client library used for pod management, sandbox orchestration, and cluster introspection |

## L

| Term | Definition |
|:---|:---|
| **LiteLLM** | Multi-provider LLM proxy that routes inference requests to Azure OpenAI, Ollama, or other backends with unified API |

## M

| Term | Definition |
|:---|:---|
| **MCP** | Model Context Protocol — JSON-RPC protocol for AI tool integration; the factory exposes tools via `factory-mcp-server` |
| **Mission** | A complete unit of autonomous work: an issue or PR event that triggers a 6-phase DAG execution |
| **MissionStatus** | Enum: `Pending`, `Running`, `Completed`, `Failed` |

## N

| Term | Definition |
|:---|:---|
| **NHI** | Non-Human Identity — cryptographic Ed25519 credential proving agent authenticity; issued per mission and verified before code execution |

## O

| Term | Definition |
|:---|:---|
| **OKF** | Open Knowledge Format — markdown documentation standard with YAML frontmatter, source path references, and architectural tags |
| **Onion Architecture** | Layered architecture where dependencies point inward: Domain (factory-core) → Application (factory-application) → Infrastructure (factory-infrastructure) → Interface (factory-mcp-server) |
| **OpenZiti** | Zero Trust overlay networking providing encrypted tunnels between factory services without traditional VPN |
| **OSR** | Orphan Symbol Rate — percentage of public Rust AST symbols not documented in the wiki; quality gate: OSR < 5% |

## P

| Term | Definition |
|:---|:---|
| **PipelineFailureEvent** | Domain event representing a detected CI/CD pipeline failure from GitHub Actions or GitLab CI |
| **PipelineScopeFilter** | Utility evaluating whether a CI run is in-scope for remediation based on active PR/MR context |
| **PolledIssueEvent** | Domain event from the outbound poller representing a newly detected issue eligible for autonomous processing |
| **PRDirective** | Enum: `Spec`, `Refine`, `Retry`, `Status`, `Validate`, `Interact` — bot commands parsed from PR comments |
| **PRCommentEvent** | Domain event capturing a parsed `@darkgravity` or `@dark-gravity` PR comment with the extracted directive |

## Q

| Term | Definition |
|:---|:---|
| **QAObserverAgent** | Factory agent monitoring test coverage, quality metrics, and gate enforcement |

## R

| Term | Definition |
|:---|:---|
| **R2R (RAGtoRiches)** | GraphRAG retrieval engine providing context-aware code search and knowledge retrieval for agent planning |
| **Red-Green-Refactor** | TDD discipline: write failing test (Red), implement to pass (Green), improve code structure (Refactor) |
| **RemediationOutcome** | Final result of a remediation attempt with status, fix PR URL, duration, and pipeline pass status |
| **RemediationStatus** | Enum: `Pending`, `InProgress`, `Success`, `Failed`, `Escalated`, `Skipped` |
| **RustantAgent** | Planner agent executing the 6-phase Spec-Kit SDD sequence: init → specify → plan → execute → verify → git-commit |

## S

| Term | Definition |
|:---|:---|
| **SandboxConstraint** | Struct defining pod resource limits: `max_memory_mb` (≤30), `max_cpu_cores` (≤0.25), `network_egress_allowed` (false) |
| **SAST** | Static Application Security Testing — automated code scanning via Semgrep; enforced as a gate before code execution |
| **SastScanResult** | Security scan output with `score` (0–10), `is_safe`, `findings`, `critical_vulnerabilities_detected`; passes gate when score ≥ 8.0 |
| **SDD** | Spec-Driven Development — the engineering methodology where specs, plans, and tasks are generated before code |
| **SddMissionPlan** | Structured plan output from RustantAgent containing parsed SDD tasks for Hatchet orchestration |
| **SddTaskItem** | Individual task from `tasks.md` with `id`, `description`, `is_parallel`, `dependencies`, `target_files` |
| **SecurityValidator** | Trait in `factory-core::security` for Ed25519 signature validation and content auditing |
| **Spec-Kit** | The specification toolkit providing `/speckit-specify`, `/speckit-plan`, `/speckit-tasks`, `/speckit-implement` commands |
| **SPRI** | Sociedad para la Promoción y Reconversión Industrial — Basque government agency administering Hazitek grants |
| **SurgicalPatch** | Struct for file-level code modifications: `file_path`, `search_block`, `replace_block` |

## T

| Term | Definition |
|:---|:---|
| **Task** | Individual unit of work within a Mission, with `assigned_agent`, `dependencies`, and `status` |
| **TaskStatus** | Enum: `Queued`, `Active`, `Finished`, `Blocked` |
| **TDD** | Test-Driven Development — methodology enforced by ZeroClawAgent: Red (write failing test) → Green (implement) → Refactor |

## V

| Term | Definition |
|:---|:---|
| **Vault** | HashiCorp Vault — secret management service for API keys, tokens, and cryptographic material |
| **Verification Triad** | Three-gate quality pipeline: Logical (tests), Architectural (linting/deps), Security (SAST/NHI) |

## Z

| Term | Definition |
|:---|:---|
| **ZeroClawAgent** | Developer agent executing TDD tasks in gVisor-sandboxed pods with SAST pre-scan, checkpoint resumption, and circuit breaker integration |
| **Zero Trust** | Security model where no implicit trust is granted; all agent-to-service communication flows through OpenZiti encrypted tunnels |
| **Zeroize** | Rust crate ensuring sensitive data (JIT tokens, private keys) is cryptographically wiped from memory on drop |