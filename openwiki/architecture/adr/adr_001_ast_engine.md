---
iso_doc_type: "Description"
iso_viewpoint: "ArchitectureDecision"
type: "adr"
title: "ADR 001: Local AST Engine Over External LLM Vector Databases"
description: "Decision record documenting the adoption of local Graphify/Pyreverse AST parsing over external vector embedding services."
tags: ["adr", "iso42010", "decision", "ast", "graphify"]
timestamp: "2026-07-31T16:35:00Z"
---

# Architecture Decision Record (ADR 001): Local AST Parsing Engine

## 1. Status
**ACCEPTED** (Date: 2026-07-31)

---

## 2. Context & Stakeholder Concern

* **Addressed Concern**: Avoid reliance on external vector database clusters, third-party embedding APIs, or multi-service container orchestration for code understanding. Maintain 100% deterministic code understanding without LLM hallucination risks on function contracts or line spans.
* **Framing Viewpoints**: Component View, Security View, and ISO 25010 Maintainability.

---

## 3. Decision

Adopt local, executable AST tools (`graphify update .`, `pyreverse`, Python `ast`, and native Rust parser scripts) as the primary knowledge extraction engine for generating OpenWiki software architecture documentation.

---

## 4. Rationale & Alternatives Evaluated

| Alternative Evaluated | Trade-Off / Failure Mode | Decision Result |
| :--- | :--- | :--- |
| **External Vector Embedding Server** | High latency, multi-service setup complexity, risk of missing exact class hierarchies (`<|--`). | Rejected |
| **LLM-Only Code Scanning** | Potential hallucination on line spans (`:L#`) and method parameters. | Rejected |
| **Local AST Scripts + Primary LLM** | 100% accurate line range citations, zero API costs for AST indexing, deterministic graph. | **SELECTED** |

---

## 5. System Impact & Traceability

- **Execution Script**: `generate_openwiki.py` (`generate_openwiki.py:L1-L197`)
- **Knowledge Graph**: `graphify-out/graph.json` containing AST nodes and Leiden community clusters.
- **Governed Architecture**: [[Architecture/ComponentStructure]]
