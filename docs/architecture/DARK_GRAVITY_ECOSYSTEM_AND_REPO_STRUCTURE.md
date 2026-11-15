# Dark Gravity Autonomous Software Factory — Complete Repository Structure & Ecosystem Reference (V7.3 / v0.3.4)

## 1. Executive Definition: What is the Dark Gravity Autonomous Factory?

**Dark Gravity** is a Kubernetes-native, **Continuous Agentic / Continuous Deployment (CA/CD)** software engineering factory built in **Rust (Edition 2024)** and reconciled declaratively via **FluxCD GitOps**.

Instead of relying on ad-hoc "vibe coding" or unisolated cloud agents, Dark Gravity turns a GitHub/GitLab Issue into a verified Pull Request through a deterministic, contract-driven pipeline:
1. **Spec-Driven Development (SDD) Contract Layer (`WHAT`)**: Requirements are decomposed via **Spec-Kit** (`lgcorzo/spec-kit`) into durable design artifacts (`constitution.md` -> `spec.md` -> `plan.md` -> `tasks.md`).
2. **Bridge & Handoff Protocol (`WHAT -> HOW`)**: The **Spec-Kit Superpowers Bridge** (`lgcorzo/speckit-superpowers-bridge`) enforces boundary guard rules and writes `.specify/superpowers-handoff.json`, handing atomic tasks to the execution harness without re-planning.
3. **Agentic Execution Discipline (`HOW`)**: Specialized Rust agents (**Rustant** Planner/Reviewer, **ZeroClaw** Developer/Validator, **Auditor** Security Judge, **FinOpsAgent**, **QAObserver**, **DocAgent**) execute tasks using **Superpowers** (`lgcorzo/superpowers`) TDD, subagent dispatch, and systematic debugging workflows inside ephemeral **gVisor (`runsc`)** sandboxes.
4. **Durable 6-Phase Hatchet DAG & Event Backbone**: Orchestrated by **Hatchet** over **Apache Kafka (KRaft)** topics (`mission-ingestion`, `agent-thought`, `mission-events`) with PostgreSQL (`CloudNativePG`) and self-hosted **MinIO S3** state checkpointing (`StepCheckpoint`).
5. **Defense-in-Depth & 4-Vertex Human-in-the-Loop (HITL) Governance**:
   - **Kernel Isolation**: Untrusted agent compilation and test runs execute in `gVisor` (`io.containerd.runsc.v1`) pods clamped to `512Mi` RAM / `500m` CPU (and `<= 30MiB` for micro-sandboxes).
   - **Dark Network Perimeter**: Zero inbound public ports; outbound polling (`GitPlatformPoller`) and **OpenZiti mTLS 1.3** overlay networking with default-deny egress `NetworkPolicies`.
   - **Cryptographic Non-Human Identity (NHI) & Memory Hygiene**: Every mission and tool invocation is signed with **Ed25519** (`ed25519-dalek` W3C Verifiable Credentials). Ephemeral **HashiCorp Vault JIT tokens** (5-minute TTL) are wiped from RAM on drop via `zeroize` (benchmarked at `54.2 ns`).
   - **Mandatory Human Merge Gate (Vertex 4)**: Autonomous agents can create branches, run tests, and open PRs, but are **strictly prohibited from merging into `main` on core factory repositories**.
   - **Regulatory Readiness Objective**: The architecture implements technical controls for auditability and end-to-end traceability, with future alignment to the **EU AI Act (Art. 12 & 14)**, **SOC 2 Type II**, and **ISO/IEC 25059** as the long-term roadmap objective (no formal third-party certification is claimed).

---

## 2. Physical Directory & Repository Structures of the Core Factory

### 2.1 Core Execution Engine: `lgcorzo/rust_CACD_autonomous_factory` (Public)

The core orchestrator is structured as a **5-Crate Rust Onion Architecture Workspace** where inner domain layers have zero dependency on outer infrastructure or HTTP/MCP interfaces:

