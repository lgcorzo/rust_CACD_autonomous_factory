# Phase 0 Research: Professional Enterprise Wiki Documentation

**Feature Branch**: `007-professional-wiki-docs`  
**Date**: 2026-09-30  
**Feature Spec**: [spec.md](spec.md)  

---

## 1. Architectural & Technical Decisions

### Decision 1: C4 Model & Diagram Standard (Levels 1–4)
- **Decision**: Adopt the standard C4 Model (Context, Container, Component, Code) natively authored using Mermaid.js fenced code blocks (`mermaid`).
- **Rationale**:
  - Eliminates external image rendering dependencies and binary asset drift.
  - Mermaid renders natively in GitHub, GitLab, and local IDE markdown previewers.
  - C1 (Context) captures human actors (PO, Tech Lead, Architect, Reviewer) and external infrastructure (GitHub, GitLab, Jira, FluxCD, Kafka, Ziti, Hatchet).
  - C2 (Container) captures the 5 physical Rust crates and Kubernetes namespaces (`agents`, `orchestrators`, `llm-apps`, `confluent`).
  - C3 (Component) details the internal agent modules (`Rustant`, `ZeroClaw`, etc.) and MCP tools (`launch_sandbox_pod`, `run_tests`, `spec_kit_tool`).
  - C4 (Code) details UML 2.0 class/trait diagrams and state machines.
- **Alternatives Considered**:
  - *PlantUML / Graphviz*: Requires external server/Java rendering pipeline, leading to broken images in air-gapped or offline viewers.
  - *Static PNG/SVG*: Unsearchable, unmaintainable, prone to documentary amnesia during code refactors.

---

### Decision 2: Information Architecture & Canonical Document Hierarchy
- **Decision**: Implement the canonical Dark Gravity wiki hierarchy established in NotebookLM (`07_Wiki_Structure_Plan.md`), structured as a hybrid top-down governance suite complemented by modular OKF per-crate references:
  1. `Home.md` / `README.md` / `_Sidebar.md` (Global navigation hub).
  2. `GLOSSARY.md` (Ubiquitous Language, DDD terms, acronyms).
  3. `BUSINESS-CONTEXT.md` (CA/CD vision, Hazitek 2026 goals, ROI, KPI targets).
  4. `STRATEGIC-DESIGN.md` (Bounded Contexts, Onion Architecture, Subsystem boundaries).
  5. `TACTICAL-DESIGN.md` (5-crate mapping, DAG phases, MCP tools).
  6. `AGENT-SPECIFICATIONS.md` (Rustant, ZeroClaw, FinOps, QAObserver, Auditor, DocAgent).
  7. `EXPERIMENT-LIFECYCLE.md` (6-phase Hatchet DAG, Spec-Kit SDD workflows).
  8. `INFRASTRUCTURE-ADAPTERS.md` (Kafka KRaft, R2R GraphRAG, OpenZiti, Vault).
  9. `VERIFICATION-TRIAD.md` (Logical, Architectural, Security gates).
  10. `PRODUCTION-OPERATIONS.md` (FluxCD GitOps, K8s topology, LiteLLM routing, troubleshooting).
  11. `00_Project_Charter.md` through `08_Implementation_Completion_Plan.md` (Agile ceremonies, ADR-001 to ADR-007, Roadmap, DoR/DoD).
  12. Per-Crate OKF Source Maps (`wiki/crates_factory-*.md`).
- **Rationale**: Provides immediate value for both executive stakeholders seeking strategic alignment and core developers requiring granular AST symbol documentation.
- **Alternatives Considered**:
  - *Flattened Single Directory without Structure*: Creates unnavigable clutter.
  - *Code-Only Docs (Rustdoc alone)*: Fails to capture Kubernetes topology, GitOps workflows, zero-trust security, or business ROI.

---

### Decision 3: Orphan Symbol Rate (OSR) Quality Gate (< 5%)
- **Decision**: Enforce a strict Orphan Symbol Rate threshold (**OSR < 5%**) verified through AST symbol extraction comparing compiled Rust public symbols against wiki markdown entity declarations.
- **Rationale**:
  - Prevents "documentary amnesia" and documentation drift.
  - Quantitative and objective quality gate rather than subjective approval.
  - Implements the verification triad before committing or syncing documentation.
- **Alternatives Considered**:
  - *Manual Spot-Checking*: Prone to human oversight and rapid staleness across hundreds of functions and structs.

---

### Decision 4: Human-in-the-Loop (HITL) 4-Vertex Governance Mesh
- **Decision**: Formally document the 4 zero-friction governance vertices:
  1. *Vertex 1: Strategic Injection* (Human PO creates Epics tagged `autonomous-plan`).
  2. *Vertex 2: Sprint Mobilization* (Human Tech Lead approves decomposed Spec-Kit tasks).
  3. *Vertex 3: Exception Override* (Human Architect resolves `Agent-Stuck` on 3-retry deadlocks).
  4. *Vertex 4: Categorical Scrutiny* (Human Senior Reviewer reviews design & merges PR/MR; agents strictly forbidden from merging).
- **Rationale**: Aligns autonomous agent acceleration with rigorous enterprise compliance, risk management, and ISO/IEC 25059 AI governance standards.
- **Alternatives Considered**:
  - *Unsupervised Autonomous Merge*: Violates security posture and repository rules.

---

### Decision 5: R&D Compliance & Audit Trail Packaging
- **Decision**: Define the automated telemetry audit packager for European/regional R&D grants (Hazitek 2026, SPRI) and EU AI Act Art. 12 & 14 compliance:
  - AST code deltas and diff metrics.
  - Sandbox compute core-hours (gVisor CPU/RAM).
  - FinOps token consumption logs via virtual tags (`FinOpsTag`).
  - Ed25519 cryptographic Non-Human Identity (NHI) claims.
- **Rationale**: Guarantees auditability for government grants and compliance bodies without manual engineering timesheets.

---

## 2. Research Validation Summary

All technical decisions are grounded in the active codebase in `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory` and verified against the curated research notebook `116d5f03-2fb0-4fa7-bd7c-1c9849920abc`.
No unresolved clarification markers remain. Ready for Phase 1 Design.
