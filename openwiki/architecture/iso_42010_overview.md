---
iso_doc_type: "Description"
iso_viewpoint: "ArchitectureDescription"
type: "architecture"
title: "ISO/IEC/IEEE 42010 Architecture Description Overview"
description: "Master architecture description artifact defining Entity of Interest, stakeholders, viewpoints, and system views."
tags: ["iso42010", "architecture", "overview", "okf"]
timestamp: "2026-07-31T16:35:00Z"
---

# ISO/IEC/IEEE 42010 Architecture Description: `rust_CACD_autonomous_factory`

## 1. Entity of Interest (EoI) & System Identification

* **System Name:** `rust_CACD_autonomous_factory` (Dark Gravity Autonomous Factory Engine)
* **Target Environment:** Linux x86_64 / Rust 1.75+ / Tokio Async Runtime / gVisor Sandboxes
* **Primary Repository:** `.` (Anchored to repository root)
* **Crate Workspace:** `Cargo.toml` defining 5 core crates:
  - `crates/factory-core` (Core domain abstractions, security models, executor traits)
  - `crates/factory-infrastructure` (External service clients: Aethalgard, Kafka, OpenZiti, S3, Vault)
  - `crates/factory-application` (Agent roles, ADK bridge state drivers, mission workflows)
  - `crates/factory-mcp-server` (Model Context Protocol server with gVisor sandbox drivers)
  - `crates/factory-cli` (CLI binary entry points for triggering missions)

---

## 2. Stakeholder Perspectives & Concerns Matrix

| Stakeholder Persona | Primary Concerns | Framing ISO Viewpoint | Governed Wiki Page |
| :--- | :--- | :--- | :--- |
| **Lead System Architect** | Modular decomposition, zero-trust boundaries, crate dependencies | Component View | [[Architecture/ComponentStructure]] |
| **Chief Information Security Officer** | Cryptographic token validation, zeroize secret destruction, gVisor pod isolation | Security View | [[Architecture/SecurityView]] |
| **Autonomous Agent Developer** | Code surgery patch contracts, AST graph query performance, execution flows | Sequence View | [[Architecture/RuntimeSequences]] |
| **Site Reliability / DevOps Lead** | Multi-backend sandbox execution, Docker containerization, gVisor K8s integration | Deployment View | [[Architecture/DeploymentView]] |
| **Quality & Audit Compliance Officer** | ISO 25010 SQuaRE compliance, traceability of code changes to requirements | SRS & Quality View | [[Quality/ISO25010Quality]] |

---

## 3. Viewpoints Framework & Architectural Navigation

The architecture description is governed by five specialized ISO 42010 viewpoints:

1. 🌐 [[Architecture/SystemContext|Context View]]: Defines the system boundary, external actors (GitLab, Jira, Kafka, Vault, OpenZiti, MCP Clients), and protocol boundaries.
2. 📦 [[Architecture/ComponentStructure|Component View]]: Maps the workspace crate layout, inter-crate dependencies, trait interfaces, and UML 2.0 Class Diagrams.
3. 🔄 [[Architecture/RuntimeSequences|Sequence View]]: Documents dynamic interaction flows for code surgery execution, sandbox isolation, and remediation webhooks.
4. 🐳 [[Architecture/DeploymentView|Deployment View]]: Outlines physical and virtual infrastructure, gVisor sandboxed pod manifests, and Cargo compilation targets.
5. 🔐 [[Architecture/SecurityView|Security View]]: Specifies zero-trust security bounds, Ed25519 signature verification, `ZeroizeOnDrop` JIT token safety, and Aethalgard automated remediation.

---

## 4. Architecture Decision Records Index

- 📝 [[Architecture/ADR/ADR_001_AST_Engine|ADR 001: Local AST Engine]]: Documents the selection of local, AST-driven deterministic parsing (`graphify`, `pyreverse`, Python `ast`) over external embedding databases.