```text
rust_CACD_autonomous_factory/
├── Cargo.toml                           # Workspace manifest (Rust 2024, rust-version = "1.85")
├── AGENTS.md                            # Mandatory HITL & Spec-Kit -> Superpowers agent rules
├── README.md                            # System overview, Evidence Tree & 4-Tier Lifecycle Matrix
├── crates/
│   ├── factory-core/                    # [Layer 1: Domain Core - Zero I/O]
│   │   ├── benches/zeroize_benchmark.rs # Criterion benchmark (54.2 ns forensic RAM wipe)
│   │   └── src/
│   │       ├── lib.rs                   # Mission, Task, SddMissionPlan, SddTaskItem, PRDirective
│   │       ├── error.rs                 # Unified FactoryError taxonomy
│   │       ├── config.rs                # AgentModelConfig & runtime bounds
│   │       ├── executor.rs              # CodeSurgeryExecutor trait
│   │       ├── security.rs              # SecurityValidator, SandboxConstraint, JitToken (Zeroize)
│   │       └── security/nhi.rs          # Ed25519 Non-Human Identity & W3C VC JCS signer
│   │
│   ├── factory-application/             # [Layer 2: Application & Orchestration]
│   │   └── src/
│   │       ├── lib.rs                   # Agent trait & application interfaces
│   │       ├── poller_service.rs        # Outbound GitPlatformPoller & 7-class CI Error Classifier
│   │       ├── telemetry_export.rs      # Prometheus / OpenTelemetry & S3 compliance exporter
│   │       ├── agents/
│   │       │   ├── rustant.rs           # RustantAgent (PO Planner & Heterogeneous SAST Reviewer)
│   │       │   ├── zeroclaw.rs          # ZeroClawAgent (Sandboxed Code Executor & Validator)
│   │       │   ├── auditor.rs           # AuditorAgent (Security & OWASP diff evaluator)
│   │       │   ├── finops.rs            # FinOpsAgent ($45/$50 90% HardStop & $1/min velocity guard)
│   │       │   ├── qa_observer.rs       # QAObserverAgent (Test coverage & regression verifier)
│   │       │   └── doc_agent.rs         # DocumentationAgent (OpenWiki / OKF & C4 generator)
│   │       ├── workflows/
│   │       │   ├── autonomous_mission.rs# Hatchet 6-Phase Durable DAG (Plan->Exec->Val->Rev->Deliver)
│   │       │   ├── circuit_breaker.rs   # CircuitBreakerGuard (3-retry cap, A-B-A-B diff-hash loop guard)
│   │       │   ├── develop_task.rs      # Per-task TDD execution workflow
│   │       │   ├── pipeline_remediation.rs # Automated CI/CD failure repair workflow
│   │       │   ├── comment_control.rs   # Interactive PR directives (@darkgravity /status, /refine, etc.)
│   │       │   └── deep_research.rs     # Deep research & R2R GraphRAG synthesis workflow
│   │       └── bridge/
│   │           ├── adk_driver.rs        # ADK-Rust agent runtime bridge
│   │           ├── kafka_bridge.rs      # Event streaming bridge
│   │           ├── semantica_bridge.rs  # Causal lineage & provenance bridge
│   │           └── state.rs             # Durable StepCheckpoint state manager
│   │
│   ├── factory-infrastructure/          # [Layer 3: External Adapters]
│   │   └── src/
│   │       ├── github.rs / gitlab.rs    # Git platform REST/GraphQL adapters
│   │       ├── git_poller.rs            # Outbound work-item & PR comment poller
│   │       ├── jira.rs                  # Jira Cloud Sprint & Epic adapter
│   │       ├── kafka.rs                 # Confluent Kafka (rdkafka) producer/consumer
│   │       ├── mcp_client.rs            # JSON-RPC over SSE MCP client
│   │       ├── r2r.rs                   # R2R GraphRAG semantic retrieval client
│   │       ├── semantica.rs             # Semantica knowledge graph & decision lineage client
│   │       ├── s3.rs                    # Sovereign MinIO S3 artifact & checkpoint client
│   │       ├── vault.rs                 # HashiCorp Vault 5-minute JIT token client
│   │       ├── ziti.rs                  # OpenZiti mTLS dark mesh adapter
│   │       ├── sentry.rs                # Sentry crash & anomaly telemetry adapter
│   │       └── finops_stop_store.rs     # Persistent FinOps budget circuit-breaker store
│   │
│   ├── factory-mcp-server/              # [Layer 4: Interface - Axum MCP Server (:8100)]
│   │   └── src/
│   │       ├── main.rs / lib.rs         # Axum HTTP/SSE JSON-RPC server entrypoint
│   │       ├── sandbox.rs               # Ephemeral Kubernetes gVisor Job/Pod launcher
│   │       ├── github_webhook.rs        # HMAC-SHA256 webhook receiver
│   │       └── tools/                   # 15+ MCP tools (plan_mission, execute_code, run_tests, etc.)
│   │
│   └── factory-cli/                     # [Layer 5: Binary Entrypoint & K8s Operator]
│       └── src/
│           ├── main.rs                  # CLI dispatcher (worker, poller, trigger_mission, operator)
│           └── operator/                # AgentWorkspace CRD controller (kube-rs, ADR-008)
│
├── config/                              # Declarative runtime configs (gVisor profile, LiteLLM, NetworkPolicy)
├── deploy/                              # Kubernetes ValidatingAdmissionPolicies & CRD manifests
├── evidence/                            # Empirical verification ledger
│   ├── verified_status.json             # Machine-readable CI & test verification state
│   ├── baseline.md                      # Live cluster workload & zero-trust baseline
│   ├── reliability-tests.md             # Worker kill recovery & circuit-breaker evidence
│   ├── security-tests.md                # Secret redaction & gVisor 512Mi resource profiling
│   ├── benchmarks/                      # Criterion & sandbox resource profiling JSONs
│   ├── missions/mission-001/            # End-to-end mission telemetry, timeline & SHA-256 checksums
│   └── security/                        # Threat model, held-out adversarial & corpus results
├── specs/                               # Spec-Kit feature specifications (001 through 019)
├── wiki/                                # Canonical ISO/IEC/IEEE 42010 & 15289 OKF Wiki
│   ├── Home.md / README.md / GLOSSARY.md
│   ├── architecture/                    # business_context, strategic_design, tactical_design, agent_specs
│   ├── modules/                         # Per-crate AST module documentation
│   ├── operations/                      # production_operations, user_manual, lab_notebook_exp_*
│   └── security/                        # security_architecture, hitl_governance, compliance_audit
└── docs/                                # Operational guides, screencast walkthrough & audit reports
```

