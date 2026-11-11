# Dark Gravity: CACD Autonomous Agent Factory

[![CI/CD Pipeline](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/pipeline.yml/badge.svg)](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/pipeline.yml)
[![Wiki Content Sync](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/docs-to-wiki.yml/badge.svg)](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/docs-to-wiki.yml)
[![Walkthrough source](https://img.shields.io/badge/Walkthrough-Source-blueviolet.svg)](docs/screencast_walkthrough.md)
[![Docker Pulls](https://img.shields.io/docker/pulls/lgcorzo/dark-gravity-factory.svg)](https://hub.docker.com/r/lgcorzo/dark-gravity-factory)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/lgcorzo/rust_CACD_autonomous_factory)

**Dark Gravity** is a high-performance, long-term agentic system serving as the core intelligence for the **CA/CD Autonomous Agent Factory**. Rebuilt from the ground up in **Rust**, it orchestrates complex multi-agent workflows with zero-trust security and durable task execution.

> **[Walkthrough source](docs/screencast_walkthrough.md)** — This local Markdown artifact is rendered by a script. It describes issue polling, Hatchet orchestration, gVisor (`RuntimeClass` `gvisor`, handler `runsc`), PR directives, and Human-in-the-Loop (HITL) review.
>
> The sandbox profile sets worker limits to `30Mi`/`250m` and sidecar limits to `20Mi`/`100m`. RuntimeClass overhead adds `35Mi`/`100m`. The configured sums are `85Mi`/`450m` in limits and `60Mi`/`250m` in requests. These are configuration values, not measured usage. The job builder separately defaults to `512Mi`/`500m`, which conflicts with the admission-policy cap of `30Mi`/`250m`. Actual cluster usage is **TO CONFIRM**.

---

## Walkthrough source and evidence

### Walkthrough source
The [local walkthrough](docs/screencast_walkthrough.md) is rendered from a script. It is not proof of current runtime behavior.

### Evidence artifacts
Evidence files have different scopes and limits. Read [evidence/README.md](evidence/README.md) before relying on an artifact. The historical baseline is pending regeneration and is not proof of current runtime state.

```
evidence/
├── 📄 baseline.md                       # Historical baseline, pending regeneration
├── 📄 reliability-tests.md              # Two resilience scenarios: interrupted-mission recovery and broken-test safe shutdown
├── 📄 security-tests.md                 # Secret-redaction scenarios (2), code inspection, gVisor resource benchmark; not SAST/DAST or pen-testing
├── 📁 security/
│   ├── 📄 heldout_results.json          # Held-out adversarial evaluation metrics
│   ├── 📄 corpus_results.json           # Labeled diff scanner corpus evaluation metrics
│   └── 📄 threat-model.md               # Dark Gravity V7.2 Threat Model & Zero-Trust Audit Matrix
├── 📁 followup/
│   └── 📄 verification_report.md        # Follow-up report; see its stated scope
├── 📁 benchmarks/
│   └── 📄 benchmark_summary.json        # Double-blind evaluation protocol status (status: not_run)
└── 📁 missions/
    └── 📁 mission-001/
        ├── 📄 checksums.txt             # SHA-256 Digest Manifest for Mission Telemetry Bundles
        ├── 📄 report.json                # Mission Execution Summary & Verification Metrics
        └── 📄 timeline.md               # Chronological Event Log of Mission Operations
```

#### Quick links to evidence
- 📊 **[Historical cluster baseline (`evidence/baseline.md`)](evidence/baseline.md)** — Unverified historical material pending regeneration; do not cite it as proof.
- 🎯 **[Mission 001 Evidence Bundle (`evidence/missions/mission-001/`)](evidence/missions/mission-001/)** — Mission report ([`report.json`](evidence/missions/mission-001/report.json)), event timeline ([`timeline.md`](evidence/missions/mission-001/timeline.md)), and SHA-256 checksums ([`checksums.txt`](evidence/missions/mission-001/checksums.txt)).
- 🛡️ **[Security threat model (`evidence/security/threat-model.md`)](evidence/security/threat-model.md)** — Threat descriptions and mitigation proposals.
- 🧪 **[Security Test Suite (`evidence/security-tests.md`)](evidence/security-tests.md)** & **[Reliability Test Suite (`evidence/reliability-tests.md`)](evidence/reliability-tests.md)** — secret-redaction scenarios and a gVisor resource benchmark (security), and two resilience scenarios (reliability). These are not SAST/DAST, penetration tests or long-haul tests.
- 🧩 **[Component Inventory Matrix (`evidence/architecture/component-inventory.md`)](evidence/architecture/component-inventory.md)** — Hardware, software, and agentic module component mapping.

---

## 🏗 Standards & Architecture

This project uses **LLMOps** and **Domain-Driven Design (DDD)** concepts. `V7.2` and `V7.3` are project-generation labels in existing documentation; release tags use a separate `v0.3.x` form. Cargo package manifests currently say `0.1.0`. The active project-generation label and its release-tag mapping are **TO CONFIRM**.

- **LLMOps Lifecycle**: We follow a robust LLM application lifecycle, including automated prompt engineering, retrieval-augmented generation (RAG) evaluation, and model-in-the-loop security audits to ensure production-grade reliability and observability.
- **DDD-based Structure**: The codebase is architected using DDD principles to maintain a clear separation of concerns. This ensures that the core domain logic remains isolated from infrastructure details and external service adapters (like Jira or R2R), allowing the system to scale and evolve without regression.

The codebase is organized as a unified Rust workspace for maximum modularity and performance.

### 1. Workspace Crate Map

- **`factory-core`**: Innermost domain layer. Manages domain entities (`Mission`, `Task`), a `SecurityValidator` trait for agent output auditing, and unified error handling via `FactoryError`.
- **`factory-application`**: Workflow orchestration layer. Leverages **Rustant** (Planner) and **ZeroClaw** (Executor) to process task workflows. Maintains durably checkpointed state via `StepCheckpoint`s using Hatchet's PostgreSQL backend.
- **`factory-mcp-server`**: Presentation layer. Axum-based Model Context Protocol server exposing tools over Server-Sent Events (SSE) transport with isolated gVisor execution interfaces.
- **`factory-infrastructure`**: Adapter layer. Concrete clients for Jira, R2R (RAG), MinIO/AWS S3, OpenZiti (Zero Trust overlay networks), and Kafka (via `rdkafka`; broker provider is **TO CONFIRM**).
- **`factory-cli`**: Interface entry point. Contains the Hatchet worker initializer and Kafka telemetry querying utilities.

### 2. Durable Orchestration (Phase 12 State-Loop)

Missions are orchestrated using a documented six-phase directed acyclic graph (DAG) in **Hatchet** with database-backed `StepCheckpoint` recovery:
`Ingestion` → `Plan (Rustant)` → `Code (ZeroClaw)` → `Validation (ZeroClaw)` → `Review (Rustant)` → `Delivery (GitOps)`.

### 3. Architecture & System Status Matrix

This status matrix reports checked-in design and implementation facts. Repository files do not establish which workloads are currently deployed:
- **Designed**: Specification, architecture contract, or policy defined in `specs/` or `wiki/`.
- **Implemented**: Source code or configuration is present in the repository.
- **Tested**: A test result is recorded for the identified scope and revision.

| Component / Feature | Category | Status Tier | Source & Policy Reference | Operational Details & Evidence Link |
| :--- | :--- | :--- | :--- | :--- |
| **gVisor (`runsc`) Sandbox** | Infrastructure | **Implemented** | `config/sandbox-gvisor-profile.yaml`; `deploy/policies/runtimeclass-gvisor.yaml` | The RuntimeClass name is `gvisor`. No live deployment is established by these files. |
| **Sandbox resource accounting** | Kernel isolation | **Implemented** | `deploy/crds/agentworkspaces.darkgravity.io.yaml`; `deploy/policies/validating-admission-policies.yaml`; `config/sandbox-gvisor-profile.yaml` | The declared workload cap is `30Mi`/`250m`; the profile's two containers plus `35Mi`/`100m` RuntimeClass overhead account for `85Mi`/`450m` limits and `60Mi`/`250m` requests. The job builder defaults to `512Mi`/`500m`; this conflicts with the cap. |
| **Namespace references** | Deployment configuration | **Designed** | `deploy/policies/validating-admission-policies.yaml`; `config/sandbox-gvisor-profile.yaml`; `config/deny-all-egress-networkpolicy.yaml` | `deploy/` contains no `Namespace` objects; its admission bindings select the namespace name `development`. Other checked-in configuration refers to `factory-sandbox` and the `openziti` namespace label. The deployed namespace inventory is **TO CONFIRM**. |
| **OpenZiti sidecar profile** | Zero-Trust network | **Implemented** | `config/sandbox-gvisor-profile.yaml` | The profile declares sidecar limits of `20Mi`/`100m`; actual runtime usage is **TO CONFIRM**. |
| **Network policy configuration** | Security policy | **Designed** | `config/deny-all-egress-networkpolicy.yaml` | The file contains Kubernetes and Cilium policy objects. The installed container network interface (CNI) is **TO CONFIRM**. |
| **Hatchet 6-phase DAG workflow** | Orchestration | **Implemented** | `crates/factory-application/src/workflows/` | Workflow source is checked in; deployment state is not established by the repository. |
| **LiteLLM Proxy & FinOps Headers** | AI inference | **Implemented** | `config/litellm_proxy_config.yaml` | The configuration includes virtual-key budget settings. A deployed namespace is not established by this file. |
| **CircuitBreakerGuard (Diff Hashing)** | Application Logic | **Implemented** | `crates/factory-application/src/workflows/circuit_breaker.rs` | Prevents infinite retry loops via diff normalization and A-B-A-B oscillation tracking. [Evidence: `reliability-tests.md`](evidence/reliability-tests.md) |
| **Pipeline Error Classifier** | Application Logic | **Implemented** | `crates/factory-application/src/poller_service.rs` | Parses CI logs into 7 canonical error categories. [Evidence: `reliability-tests.md`](evidence/reliability-tests.md) |
| **Vault adapter** | Security adapter | **Implemented** | `crates/factory-infrastructure/src/vault.rs` | Source path for Vault handling. |
| **Ed25519 credentials for non-human identity (NHI)** | Identity | **Implemented** | `crates/factory-core/src/security/nhi.rs` | Source path for identity credential handling. |
| **FinOps hard-stop threshold** | Governance gate | **Implemented** | `crates/factory-application/src/agents/finops.rs` | The default maximum is a daily budget of 50; code computes a hard-stop threshold at 90%. |
| **Multi-Region GPU Failover** | Resiliency | **Designed** | `wiki/architecture/infrastructure_adapters.md` | Multi-region cluster failover spec defined for high availability. [Docs: `wiki/architecture/`](wiki/architecture/) |
| **Autonomous direct main merges** | Governance policy | **Designed** | `AGENTS.md` & `wiki/security/hitl_governance.md` | The documentation describes a human review gate. |

### 4. Integrated Target Repositories

Dark Gravity continuously polls and autonomously executes missions across configured repositories:

| Target Repository | Domain | Autonomous Workflow Focus |
|:---|:---|:---|
| [`lgcorzo/rust_CACD_autonomous_factory`](https://github.com/lgcorzo/rust_CACD_autonomous_factory) | Autonomous Multi-Agent Engine | Self-evolution, DAG workflows & MCP servers |
| [`lgcorzo/lince-rs`](https://gitlab.com/lgcorzo/lince-rs) | Otimization process | Algorithmic optimization to control agents costs |
| [`lgcorzo-lab/autonomous_factory`](https://gitlab.com/lgcorzo-lab/autonomous_factory) | Architecture & Knowledge Base | Spec-Kit SDD contracts, C4 diagrams, OSR audits |
| [`lgcorzo/fastapi-autogen-team`](https://github.com/lgcorzo/fastapi-autogen-team) | Agentic Access Layer | R2R and document services |
| [`lgcorzo/mlops-python-package`](https://github.com/lgcorzo/mlops-python-package) | MLOps & ML Pipelines | Automated testing, DVC/Poetry dependency validation, CI/CD error remediation & HITL governance ([`mlops-python-package/README.md`](mlops-python-package/README.md)) |

### 5. Integrated Intelligence & Security

- **Rustant**: Leverages **R2rClient** for semantic code search and context retrieval to inform specification design and planning.
- **ZeroClaw**: Compiles and executes code in secure **gVisor (runsc) user-space sandboxes** via a sandboxed executor driver.
- **Zero Trust Mesh**: Agent-to-agent communication is fully encrypted and bound to identity namespaces using **OpenZiti mTLS 1.3** overlays.
- **Telemetry**: Kafka clients use `KAFKA_BROKERS`. The CLI and example default to `localhost:9092`; the MCP server uses a different service-style default. The active broker provider and endpoint are **TO CONFIRM**.

---

## 🛡️ Technological Sovereignty, Zero-Trust Security & Sovereign Dependency Policy

The project describes readiness controls aligned to these frameworks; no third-party certification claimed. The frameworks are the **EU AI Act** and **SOC 2 Type II**. `V7.2`/`V7.3` are project-generation labels, not Cargo versions or release tags. Their current mapping is **TO CONFIRM**.

*(For detailed architectural specifications and design records, refer to the [Dark Gravity Multi-Agent Architecture Knowledge Base](https://notebook.google.com/notebook/116d5f03-2fb0-4fa7-bd7c-1c9849920abc))*.

### 1. The Technological Sovereignty Imperative (Why Sovereign Forks?)
In an autonomous software factory where AI agents autonomously synthesize, compile, and deliver production software, relying blindly on third-party commercial cloud services or unmaintained open-source dependencies poses existential operational risks:
- **Mitigation of Upstream Licensing Closures:** When foundational projects alter their licensing model, close public distributions, or abandon open-source governance (as occurred with MinIO), Dark Gravity prevents vendor lock-in and supply-chain disruption by maintaining **semantic mirrors and sovereign forks** under `@lgcorzo`.
- **Dependency auditing:** The GitHub Actions pipeline runs `cargo audit -D warnings` on pushes to `main` or `feature/**`, pull requests to `main`, manual dispatches, and published or edited releases. This does not establish a zero-vulnerability result. No container scanner is identified in that workflow.
- **Air-Gapped Operational Continuity:** The factory operates on private, sovereign Kubernetes infrastructure. Proprietary source code, AST surgical mutations, and causal corporate memory remain strictly within sovereign boundaries.

### 2. Core Sovereignty & Security Architecture
- **Webhook route and exposure:** The router registers `POST /webhooks/github` unconditionally, so the route is enabled at the application level by default. In the default local mode, an empty webhook secret does not require signature validation; protected mode requires a secret. The server binds to `0.0.0.0`, but no Service or Ingress is declared in `deploy/` or `config/`. External exposure is **TO CONFIRM**.
- **Non-human identity (NHI):** The identity module is at [`crates/factory-core/src/security/nhi.rs`](crates/factory-core/src/security/nhi.rs). The repository does not establish that every agent action, commit, or Kafka event is signed.
- **Ephemeral credentials and zeroization:** The benchmark source measures `JitToken::zeroize()` on a 36-character ASCII value. The repository does not record benchmark output or machine details; timing is **TO CONFIRM**.
- **Sandbox resource accounting:** The Custom Resource Definition (CRD) and admission policies cap declared workload limits at `30Mi`/`250m`. The profile plus RuntimeClass overhead accounts for `85Mi`/`450m` in limits and `60Mi`/`250m` in requests. The job builder defaults to `512Mi`/`500m`, which conflicts with the policy cap. These are configured values, not measured usage.
- **Hardware-Accelerated SIMD Primitives:** Sovereign Go/Rust cryptographic and parsing libraries (`sha256-simd`, `simdjson-go`, `highwayhash`, `md5-simd`, `blake2b-simd`) leverage AVX-512 and ARM64 vectorization, enabling high-throughput execution without exceeding hard memory ceilings.
- **Sovereign S3 Storage Tier & Isolated Buckets:** Artifacts are persisted in decoupled S3 storage (`factory-artifacts`, `factory-artifacts/compliance/`, `dg-factory-checkpoints`, `semantica-provenance`, `r2r-documents`, `mlflow`) protected by strict whitelist validation against path traversal.

---

## 🎮 Controlling Dark Gravity from GitLab & GitHub

Dark Gravity includes native outbound polling daemons and event handlers that listen for issues, work items, and pull/merge request discussions across configured GitHub and GitLab repositories.

### 1. Triggering Missions via Issues & Work Items

To trigger an autonomous mission, create an issue or work item in any integrated repository (such as `lgcorzo/rust_CACD_autonomous_factory` or `lgcorzo/lince-rs`):

- **Authorized Authors**: Only repository collaborators and maintainers with triage/write permissions are permitted to trigger missions. External submissions require explicit human labeling.
- **Required Labels** (at least one):
  - `autonomous-mission`
  - `dark-gravity`
- **Resource Limits Definition**:
  Include an explicit resource limit block within the issue description. The poller automatically parses these constraints using regex:
  ```markdown
    Resource limits: CPU: 250m, RAM: 30Mi, Timeout: 300s
  ```

  The example stays within the Custom Resource Definition (CRD) and admission-policy limits. The sandbox job builder currently defaults to `500m` CPU and `512Mi` memory; this conflicts with those limits. See the [fact register](docs/audit/DOC_FACT_REGISTER.md).
- **Autonomous Ingestion & Zero-Trust Identity**:
  1. The poller extracts title, description, labels, and resource limits.
  2. Generates a cryptographic **Ed25519-signed Verifiable Credential (VC)** for Non-Human Identity (NHI) under `did:factory:dark-gravity-euskadi`.
  3. Publishes the mission specification to Kafka topic `mission-input`.
  4. Records the event in the `CursorStore` to ensure **idempotent, single-execution processing** across polling intervals.

### 2. Interactive Directives on PRs and MRs

Dark Gravity actively listens to comments on active GitHub Pull Requests (PRs) and GitLab Merge Requests (MRs). When a developer or architect tags the bot with a directive:

```text
@darkgravity /<command> [optional prompt/instruction]
```
*(Also accepts `@dark-gravity`)*

#### Supported Directives

| Directive | Description | Agent Action |
|:---|:---|:---|
| `@darkgravity /status` | Queries real-time factory health and DAG state. | Generates a markdown system report with repository, PR target, DAG health, sandbox status, and FinOps budget state. |
| `@darkgravity /interact <prompt>` | Natural-language query or technical instruction. | Triggers Rustant (PO) to analyze the query with contextual knowledge and reply directly in the thread. |
| `@darkgravity /spec <prompt>` | Requests a specification review or modification. | Triggers Rustant (PO) to re-evaluate `spec.md` against provided requirements and stage revisions. |
| `@darkgravity /refine <instruction>` | Requests localized code edits or targeted bugfixes. | Triggers ZeroClaw (Dev) to apply surgical code mutations in the gVisor sandbox (≤30MiB RAM). |
| `@darkgravity /validate` | Requests validation. | The repository does not establish that this directive runs a SAST scanner or reports vulnerability results. |
| `@darkgravity /retry` | Forces a restart of the execution loop. | Restarts the Aethelgard verification loop and re-runs failed test suites. |

> **Security & Governance Note on Directives:**
> - **Authorized Operators:** Only repository collaborators and maintainers with write permissions may issue `@darkgravity` directives.
> - **Input Isolation:** All user comments, issue descriptions, and CI error logs are treated as **untrusted data** and parsed strictly into structured event models.
> - **Self-Targeting Guard:** When operating on `rust_CACD_autonomous_factory` itself, Dark Gravity is restricted from modifying `.github/workflows/`, `CODEOWNERS`, `AGENTS.md`, or security enforcement crates without explicit human sign-off.
> - **Strict HITL Gate:** Even with interactive directives, Dark Gravity cannot merge pull requests. All merges into `main` require manual human review and execution.

### 3. Immediate Feedback & Loop Prevention

- **Instant Acknowledgement**: Upon receiving any valid directive, Dark Gravity immediately awards an **`👀` (eyes)** emoji reaction to the user's note/comment to signal receipt before processing begins.
- **Contextual Thread Replies**: Once execution finishes, Dark Gravity replies directly to the originating discussion thread with structured markdown reports.
- **Bot Loop Suppression**: Any notes authored by `DARK_GRAVITY_BOT_USERNAME` (default `darkgravity-bot`) are automatically ignored to prevent recursive comment loops.

### 4. Live Health Probing CLI

Operators can verify GitLab/GitHub connectivity, token scopes, and project accessibility using the native CLI:

```bash
# Verify live GitLab communication and project permissions
cargo run --bin factory-cli -- gitlab-verify \
  --gitlab-token "$GITLAB_API_TOKEN" \
  --gitlab-projects "lgcorzo/lince-rs,lgcorzo/fastapi-autogen-team"

# Output machine-readable JSON scorecard
cargo run --bin factory-cli -- gitlab-verify --json
```

### 5. Branch & Pull Request Policy: Human-in-the-Loop (HITL)

- Autonomous agents create isolated feature branches, commit code, and open Pull Requests (GitHub) or Merge Requests (GitLab).
- **Strict Manual Merge Requirement**: Dark Gravity **never** automatically merges PRs or MRs into `main` or default production branches.
- Once configured CI checks finish, the agent hands off the PR/MR link to a human developer for review. The checked-in workflow does not establish a CodeQL check.

### 6. Autonomous Pipeline Error Remediation

Dark Gravity continuously monitors CI/CD pipelines across GitHub and GitLab repositories to autonomously triage, remediate, or escalate errors:

- **Detection**: The outbound poller queries GitHub Actions (`GET /repos/{repo}/actions/runs?status=failure`) and GitLab CI (`GET /projects/{id}/pipelines?status=failed`), extracting run IDs, failing jobs/steps, and error logs (truncated safely to 10KB).
- **Classification**: Errors are parsed using an ordered regex classification engine into 7 canonical categories:
  - `LintViolation`: `cargo clippy`, `eslint`, formatting warnings/errors.
  - `CodeCompilation`: Rust compiler errors (`error[E0308]`), syntax or type errors.
  - `TestFailure`: Unit, integration, or E2E assertion failures.
  - `InfrastructureBuild`: Dockerfile build failures, apt-get, package installation failures.
  - `InfrastructureTransient`: Network timeouts, rate limits, transient connection drops.
  - `SecurityAudit`: Cargo audit / RUSTSEC advisory failures.
  - `Unknown`: Unrecognized error patterns (always escalated to human review).
- **Autonomous Mission Triggering**: Remediable failures construct a remediation mission payload signed with Ed25519 Non-Human Identity (NHI) credentials and publish to the `mission-input` Kafka topic.
- **Safety Guards & Governance**:
  - **Self-Referential Safety Guard**: Any failure originating from `lgcorzo/rust_CACD_autonomous_factory` is immediately escalated to a human engineer via a GitHub issue, strictly preventing recursion loops.
  - **Concurrency Control**: Remediations are throttled by `MAX_CONCURRENT_REMEDIATIONS` (default: 3) via an async semaphore.
  - **Recurring Failure Escalation**: Error fingerprints (SHA-256 hash of category + file + rule/test) are tracked in `CursorStore`. If the same error fingerprint recurs 3 times consecutively, it automatically escalates to human review.
  - **Full Observability**: Error classifications and remediation outcomes are emitted to Kafka topics `agent-thought` and `mission-artifact`.

---

## 🚀 Getting Started

### Prerequisites

- [Rust Toolchain](https://rustup.rs/) (two crates use edition 2024, which requires Rust 1.85 or newer; the workspace does not pin an exact patch release, so the tested minimum patch is **TO CONFIRM**)
- [Hatchet Server](https://github.com/hatchet-dev/hatchet)
- [MCP Connection](https://modelcontextprotocol.io/)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/lgcorzo/rust_CACD_autonomous_factory
   cd rust_CACD_autonomous_factory
   ```

2. Build the workspace:

   ```bash
   cargo build --release
   ```

### Running the Factory Worker

Set `KAFKA_BROKERS` to the broker endpoint for your environment. The repository does not establish an active broker provider or endpoint. The CLI and example default to `localhost:9092`; the MCP server has a separate default:
```env
KAFKA_BROKERS=localhost:9092
```

Start the unified worker to begin processing autonomous missions:

```bash
# Provide your MCP gateway URL
cargo run -p factory-cli -- worker --mcp-url http://localhost:8100
```

### Agent Tooling: Graphify & Code Review

**Graphify** turns the repository into a navigable knowledge graph, enabling AI agents and developers to quickly understand the project architecture and file relationships.
**Code-Review-Graph** works alongside Graphify to provide incremental, graph-based code reviews and semantic search capabilities.

#### 1. Install & Register Tools

Install the CLIs and register them as skills with your autonomous agent (e.g., Antigravity):

```bash
# Install tools
uv tool install graphifyy
uv tool install code-review-graph

# Register with Antigravity
graphify install
code-review-graph install -y --platform antigravity
```

#### 2. Configuration & API Keys

For secure embeddings and semantic extraction, both tools route through OpenZiti to reach internal LiteLLM endpoints. Ensure your `.env` file is configured correctly:

```env
# Graphify / Code Review Embeddings Config (LiteLLM / OpenAI Gateway)
OPENAI_BASE_URL=https://<your-litellm-or-openai-gateway>/v1
OPENAI_API_KEY=<insert-your-actual-api-key>
OPENAI_MODEL=internal-gpt4_v0.1

# Fallback for graphify extraction if not using LiteLLM:
# GEMINI_API_KEY=<insert-your-gemini-api-key>
# GOOGLE_API_KEY=<insert-your-google-api-key>

CRG_OPENAI_BASE_URL=https://<your-litellm-or-openai-gateway>/v1
CRG_OPENAI_API_KEY=<insert-your-actual-api-key>
CRG_OPENAI_MODEL=lite_embedding
```

#### 3. Common Usage & Workflow

Keep the graphs up-to-date and leverage semantic search during development:

- **Update Graphify**: Run after major code changes to rebuild the knowledge graph (AST extraction is fast and free).
  ```bash
  graphify update .
  ```
- **Query the Knowledge Graph**: Explore architecture or concepts.
  ```bash
  graphify query "What does this project do?"
  ```
- **Update Code Review Graph**: Incrementally index file changes.
  ```bash
  code-review-graph update
  ```
- **Enable Semantic Search**: Download embedding models and index new nodes for search.
  ```bash
  code-review-graph embed
  ```
- **Code Review & Impact Analysis**: Use semantic features to detect changes and their blast radius.
  ```bash
  code-review-graph detect-changes
  ```

---

## 🛠 Development Workflow

We enforce high source quality through automated CI pipelines.

### Standard Commands

- **Check Style**: `cargo fmt --all -- --check`
- **Static Analysis**: `cargo clippy --workspace -- -D warnings`
- **Run Tests**: `cargo test --workspace`

## ✅ Repository Checks

The following automated checks are active in this repository to maintain high engineering standards:

### 1. CI/CD Pipeline (`pipeline.yml`)

- **Linting**: Enforces strict `rustfmt` and `clippy` (warnings-as-errors) across the entire workspace.
- **Testing**: Runs `cargo test --workspace -- --skip smoke` and the evidence-contract test target.
- **Dependency audit**: Runs `cargo audit -D warnings` when the workflow is triggered by a push to `main` or `feature/**`, a pull request to `main`, manual dispatch, or a published/edited release.
- **Results**: The workflow configuration does not establish a zero-vulnerability result.
- **Protoc Build**: Automated Protocol Buffers compilation verification for all service definitions.
- **Docker Build**: Validates that a production-ready image can be built and pushed to Docker Hub upon merge to `main`.

### 2. Wiki Content Sync (`docs-to-wiki.yml`)

- Maintains bi-directional synchronization between the repo's `wiki/` folder and the GitHub Project Wiki.
- Ensures documentation remains consistent and discoverable.

### 3. Image Integrity & Deployment

- Production images are published under `lgcorzo/dark-gravity-factory`.
- Every release candidate is tagged with the specific `git SHA` for full auditability and rollback capability.

### 4. Code Standards

- **Edition 2024 Readiness**: The project uses modern Rust editions for safety and performance.
- **Dependency Guard**: Automated monitoring of critical libraries (Hatchet, OpenZiti, the AWS SDK).

---

## 📜 Project Documentation

For a deep-dive into the factory's mechanics, follow the documentation roadmap:

1. **[Strategic Design](wiki/architecture/strategic_design.md)** — Bounded contexts, Onion Architecture layers, and Zero Trust ZTA patterns.
2. **[Execution Flows & Lifecycle](wiki/operations/experiment_logs.md)** — Hatchet 6-phase DAG logic, step checkpointing, and real-time Kafka telemetry.
3. **[Verification Triad](wiki/security/verification_triad.md)** — Strict testing strategy (logic, architecture, and security) with sandboxed execution.
4. **[Infrastructure Adapters](wiki/architecture/infrastructure_adapters.md)** — Production connectors for Jira, R2R Graph RAG, and live rdkafka Kafka adapters.
5. **[Production Operations](wiki/operations/production_operations.md)** — Operational scaling, Kubernetes, KEDA, OpenZiti overlays, and secure memory management.

Access the full **[Documentation Index](wiki/README.md)** for business context and mission history.

---

*Dark Gravity - Engineering the Future of Autonomous Systems.*
