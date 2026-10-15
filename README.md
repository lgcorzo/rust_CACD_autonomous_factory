# Dark Gravity: CACD Autonomous Agent Factory

[![CI/CD Pipeline](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/pipeline.yml/badge.svg)](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/pipeline.yml)
[![Wiki Content Sync](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/docs-to-wiki.yml/badge.svg)](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/docs-to-wiki.yml)
[![Screencast Walkthrough](https://img.shields.io/badge/Live_Demo-1080p_Video-blueviolet.svg)](docs/screencast_walkthrough.md)
[![Docker Pulls](https://img.shields.io/docker/pulls/lgcorzo/dark-gravity-factory.svg)](https://hub.docker.com/r/lgcorzo/dark-gravity-factory)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/lgcorzo/rust_CACD_autonomous_factory)

**Dark Gravity** is a high-performance, long-term agentic system serving as the core intelligence for the **CA/CD Autonomous Agent Factory**. Rebuilt from the ground up in **Rust**, it orchestrates complex multi-agent workflows with zero-trust security and durable task execution.

> 🎬 **[Watch the 5-Minute Live Factory Execution Screencast Walkthrough](docs/screencast_walkthrough.md)** (`docs/assets/screencast_dark_gravity_walkthrough.mp4`) — Demonstrating outbound issue polling, Hatchet 6-phase DAG orchestration, gVisor container sandboxing (500m CPU / 512Mi RAM), interactive PR directives (`@darkgravity /status`, `@darkgravity /refine`), and mandatory Human-in-the-Loop (HITL) merge blocking.

---

## 🏗 Standards & Architecture

This project strictly adheres to **LLMOps** and **Domain-Driven Design (DDD)** standards, elevated to the production-grade **Version 7 (V7) Architecture** for autonomous operations.

- **LLMOps Lifecycle**: We follow a robust LLM application lifecycle, including automated prompt engineering, retrieval-augmented generation (RAG) evaluation, and model-in-the-loop security audits to ensure production-grade reliability and observability.
- **DDD-based Structure**: The codebase is architected using DDD principles to maintain a clear separation of concerns. This ensures that the core domain logic remains isolated from infrastructure details and external service adapters (like Jira or R2R), allowing the system to scale and evolve without regression.

The codebase is organized as a unified Rust workspace for maximum modularity and performance.

### 1. Workspace Crate Map

- **`factory-core`**: Innermost domain layer. Manages domain entities (`Mission`, `Task`), a `SecurityValidator` trait for agent output auditing, and unified error handling via `FactoryError`.
- **`factory-application`**: Workflow orchestration layer. Leverages **Rustant** (Planner) and **ZeroClaw** (Executor) to process task workflows. Maintains durably checkpointed state via `StepCheckpoint`s using Hatchet's PostgreSQL backend.
- **`factory-mcp-server`**: Presentation layer. Axum-based Model Context Protocol server exposing tools over Server-Sent Events (SSE) transport with isolated Firecracker execution interfaces.
- **`factory-infrastructure`**: Adapter layer. Concrete clients for Jira, R2R (RAG), MinIO/AWS S3, OpenZiti (Zero Trust overlay networks), and Confluent Cloud Kafka (via `rdkafka` client).
- **`factory-cli`**: Interface entry point. Contains the Hatchet worker initializer and Kafka telemetry querying utilities.

### 2. Durable Orchestration (Phase 12 State-Loop)

Missions are orchestrated using a robust **6-phase DAG** in **Hatchet** with database-backed `StepCheckpoint` recovery:
`Ingestion` → `Plan (Rustant)` → `Code (ZeroClaw)` → `Validation (ZeroClaw)` → `Review (Rustant)` → `Delivery (GitOps)`.

### 3. Integrated Target Repositories

Dark Gravity continuously polls and autonomously executes missions across configured repositories:

| Target Repository | Domain | Autonomous Workflow Focus |
|:---|:---|:---|
| [`lgcorzo/rust_CACD_autonomous_factory`](https://github.com/lgcorzo/rust_CACD_autonomous_factory) | Autonomous Multi-Agent Engine | Self-evolution, DAG workflows & MCP servers |
| [`lgcorzo/lince-rs`](https://gitlab.com/lgcorzo/lince-rs) | Otimization process | Algorithmic optimization to control agents costs |
| [`lgcorzo-lab/autonomous_factory`](https://gitlab.com/lgcorzo-lab/autonomous_factory) | Architecture & Knowledge Base | Spec-Kit SDD contracts, C4 diagrams, OSR audits |
| [`lgcorzo/fastapi-autogen-team`](https://github.com/lgcorzo/fastapi-autogen-team) | Agentic Access Layer | R2R/Confluent document services |
| [`lgcorzo/mlops-python-package`](https://github.com/lgcorzo/mlops-python-package) | MLOps & ML Pipelines | Automated testing, DVC/Poetry dependency validation & CI/CD error remediation |

### 4. Integrated Intelligence & Security

- **Rustant**: Leverages **R2rClient** for semantic code search and context retrieval to inform specification design and planning.
- **ZeroClaw**: Compiles and executes code in secure **Firecracker micro-VMs** via a sandboxed executor driver.
- **Zero Trust Mesh**: Agent-to-agent communication is fully encrypted and bound to identity namespaces using **OpenZiti mTLS 1.3** overlays.
- **Telemetry**: Reasoning logs are published to **Confluent Cloud Kafka** topics (`mission-input`, `agent-thought`, `mission-artifact`) for real-time observability.

---

## 🛡️ Technological Sovereignty, Zero-Trust Security & Sovereign Dependency Policy

The **Dark Gravity Autonomous CA/CD Software Factory (V7.2 / V7.3)** is built under strict compliance with the **EU AI Act**, **Hazitek / SPRI**, and **SOC 2 Type II** standards, requiring absolute architectural and supply-chain sovereignty.

*(For detailed architectural specifications and design records, refer to the [Dark Gravity Multi-Agent Architecture Knowledge Base](https://notebook.google.com/notebook/116d5f03-2fb0-4fa7-bd7c-1c9849920abc))*.

### 1. The Technological Sovereignty Imperative (Why Sovereign Forks?)
In an autonomous software factory where AI agents autonomously synthesize, compile, and deliver production software, relying blindly on third-party commercial cloud services or unmaintained open-source dependencies poses existential operational risks:
- **Mitigation of Upstream Licensing Closures:** When foundational projects alter their licensing model, close public distributions, or abandon open-source governance (as occurred with MinIO), Dark Gravity prevents vendor lock-in and supply-chain disruption by maintaining **semantic mirrors and sovereign forks** under `@lgcorzo`.
- **Autonomous Security Verification & Zero-CVE Policy:** Third-party libraries are not trusted implicitly. Dark Gravity continuously scans, audits, and patches its sovereign dependency tree independently (enforcing 0 leaked credentials and zero high-severity CVEs) rather than awaiting upstream commercial release cycles.
- **Air-Gapped Operational Continuity:** The factory operates on private, sovereign Kubernetes infrastructure. Proprietary source code, AST surgical mutations, and causal corporate memory remain strictly within sovereign boundaries.

### 2. Core Sovereignty & Security Architecture
- **Zero Public Ingress Ports (Air-Gapped Outbound Polling):** Dark Gravity does not expose public HTTP webhooks or third-party ingress tunnels. The native Rust `GitPlatformPoller` daemon (`factory-cli poller`) executes outbound polling loops every 30–60 seconds across GitHub and GitLab APIs.
- **Non-Human Identity (NHI) & Cryptographic Proof-of-Lineage:** Every agent action, commit, and Kafka event is cryptographically signed using **W3C Verifiable Credentials** (`Ed25519Signature2020` / JWS) via `ed25519-dalek` v2.
- **Ephemeral JIT Credentials & Forensic RAM Sanitization:** Repository tokens are provisioned Just-In-Time by HashiCorp Vault with an unrenewable 5-minute (300s) TTL. All in-memory secrets implement `#[derive(Zeroize, ZeroizeOnDrop)]`, wiping memory with null bytes in $< 4.33\,\mu\text{s}$ (empirically $54.2\text{ ns}$) upon task completion.
- **Kernel-Level Sandboxing (gVisor `runsc`):** Code synthesis and test execution run in ephemeral containers strictly clamped to $\text{RAM} \le 30\text{ MiB}$ and $\text{CPU} \le 250\text{m}$.
- **Hardware-Accelerated SIMD Primitives:** Sovereign Go/Rust cryptographic and parsing libraries (`sha256-simd`, `simdjson-go`, `highwayhash`, `md5-simd`, `blake2b-simd`) leverage AVX-512 and ARM64 vectorization, enabling high-throughput execution without exceeding hard memory ceilings.
- **Sovereign S3 Storage Tier & Isolated Buckets:** Artifacts are persisted in decoupled S3 storage (`factory-artifacts`, `factory-artifacts/compliance/`, `dg-factory-checkpoints`, `semantica-provenance`, `r2r-documents`, `mlflow`) protected by strict whitelist validation against path traversal.

---

## 🎮 Controlling Dark Gravity from GitLab & GitHub

Dark Gravity includes native outbound polling daemons and event handlers that listen for issues, work items, and pull/merge request discussions across configured GitHub and GitLab repositories.

### 1. Triggering Missions via Issues & Work Items

To trigger an autonomous mission, create an issue or work item in any integrated repository (such as `lgcorzo/rust_CACD_autonomous_factory` or `lgcorzo/lince-rs`):

- **Required Labels** (at least one):
  - `autonomous-mission`
  - `dark-gravity`
- **Resource Limits Definition**:
  Include an explicit resource limit block within the issue description. The poller automatically parses these constraints using regex:
  ```markdown
  Resource limits: CPU: 500m, RAM: 512Mi, Timeout: 300s
  ```
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
| `@darkgravity /validate` | Requests full test and security verification. | Executes automated test suites and SAST security gates within the sandbox, reporting CVE and pass/fail status. |
| `@darkgravity /retry` | Forces a restart of the execution loop. | Restarts the Aethelgard verification loop and re-runs failed test suites. |

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
- Once all automated CI/CD and CodeQL checks pass, the agent hands off the PR/MR link to the human developer for final manual review and merge.

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

- [Rust Toolchain](https://rustup.rs/) (Stable 1.75+)
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

Ensure your `.env` file contains the live Kafka broker endpoint:
```env
KAFKA_BROKERS=my-kafka-cluster-bootstrap.confluent.svc.cluster.local:9092
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
# Graphify / Code Review Embeddings Config (LiteLLM via Ziti)
OPENAI_BASE_URL=https://litellm.ziti/v1
OPENAI_API_KEY=<insert-your-actual-litellm-api-key>
OPENAI_MODEL=internal-gpt4_v0.1

# Fallback for graphify extraction if not using LiteLLM:
# GEMINI_API_KEY=<insert-your-gemini-api-key>
# GOOGLE_API_KEY=<insert-your-google-api-key>

CRG_OPENAI_BASE_URL=https://litellm.ziti/v1
CRG_OPENAI_API_KEY=<insert-your-actual-litellm-api-key>
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
- **Testing**: Executes all unit and integration tests (excluding heavy smoke tests).
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

1. **[Strategic Design](wiki/STRATEGIC-DESIGN.md)** — Bounded contexts, Onion Architecture layers, and Zero Trust ZTA patterns.
2. **[Execution Flows & Lifecycle](wiki/EXPERIMENT-LIFECYCLE.md)** — Hatchet 6-phase DAG logic, step checkpointing, and real-time Kafka telemetry.
3. **[Verification Triad](wiki/VERIFICATION-TRIAD.md)** — Strict testing strategy (logic, architecture, and security) with sandboxed execution.
4. **[Infrastructure Adapters](wiki/INFRASTRUCTURE-ADAPTERS.md)** — Production connectors for Jira, R2R Graph RAG, and live rdkafka Kafka adapters.
5. **[Production Operations](wiki/PRODUCTION-OPERATIONS.md)** — Operational scaling, Kubernetes, KEDA, OpenZiti overlays, and secure memory management.

Access the full **[Documentation Index](wiki/README.md)** for business context and mission history.

---

*Dark Gravity - Engineering the Future of Autonomous Systems.*