---

### 2.2 GitOps Control Plane: `gitops_internal_lgcorzo` (Private Infrastructure Repository)

*(Note: In public documentation, this repository is referred to generically as the **Internal GitOps Control Plane** to preserve private repository boundaries per `DOC-011`.)*

```text
gitops_internal_lgcorzo/
├── README.md                            # Cluster architecture, 10-namespace map & operations guide
├── DOCUMENTATION_ECOSYSTEM_AUDIT.md     # Full 72-repository audit report
├── DOCUMENTATION_REMEDIATION_PLAN.md    # Prioritized P0-P3 remediation plan
├── DOCUMENTATION_STANDARDS.md           # Ecosystem-wide documentation standards (DOC-001 to DOC-015)
├── DOCUMENTATION_ECOSYSTEM_STATUS.md    # Live repository documentation scorecard
├── clusters/                            # FluxCD cluster sync entrypoints
├── infrastructure/                      # Declarative Kubernetes manifests & HelmReleases
│   ├── agents/                          # Dark Gravity workers, MCP server, gVisor RuntimeClass
│   ├── orchestrators/                   # Hatchet Engine (:7077/:8000) & KEDA autoscalers
│   ├── confluent/                       # Apache Kafka (KRaft mode) brokers & topics
│   ├── llm-apps/                        # Dark-LiteLLM Gateway, R2R GraphRAG, Semantica, MLflow, Ollama
│   ├── storage/                         # CloudNativePG (PostgreSQL 16 + pgvector), MinIO S3, Redis, RabbitMQ
│   ├── security/                        # Keycloak OIDC, HashiCorp Vault, SealedSecrets
│   ├── openziti/                        # OpenZiti Controller & Edge Router dark mesh
│   ├── monitoring/                      # Prometheus, Grafana, Loki, Tempo, OpenTelemetry, DCGM
│   └── development/                     # Ephemeral sandbox execution bounds & dev workspaces
├── .agents/skills/
│   ├── autonomous_vulnerability_monitor/# Hourly Project 6 Dependabot/CodeQL scanner & Jules dispatcher
│   ├── repo_security_audit/             # Portfolio-wide branch protection & security auditor
│   ├── ast-documentation-generator/     # AST -> OpenWiki / OKF generator skill
│   └── uml2-okf-documenter/             # ISO 42010 / UML 2.0 Mermaid documenter skill
├── specs/003-sync-docs-notebooklm/      # NotebookLM domain bundle compiler & verification scripts
└── docs/                                # Operational manuals (OpenZiti, SealedSecrets, Disaster Recovery)
```

