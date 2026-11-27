---
manifest_version: "1.0"
platform: "Dark Gravity"
repository: "rust_CACD_autonomous_factory"
tier: "core_factory"
visibility: "public"
canonical_hub: "https://lgcorzo.github.io/start-here"
auditor_pack: "https://lgcorzo.github.io/docs/auditor/"
mcp_server: "dark-gravity-mcp"
knowledge_graph: "https://lgcorzo.github.io/knowledge_graph.json"
---

# Dark Gravity Agent Manifest (`rust_CACD_autonomous_factory`)

## What Dark Gravity Is
Dark Gravity is a Kubernetes-native **Autonomous Software Factory (CA/CD)** built in Rust (Edition 2024) and reconciled declaratively via pull-based GitOps, Hatchet durable DAGs, Apache Kafka event buses, `gVisor` (`runsc`) sandboxes, Ed25519 W3C Verifiable Credentials with `54.2 ns` Vault JIT RAM zeroization, and hybrid R2R/Semantica GraphRAG memory.

## How This Repo Fits
- **Repository**: `rust_CACD_autonomous_factory` (`core_factory`)
- **Role**: Core 5-crate Rust 2024 CA/CD orchestrator (`factory-core`, `factory-application`, `factory-infrastructure`, `factory-mcp-server`, `factory-cli`), `AgentWorkspace` operator (`ADR-008`), and Hatchet workflow runtime.
- **Local Docs**: [AGENTS.md](./AGENTS.md) · [ARCHITECTURE.md](./ARCHITECTURE.md) · [REPOSITORIES.md](./REPOSITORIES.md)

## Global Architecture Spine
1. **WHAT (SDD Contract)**: `spec-kit` (`constitution.md` -> `spec.md` -> `plan.md` -> `tasks.md`).
2. **Bridge**: `speckit-superpowers-bridge` enforces boundary guards and `.specify/superpowers-handoff.json`.
3. **HOW (Execution)**: `RustantAgent` & `ZeroClawAgent` execute **5 Hatchet durable steps** (`rustant-plan`, `zeroclaw-execute`, `zeroclaw-validate`, `rustant-review`, `factory-deliver`) post-Kafka `mission-ingestion`.
4. **Governance**: `FR-008`/`CTRL-02` gVisor live exit remains `TESTED` until `AUD-010` closes; **Vertex 4 HITL** forbids autonomous merges to `main`; **no third-party compliance certification is claimed**.

## Active Research Questions
- `RQ1`: Can autonomous agents reduce MTTR?
- `RQ2`: Can verifiable credentials improve governance?
- `RQ3`: Can GitOps act as a security boundary?
- `RQ4`: Can agentic development outperform traditional development workflows?

## Machine-Readable Endpoints
- Portal & Quickstart: <https://lgcorzo.github.io/start-here> · <https://lgcorzo.github.io/ai-agents>
- Self-Contained AI Auditor Pack: <https://lgcorzo.github.io/docs/auditor/>
- Knowledge Graph: <https://lgcorzo.github.io/knowledge_graph.json>
- Permanent Metrics Snapshot: <https://lgcorzo.github.io/metrics.html>
- Read-Only Knowledge MCP: `dark-gravity-mcp` (distinct from privileged runtime `crates/factory-mcp-server/`)

## Do Not Misclassify
Do **not** evaluate `rust_CACD_autonomous_factory` in isolation from its Internal GitOps Control Plane, `speckit-superpowers-bridge`, sovereign AI/storage forks (`42` maintained forks across `64` public repos), and managed domain targets (`mlops-python-package`, `llmops-python-package`, `fastapi-autogen-team`).
