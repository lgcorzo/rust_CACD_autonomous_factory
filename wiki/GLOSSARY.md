# GLOSSARY: Ubiquitous Language

This document defines the common terminology used throughout the **Dark Gravity** ecosystem.

---

## Core Entities

| Term | Definition | DDD Context |
| :--- | :--- | :--- |
| **Mission** | A high-level goal or problem statement the factory must resolve. | **Aggregate Root** |
| **Task** | A unit of work within a mission, assigned to an agent. | **Entity** |
| **Phase** | A distinct stage in the Hatchet execution DAG (6 phases). | **Domain Event** |
| **Agent** | An autonomous entity with specific roles and tools (Rustant, ZeroClaw, DevOps, Documentation). | **Domain Service** |
| **Thought** | A structured reasoning artifact produced by an agent before taking an action. | **Value Object** |
| **Artifact** | Any tangible output produced during a mission (spec.md, code, tests, PRs). | **Entity** |

## Architecture Terms

| Term | Definition |
| :--- | :--- |
| **Spec-Kit** | GitHub's Spec-Driven Development toolkit; mandatory planning protocol for the PO Agent. |
| **Superpowers** | Agentic skill framework for the Documentation Agent; provides `writing-plans`, `subagent-driven-development`, `verification-before-completion`, etc. |
| **superspec** | Orchestrator bridging Spec-Kit (planning) with Superpowers (execution) via state machine and checkpointing. |
| **MCP** | Model Context Protocol — JSON-RPC over SSE/HTTP for agent-tool communication. |
| **Hatchet Engine** | Durable workflow engine; orchestrates the 6-phase DAG with PostgreSQL-backed state persistence. |
| **Onion Architecture** | The layered architecture pattern organizing the 5-crate Rust workspace. |

## Infrastructure Terms

| Term | Definition |
| :--- | :--- |
| **Sandbox** | Isolated execution environment — gVisor (K8s runtime class) or Firecracker (KVM micro-VM). |
| **gVisor** | Application-level kernel sandbox (`runsc` runtime class); RAM ≤ 30Mi, CPU ≤ 250m. |
| **Firecracker** | Hardware-virtualized micro-VM via KVM; uses `AF_VSOCK` for host-guest communication. |
| **OpenZiti** | Zero Trust networking overlay; mTLS 1.3 tunnels with zero public ports. |
| **NHI** | Non-Human Identity; Ed25519 Verifiable Credentials for every agent action. |
| **R2R GraphRAG** | Graph-based Retrieval-Augmented Generation; backed by pgvector for semantic codebase memory. |
| **deepwiki-rs** | Native AST parser (Tree-sitter) that extracts code deltas and syncs embeddings to pgvector. |
| **Aethelgard Loop** | CI/CD auto-remediation loop; max 3 retries before `Agent-Stuck` escalation. |

## Verification Terms

| Term | Definition |
| :--- | :--- |
| **Verification Triad** | Three-pillar validation: Logical (tests), Architectural (Spec-Kit + linters), Security (SAST + LLM-as-a-Judge). |
| **OSR** | Orphan Symbol Rate; documentation quality metric — must stay < 5%. |
| **LLM-as-a-Judge** | Security-tuned LLM analyzing code diffs; minimum score 8.0/10 required. |

## Governance Terms

| Term | Definition |
| :--- | :--- |
| **Vtags** | Virtual Tags for per-Epic LLM cost attribution (StackSpend/Finout). |
| **Closed-Loop QA** | Sentry error → severity grade → GraphRAG mapping → backlog issue. |
| **Hazitek/SPRI** | European R&D grant schemas auto-compiled by the Documentation Agent. |