---

### 2.3 Supporting Application & Governance Repositories (Directory Structures)

#### A. `lgcorzo/mlops-python-package` (Industrial MLOps Reference Standard)
```text
mlops-python-package/
├── pyproject.toml / poetry.lock         # Deterministic Python 3.12+ Poetry lockfile
├── security-audit-requirements.txt      # SHA-256 hash-pinned pip-audit lockfile
├── confs/                               # Hydra / OmegaConf ML pipeline configurations
├── src/bikes/                           # DDD MLOps package (data, features, models, evaluation, SHAP)
├── tests/                               # Unit & integration pytest suite (98 tests)
├── wiki/ & openwiki/                    # ISO 42010 / OpenWiki documentation (verified=false until human review)
└── .github/workflows/                   # Ruff, Mypy, Bandit, Pytest, CycloneDX SBOM & CodeQL gates
```

#### B. `lgcorzo/llmops-python-package` (Enterprise LLMOps & Multi-Agent Core)
```text
llmops-python-package/
├── pyproject.toml                       # Python >=3.11,<3.13 Poetry package (`autogen_team`)
├── confs/                               # OmegaConf jobs & `mcp_prompts.yaml`
├── src/autogen_team/
│   ├── core/                            # Shared kernel schemas & security protocols
│   ├── application/                     # Hatchet workflows, Agent personas & MCP Server
│   ├── infrastructure/                  # Kafka A2A, Hatchet client, MinIO S3, gVisor sandbox adapter
│   ├── models/ & registry/              # LLM model & state registries
│   └── evaluation/                      # Prompt & RAG evaluation bounded context
├── wiki/ & openwiki/                    # AST-synchronized OpenWiki architecture documentation
└── tests/                               # Comprehensive pytest & MCP integration suite
```

#### C. `lgcorzo/fastapi-autogen-team` (High-Throughput Native Rust Agent Service)
```text
fastapi-autogen-team/
├── Cargo.toml                           # Native Rust service (Axum 0.7 + Rig + Tokio)
├── src/
│   ├── domain/                          # AgentTeam orchestrator (Planner, Searcher, QA/Expert)
│   ├── application/                     # OpenAI-compatible DTOs & request validation
│   ├── infrastructure/                  # LiteLLM, R2R RAG, Jira, Confluence & OpenTelemetry clients
│   └── interface/                       # Axum SSE streaming handlers (`/agent/api/v1beta/chat/completions`)
├── tests/                               # DDD test suite (unit, integration, smoke, security)
└── generate_openwiki.py / fix_openwiki.py # Offline documentation maintenance helpers
```

---

## 3. Complete Explanation of All Repositories Used in the Dark Gravity Ecosystem

Every repository under `@lgcorzo` (66 public repositories + internal infrastructure/remediation components) serves a specific architectural role in the **Dark Gravity Autonomous Software Factory**, grouped into **6 Functional Domains**:

