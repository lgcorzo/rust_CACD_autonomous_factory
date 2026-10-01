# Dark Gravity CA/CD Autonomous Factory — Wiki

[![docs-to-wiki](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/docs-to-wiki.yml/badge.svg)](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/docs-to-wiki.yml)

Welcome to the **Dark Gravity Autonomous Factory** professional enterprise wiki. This documentation provides a comprehensive, C4-model-driven view of the CA/CD (Continuous Agentic / Continuous Deployment) platform — from executive business context through to deep tactical code-level UML diagrams.

---

## 🏠 Quick Navigation

| Section | Description |
|:---|:---|
| [📋 Business Context](BUSINESS-CONTEXT.md) | CA/CD vision, Hazitek 2026, ROI, KPIs |
| [🏗️ Strategic Design](STRATEGIC-DESIGN.md) | C4 Context & Container diagrams, Bounded Contexts, Onion Architecture |
| [⚙️ Tactical Design](TACTICAL-DESIGN.md) | C4 Component diagrams, 5-crate mapping, DAG phases |
| [🤖 Agent Specifications](AGENT-SPECIFICATIONS.md) | Rustant, ZeroClaw, Auditor, FinOps, QAObserver, DocAgent |
| [🔄 Experiment Lifecycle](EXPERIMENT-LIFECYCLE.md) | 6-phase Hatchet DAG, Spec-Kit SDD workflows |
| [🔌 Infrastructure Adapters](INFRASTRUCTURE-ADAPTERS.md) | Kafka, R2R GraphRAG, OpenZiti, Vault, Sentry |
| [✅ Verification Triad](VERIFICATION-TRIAD.md) | Logical, Architectural, Security gates |
| [🚀 Production Operations](PRODUCTION-OPERATIONS.md) | FluxCD GitOps, K8s topology, LiteLLM, troubleshooting |
| [📖 User Manual](USER-MANUAL.md) | CLI usage, environment config, interactive directives |
| [🔒 Security Architecture](SECURITY-ARCHITECTURE.md) | Zero Trust, Ed25519 NHI, gVisor sandbox isolation |
| [🏛️ HITL Governance](HITL-GOVERNANCE.md) | 4-Vertex governance mesh, human oversight model |
| [📜 Compliance & Audit](COMPLIANCE-AUDIT.md) | Hazitek 2026, EU AI Act, R&D telemetry packaging |
| [📚 Glossary](GLOSSARY.md) | DDD ubiquitous language, acronyms, domain terms |
| [📑 Master Index](index.md) | Complete file listing with categories |

---

## 🔧 Crate Reference (OKF Source Maps)

Each crate in the workspace has per-module OKF documentation pages:

| Crate | Layer | Pages |
|:---|:---|:---:|
| [`factory-core`](crates_factory-core_src_lib.md) | Domain | 5 |
| [`factory-application`](crates_factory-application_src_lib.md) | Application | 20+ |
| [`factory-infrastructure`](crates_factory-infrastructure_src_lib.md) | Infrastructure | 15+ |
| [`factory-mcp-server`](crates_factory-mcp-server_src_lib.md) | Interface (MCP) | 20+ |
| [`factory-cli`](crates_factory-cli_src_main.md) | CLI | 3 |

---

## 📊 Architecture at a Glance

```mermaid
C4Context
    title Dark Gravity CA/CD Autonomous Factory — System Context (C1)

    Person(po, "Product Owner", "Creates Epics tagged autonomous-plan")
    Person(techlead, "Tech Lead", "Approves Spec-Kit task decomposition")
    Person(architect, "Architect", "Resolves Agent-Stuck deadlocks")
    Person(reviewer, "Senior Reviewer", "Reviews design & merges PR/MR")

    System(factory, "Dark Gravity Factory", "Autonomous CA/CD platform")

    System_Ext(github, "GitHub", "Source code, PRs, Actions CI")
    System_Ext(gitlab, "GitLab", "MRs, CI pipelines")
    System_Ext(jira, "Jira", "Issue tracking, Sprint boards")
    System_Ext(fluxcd, "FluxCD", "GitOps Kubernetes deployment")
    System_Ext(hatchet, "Hatchet", "Workflow DAG orchestration")
    System_Ext(kafka, "Kafka KRaft", "Event streaming bus")
    System_Ext(litellm, "LiteLLM", "Multi-provider LLM routing")
    System_Ext(ziti, "OpenZiti", "Zero Trust overlay network")

    Rel(po, factory, "Creates autonomous missions")
    Rel(techlead, factory, "Approves task plans")
    Rel(architect, factory, "Overrides stuck agents")
    Rel(reviewer, factory, "Merges PRs (HITL gate)")

    Rel(factory, github, "Polls issues/PRs, pushes code")
    Rel(factory, gitlab, "Polls MRs, pushes fixes")
    Rel(factory, jira, "Searches issues, syncs tasks")
    Rel(factory, hatchet, "Dispatches 6-phase DAGs")
    Rel(factory, kafka, "Publishes/consumes mission events")
    Rel(factory, litellm, "Routes LLM inference requests")
    Rel(factory, ziti, "Secure agent-to-service tunnels")
    Rel(fluxcd, factory, "Deploys factory to K8s")
```

---

## 📄 Documentation Standards

- **Diagrams**: All architectural diagrams use [Mermaid.js](https://mermaid.js.org/) for portable, text-based rendering
- **Format**: Follows the [Open Knowledge Format (OKF)](https://github.com/openwiki-spec) / OpenWiki standard
- **Quality Gate**: Orphan Symbol Rate (OSR) < 5% — every public Rust AST symbol is documented
- **Sync**: Documentation auto-syncs via `.github/workflows/docs-to-wiki.yml`

---

> *Generated for the Dark Gravity CA/CD Autonomous Factory — Professional Enterprise Wiki*
> *Branch: `007-professional-wiki-docs` | Last updated: 2026-09-30*