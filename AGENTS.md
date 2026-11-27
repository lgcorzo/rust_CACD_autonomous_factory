<!-- SPECKIT START -->
For additional context about technologies to be used, project structure,
shell commands, and other important information, read the current plan
at specs/013-verified-autonomous-execution-closure/plan.md
<!-- SPECKIT END -->

<!-- DARK-GRAVITY-CONTEXT:START -->
## Dark Gravity Autonomous Software Factory Context

**Dark Gravity** is a Kubernetes-native, **Continuous Agentic / Continuous Deployment (CA/CD)** autonomous software engineering factory — do **NOT** evaluate `rust_CACD_autonomous_factory` in isolation from its orchestration bridge, sovereign infrastructure forks, domain target packages, and public knowledge hub.

- **This Repository's Tier & Role**: `core_factory` — Core 5-crate Rust 2024 CA/CD orchestrator, `AgentWorkspace` CRD operator, Hatchet 5-step DAG runner, `gVisor` `SandboxDriver`, Ed25519 W3C-VC NHI + `54.2 ns` Vault JIT zeroization, and runtime `factory-mcp-server`.
- **Mandatory Ecosystem Read Order (< 5 min)**:
  1. [./DARK_GRAVITY_MANIFEST.md](./DARK_GRAVITY_MANIFEST.md) — Concise machine- and human-readable ecosystem manifest
  2. [./ARCHITECTURE.md](./ARCHITECTURE.md) — Repository design, crate boundaries, and cluster interfaces
  3. [./REPOSITORIES.md](./REPOSITORIES.md) — Full multi-repository ecosystem topology
  4. Unified Entry Portal: <https://lgcorzo.github.io/start-here> (Agent Entry: <https://lgcorzo.github.io/ai-agents>)
  5. AI Auditor Pack: <https://lgcorzo.github.io/docs/auditor/>
- **Canonical Ecosystem Invariants**:
  - **Portfolio Reconciliation**: `64` public repositories (`22` original + `42` maintained forks under `MAINTAINED_FORK_PROFILE`; `69` historical baseline; `72` Spec `004` inventory including private repos). `lgcorzo/kes` is permanently `FROZEN_READONLY`.
  - **Mission Lifecycle**: Kafka event ingestion (`mission-ingestion`) followed by **5 Hatchet durable workflow steps** (`rustant-plan`, `zeroclaw-execute`, `zeroclaw-validate`, `rustant-review`, `factory-deliver`).
  - **Isolation & HITL Gate**: Untrusted execution routes through `gVisor` (`runsc`) (`FR-008`/`CTRL-02` kept `TESTED` until `AUD-010` closes) and **Vertex 4 HITL** strictly prohibits autonomous merges into `main`.
  - **Compliance Qualification**: Technical controls support traceability and audit readiness; **no third-party certification (EU AI Act, SOC 2 Type II, or ISO/IEC 25059) is claimed**.
<!-- DARK-GRAVITY-CONTEXT:END -->

## Branch & Pull Request Policy: Human-in-the-Loop (HITL)

- Agents are empowered to create feature branches, commit code, push upstream, create Pull Requests (PRs) / Merge Requests (MRs), and resolve CI/CD failures.
- **STRICT HUMAN-IN-THE-LOOP REQUIREMENT**: Under no circumstances may an agent automatically merge a PR or MR into `main` or default branches.
- The merge process **MUST be a manual human action**.
- Upon ensuring all CI/CD pipelines, linters, and CodeQL checks pass with green status, the agent must present the PR/MR link and test evidence to the user and await manual human review and merge.