### Domain 1: Core Factory Orchestration, SDD Governance & Agent Execution
1. **[`lgcorzo/rust_CACD_autonomous_factory`](https://github.com/lgcorzo/rust_CACD_autonomous_factory)**: The core brain of Dark Gravity (`v0.3.4`). Implements the 5-crate Rust 2024 workspace, the Hatchet 6-Phase Durable DAG (`Ingestion -> Plan -> Execute -> Validate -> Review -> Deliver`), `GitPlatformPoller`, Ed25519 NHI credentials, `CircuitBreakerGuard`, `FinOpsAgent`, and the `AgentWorkspace` Kubernetes operator.
2. **[`lgcorzo/spec-kit`](https://github.com/lgcorzo/spec-kit)** *(Hardened Fork of `github/spec-kit`)*: Provides the **Spec-Driven Development (SDD)** CLI (`specify`) and slash-command workflow (`/speckit-constitution`, `/speckit-specify`, `/speckit-clarify`, `/speckit-plan`, `/speckit-tasks`) used by `Rustant` (PO Agent) in Phase 1 to define **WHAT** must be built before any code is written.
3. **[`lgcorzo/superpowers`](https://github.com/lgcorzo/superpowers)** *(Hardened Fork of `obra/superpowers`)*: Provides the mandatory agent execution skills (`test-driven-development`, `executing-plans`, `subagent-driven-development`, `systematic-debugging`, `verification-before-completion`, `requesting-code-review`, `using-git-worktrees`) used by `ZeroClaw` (Developer Agent) in Phases 2–3 to govern **HOW** code is written and tested.
4. **[`lgcorzo/speckit-superpowers-bridge`](https://github.com/lgcorzo/speckit-superpowers-bridge)** *(Hardened Fork of `lihan3238/speckit-superpowers-bridge`)*: The thin protocol contract bridge uniting Spec-Kit (`WHAT`) and Superpowers (`HOW`). Manages `.specify/superpowers-handoff.json`, enforces 5 hardcoded boundary guard rules, and adapts `tasks.md` checkboxes into disposable `Task N` headings for inline execution.
5. **[`lgcorzo/adk-rust`](https://github.com/lgcorzo/adk-rust)** *(Hardened Fork of `zavora-ai/adk-rust`)*: Rust Agent Development Kit providing modular LLM, tool, memory, and session primitives used inside gVisor developer sandboxes.
6. **[`lgcorzo/openevolve`](https://github.com/lgcorzo/openevolve)** *(Hardened Fork of `algorithmicsuperintelligence/openevolve`)*: Evolutionary coding agent framework used for iterative algorithmic optimization and candidate patch evaluation under strict CodeQL and `pip-audit` gates.
7. **Internal GitOps Control Plane & Incident Remediator** *(Private Repositories)*:
   - **Internal GitOps Control Plane**: Manages the 10-namespace MicroK8s cluster via FluxCD, Bitnami SealedSecrets, OpenZiti dark overlay networking, and the hourly **GitHub Project 6 Autonomous Vulnerability Monitor**.
   - **Internal Incident Remediation Service**: Listens to FluxCD/cluster alerts, logs remediation experiments to MLflow, and dispatches automated fix PRs.
   - **Legacy Architectural Prototype (`autonomous_factory`)**: Historical GitLab specification repository whose SRS, SAR, and ADRs have been consolidated into `rust_CACD_autonomous_factory/wiki/` and synchronized with the V7.3 Rust runtime.

---

### Domain 2: Cognitive Memory, GraphRAG, Semantic Lineage & LLM Gateway
8. **[`lgcorzo/dark-litellm`](https://github.com/lgcorzo/dark-litellm)** *(Hardened Fork of `BerriAI/litellm`)*: Sovereign multi-model LLM gateway and FinOps proxy running in the `llm-apps` namespace (`:4000/:4001`). Routes agent inference calls across local GPU models (`Qwen 2.5 Coder 7B`, `Bonsai 27B` via Ollama) and external frontier models while injecting FinOps virtual tags (`x-vtags-team`, `x-vtags-epic`, `x-vtags-cost_center`) and enforcing spend limits.
9. **[`lgcorzo/R2R-Engine`](https://github.com/lgcorzo/R2R-Engine)** & **[`lgcorzo/R2R-Application`](https://github.com/lgcorzo/R2R-Application)** *(Hardened Mirrors of `SciPhi-AI/R2R`)*: Production Retrieval-Augmented Generation (GraphRAG) engine (`:7272`) and Next.js observability UI backed by CloudNativePG `pgvector` and MinIO (`r2r-documents`). Supplies semantic code and documentation context to `Rustant` and `ZeroClaw` via the `retrieve_context` and `index_code` MCP tools.
10. **[`lgcorzo/semantica`](https://github.com/lgcorzo/semantica)** *(Hardened Fork of `Hawksight-AI/semantica`)*: Semantic graph memory and causal decision-lineage engine (`ContextGraph`). Records every agent architectural decision, requirement-to-AST mutation link, and SAST review outcome (`CAUSED`, `INFLUENCED`, `PRECEDENT_FOR`) into the `semantica-provenance` bucket for auditability.
11. **[`lgcorzo/knowledge-catalog`](https://github.com/lgcorzo/knowledge-catalog)** *(Hardened Fork)*: Metadata and data asset catalog used to index datasets, schemas, and governance lineage across the lab.

---

### Domain 3: AST Code Intelligence & Living Documentation Engines
To prevent documentary amnesia and maintain an **Orphan Symbol Rate ($	ext{OSR} < 5\%$)**, Dark Gravity uses five specialized AST analysis and wiki generation repositories:
12. **[`lgcorzo/deepwiki-rs`](https://github.com/lgcorzo/deepwiki-rs)** *(Hardened Fork of `sopaco/deepwiki-rs`)*: High-performance Rust Tree-sitter AST parser and Repomix context indexer deployed in the `knowledge` namespace. Extracts call graphs, struct/trait hierarchies, and symbol tables directly from Rust/Python/Go source code.
13. **[`lgcorzo/dark-code-review-graph`](https://github.com/lgcorzo/dark-code-review-graph)** *(Hardened Fork of `tirth8205/code-review-graph`)*: Builds persistent AST call-graph and dependency communities (e.g., 254 nodes and 1,522 edges in `rust_CACD_autonomous_factory`), enabling blast-radius analysis (`get_impact_radius`), affected flow tracing, and automated PR review context.
14. **[`lgcorzo/dark-graphify`](https://github.com/lgcorzo/dark-graphify)** *(Hardened Fork of `safishamsi/graphify`)*: Converts multi-modal repository artifacts (code, ADRs, papers, diagrams) into a queryable knowledge graph (`graphify-out/`) with community detection and shortest-path explanation tools.
15. **[`lgcorzo/openwiki`](https://github.com/lgcorzo/openwiki)** *(Hardened Fork of `TryDotAt/openwiki`)*: Autonomous CLI and GitHub Actions engine that generates and refreshes `openwiki/` architecture, workflow, and source-map documentation on every commit.
16. **[`lgcorzo/CodeWiki`](https://github.com/lgcorzo/CodeWiki)** *(Hardened Fork of `FSoft-AI4Code/CodeWiki`)*: Holistic repository-level AST comprehension framework used alongside `openwiki` for deep module summarization.
17. **[`lgcorzo/deepwiki-open`](https://github.com/lgcorzo/deepwiki-open)** *(Hardened Fork of `AsyncFuncAI/deepwiki-open`)*: AI-powered interactive wiki UI for browsing repository architecture graphs.
18. **[`lgcorzo/Documentation_agent`](https://github.com/lgcorzo/Documentation_agent)**: First-party autonomous documentation worker that combines `deepwiki-rs`, `openwiki`, and ISO/IEC/IEEE 42010 / 15289 templates to generate OKF documentation and UML 2.0 Mermaid diagrams.

---

### Domain 4: Owned Production Applications & Industrial AI Workloads (Agent Targets)
These first-party repositories are both production services and continuous target workloads maintained by the Dark Gravity factory:
19. **[`lgcorzo/fastapi-autogen-team`](https://github.com/lgcorzo/fastapi-autogen-team)**: High-throughput native Rust (`Axum` + `Rig`) microservice exposing OpenAI-compatible SSE streaming endpoints (`/agent/api/v1beta/chat/completions`) for multi-agent squads (Planner, Searcher, QA/Expert) integrated with R2R, Jira, and Confluence.
20. **[`lgcorzo/mlops-python-package`](https://github.com/lgcorzo/mlops-python-package)**: Industrial MLOps reference standard (`bikes`) featuring DVC data/model versioning, MLflow tracking, Hydra configuration, SHAP explainability, CycloneDX SBOM generation, and SHA-256 hash-pinned security lockfiles.
21. **[`lgcorzo/llmops-python-package`](https://github.com/lgcorzo/llmops-python-package)**: Domain-Driven LLMOps platform (`autogen_team`) adhering to ISO/IEC/IEEE 42010:2022 and 15289:2019 standards, providing Hatchet workflow DSLs, MCP prompt registries, and automated evaluation pipelines.
22. **[`lgcorzo/devops_job_shop_scheduling`](https://github.com/lgcorzo/devops_job_shop_scheduling)**: Industrial operations research and metaheuristic/genetic algorithm optimization for the Job Shop Scheduling Problem (JSP), bridging manufacturing optimization with CI/CD automation.
23. **[`lgcorzo/cod_fish_classification_keras_model`](https://github.com/lgcorzo/cod_fish_classification_keras_model)**: Industrial computer vision pipeline for conveyor-belt fish species classification (`HYSE`, `SEI`, `TORSK`, `LASK`) and length estimation at 2 units/sec using Faster R-CNN (VGG-16 / Keras).
24. **[`lgcorzo/MLOPS_Repo_structure_template`](https://github.com/lgcorzo/MLOPS_Repo_structure_template)**: Standardized Cookiecutter/template blueprint for bootstrapping new MLOps and LLMOps repositories in the factory.
25. **[`lgcorzo/lgcorzo.github.io`](https://github.com/lgcorzo/lgcorzo.github.io)** & **[`lgcorzo/lgcorzo`](https://github.com/lgcorzo/lgcorzo)**: Public engineering lab portal (`https://lgcorzo.github.io/`) and GitHub profile overview presenting the live 4-Tier Operational Status Matrix, interactive architecture, mission lifecycle, and fork governance policy.

---

### Domain 5: Sovereign S3 Object Storage, KMS & Hardware SIMD Acceleration (38 Decoupled MinIO Ecosystem Repositories)
To eliminate external cloud storage dependencies and insulate the cluster from upstream binary/console distribution removals, Dark Gravity maintains **38 decoupled Go/TypeScript repositories** built hermetically to `ghcr.io/lgcorzo/*` under immutable `v*.*.*-lgcorzo.*` tags:
26. **Core S3 Server, K8s Operator & Admin Stack (7 repos)**:
    - [`minio`](https://github.com/lgcorzo/minio): High-performance S3-compatible object storage server hosting `factory-artifacts`, `dg-factory-checkpoints`, `semantica-provenance`, `r2r-documents`, and `mlflow` buckets.
    - [`operator`](https://github.com/lgcorzo/operator): Kubernetes operator managing S3 tenant instances in the `storage` namespace.
    - [`console`](https://github.com/lgcorzo/console): Web management UI for monitoring S3 health, buckets, and quotas.
    - [`mc`](https://github.com/lgcorzo/mc): MinIO Client CLI for bucket, policy, and mirror administration.
    - [`directpv`](https://github.com/lgcorzo/directpv): Direct-attached NVMe Kubernetes CSI storage driver for high-IOPS database and object storage volumes.
    - [`mds`](https://github.com/lgcorzo/mds): Distributed directory service primitives for object storage.
    - [`docs`](https://github.com/lgcorzo/docs): Sovereign AIStor architectural charter and Go module decoupling specifications.
27. **Cryptographic KMS, mTLS & Stream Encryption (5 repos)**:
    - [`kes`](https://github.com/lgcorzo/kes) *(Archived Reference Fork)*: Stateless Key Encryption Server compatible with HSM/Vault backends.
    - [`kms-go`](https://github.com/lgcorzo/kms-go): Go cryptographic key management client driver.
    - [`sio`](https://github.com/lgcorzo/sio): Data-At-Rest Encryption (DARE) authenticated encryption stream library.
    - [`mtls`](https://github.com/lgcorzo/mtls): Mutual TLS authentication library for inter-node communication.
    - [`certgen`](https://github.com/lgcorzo/certgen): Zero-dependency mTLS certificate generator for cluster nodes.
28. **Hardware SIMD Primitives — AVX-512, AVX2 & ARM64 NEON Assembly (6 repos)**:
    - [`sha256-simd`](https://github.com/lgcorzo/sha256-simd): Hardware-accelerated SHA-256 hashing (>3 GB/s per core) for artifact and checkpoint integrity.
    - [`highwayhash`](https://github.com/lgcorzo/highwayhash): SIMD HighwayHash implementation exceeding 10 GB/s per core.
    - [`blake2b-simd`](https://github.com/lgcorzo/blake2b-simd): SIMD-accelerated BLAKE2b cryptographic hashing in pure Go.
    - [`md5-simd`](https://github.com/lgcorzo/md5-simd): AVX-512/AVX2 accelerated MD5 hashing for S3 multipart ETag computation.
    - [`simdjson-go`](https://github.com/lgcorzo/simdjson-go): Multi-gigabit SIMD JSON stream parser for high-throughput Kafka/S3 telemetry ingestion.
    - [`crc64nvme`](https://github.com/lgcorzo/crc64nvme): Hardware-accelerated CRC64-NVMe checksum verification via carryless multiplication.
29. **Go SDKs, Load Balancing, Benchmarking & Core Support Libraries (20 repos)**:
    - **SDKs & Routing**: [`minio-go`](https://github.com/lgcorzo/minio-go) (S3 Go SDK with local Docker Quickstart), [`madmin-go`](https://github.com/lgcorzo/madmin-go) (Admin SDK), [`sidekick`](https://github.com/lgcorzo/sidekick) (High-speed HTTP load-balancing proxy).
    - **Benchmarking & Testing**: [`warp`](https://github.com/lgcorzo/warp) (S3 throughput/latency benchmark), [`dperf`](https://github.com/lgcorzo/dperf) (Network throughput benchmark), [`mint`](https://github.com/lgcorzo/mint) (S3 correctness test framework), [`multipart-debug`](https://github.com/lgcorzo/multipart-debug).
    - **System & Parser Libraries**: [`pkg`](https://github.com/lgcorzo/pkg), [`cli`](https://github.com/lgcorzo/cli), [`mux`](https://github.com/lgcorzo/mux), [`websocket`](https://github.com/lgcorzo/websocket), [`dnscache`](https://github.com/lgcorzo/dnscache), [`filepath`](https://github.com/lgcorzo/filepath), [`colorjson`](https://github.com/lgcorzo/colorjson), [`csvparser`](https://github.com/lgcorzo/csvparser), [`xxml`](https://github.com/lgcorzo/xxml), [`zipindex`](https://github.com/lgcorzo/zipindex), [`pkger`](https://github.com/lgcorzo/pkger), [`selfupdate`](https://github.com/lgcorzo/selfupdate), [`asm2plan9s`](https://github.com/lgcorzo/asm2plan9s), [`minio-cf`](https://github.com/lgcorzo/minio-cf).

---

### Domain 6: Hardware Acceleration, Quantitative AI & Educational Research Repositories (7 repos)
30. **[`lgcorzo/gpu-unlock`](https://github.com/lgcorzo/gpu-unlock)** *(Hardened Fork of `Zagitta/gpu-unlock`)*: Early-boot Linux kernel module (`dkms` + `initramfs-tools`) unlocking full FP16/BF16/TF32 tensor instruction issue rates on consumer NVIDIA RTX 30-series GPUs (GA102–GA106) used by the local Ollama/LiteLLM inference node.
31. **[`lgcorzo/FinRobot`](https://github.com/lgcorzo/FinRobot)** & **[`lgcorzo/TradingAgents`](https://github.com/lgcorzo/TradingAgents)**: Multi-agent financial analysis and quantitative market simulation frameworks maintained under strict Python security audit gates.
32. **[`lgcorzo/dark-rea`](https://github.com/lgcorzo/dark-rea)**: Real-estate and domain-specific analytical automation fork maintained with deterministic CI/CD checks.
33. **[`lgcorzo/GEN_AI_Course`](https://github.com/lgcorzo/GEN_AI_Course)**, **[`lgcorzo/ai-agents-for-beginners`](https://github.com/lgcorzo/ai-agents-for-beginners)**, and **[`lgcorzo/mocogpt`](https://github.com/lgcorzo/mocogpt)**: Educational notebooks, Generative AI labs, and lightweight GPT architectures.
