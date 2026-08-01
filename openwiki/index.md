---
iso_doc_type: "Description"
iso_viewpoint: "ArchitectureDescription"
type: "hub"
title: "OpenWiki Master Knowledge Hub: rust_CACD_autonomous_factory"
description: "Central navigation hub and ISO 15289 Description artifact for the rust_CACD_autonomous_factory project."
tags: ["index", "iso15289", "openwiki", "okf", "rust_cacd"]
timestamp: "2026-07-31T16:35:00Z"
---

# 🌐 OpenWiki Master Knowledge Hub: `rust_CACD_autonomous_factory`

Welcome to the ISO-compliant **DeepWiki / CodeWiki Architecture Documentation** for the `rust_CACD_autonomous_factory` ecosystem. This workspace represents a high-assurance, zero-trust autonomous software factory built in Rust, providing autonomous agent orchestration, AST-driven code surgery, gVisor sandboxed code execution, and Model Context Protocol (MCP) server capabilities.

This documentation suite is generated strictly under international systems and software engineering standards (**ISO/IEC/IEEE 42010**, **ISO/IEC/IEEE 15289**, **ISO/IEC 25010**, and **ISO/IEC/IEEE 26514**) and formatted using the **Open Knowledge Format (OKF)** standard.

---

## 🏛️ ISO/IEC/IEEE 42010 Architecture Description Framework

The architecture description is organized into standardized architectural viewpoints:

- 📋 [[Architecture/Overview|ISO 42010 Architecture Overview]] — Master Architecture Description (AD) framework, Entity of Interest (EoI), and Stakeholder Concerns matrix.
- 🌐 [[Architecture/SystemContext|System Context View]] — System boundaries, external integrations (GitLab, Jira, Kafka, S3, Vault, OpenZiti, MCP protocol).
- 📦 [[Architecture/ComponentStructure|Component & Structural View]] — Workspace breakdown (`factory-core`, `factory-infrastructure`, `factory-application`, `factory-cli`, `factory-mcp-server`) and UML 2.0 Class/Trait diagrams.
- 🔄 [[Architecture/RuntimeSequences|Runtime Sequence View]] — Dynamic interaction flows for mission dispatch, code surgery, AST indexing, and telemetry export.
- 🐳 [[Architecture/DeploymentView|Deployment View]] — Containerization, gVisor sandboxing pod isolation, Cargo target builds, and runtime configurations.
- 🔐 [[Architecture/SecurityView|Security View]] — Ed25519 cryptographic signatures, Zeroize JIT tokens (`ZeroizeOnDrop`), Aethalgard remediation validator, and NHI security bounds.
- 📝 [[Architecture/ADR/ADR_001_AST_Engine|ADR 001: Local AST Engine]] — Architecture Decision Record for local AST parsing without external LLM embedding server dependencies.

---

## 📑 ISO/IEC/IEEE 15289 Specifications & Reports

- 📜 [[Specifications/SRSRequirements|Software Requirements Specification (SRS)]] — Functional and non-functional requirements traceable across system components.
- 🔌 [[Specifications/APIContracts|API & Interface Contracts]] — Trait definitions (`CodeSurgeryExecutor`, `SecurityValidator`, `SandboxDriver`, `AethalgardClient`), MCP tool schemas, and gRPC Proto v1 specifications.
- 📊 [[Quality/ISO25010Quality|ISO 25010 Quality Model Matrix]] — Evaluation of software quality attributes (Functional Suitability, Maintainability, Security, Performance Efficiency, Reliability).
- 🛠️ [[UserGuides/DeveloperGuide|Developer & System User Guide]] — ISO 26514 guide for building, testing (`cargo test`), benchmark execution, and extending MCP tools.
- 🪵 [[Logs|Audit Log & Git History]] — ISO 15289 audit log tracking commit SHAs, AST graph metrics, and documentation revisions.

---

## 🧱 Granular OKF Module Specifications (1:1 Mirrored)

Explore individual crate specifications with exact line-level source code citations:

### ⚙️ `factory-core`
- [[Modules/FactoryCore/Executor|factory-core::executor]] — Surgical patch data structures (`SurgicalPatch`, `ExecutionResult`) & `CodeSurgeryExecutor` trait (`crates/factory-core/src/executor.rs:L1-L28`).
- [[Modules/FactoryCore/Security|factory-core::security]] — Cryptographic validation (`Ed25519SecurityValidator`), `JitToken` with zeroize memory protection, and `SecurityBounds` trait (`crates/factory-core/src/security.rs:L1-L72`).
- [[Modules/FactoryCore/Error|factory-core::error]] — Centralized `FactoryError` enum and result types (`crates/factory-core/src/error.rs:L1-L45`).

### 🔌 `factory-infrastructure`
- [[Modules/FactoryInfrastructure/Aethalgard|factory-infrastructure::aethalgard]] — Remediation notification client (`HttpAethalgardClient`) and JSON-RPC webhook protocol (`crates/factory-infrastructure/src/aethalgard.rs:L1-L56`).
- [[Modules/FactoryInfrastructure/MCPClient|factory-infrastructure::mcp-client]] — Infrastructure integration for Model Context Protocol client connections (`crates/factory-infrastructure/src/mcp_client.rs:L1-L60`).
- [[Modules/FactoryInfrastructure/Ziti|factory-infrastructure::ziti]] — OpenZiti zero-trust network overlay client (`crates/factory-infrastructure/src/ziti.rs:L1-L50`).
- [[Modules/FactoryInfrastructure/Kafka|factory-infrastructure::kafka]] — Event streaming integration for factory event logs (`crates/factory-infrastructure/src/kafka.rs:L1-L65`).

### 🤖 `factory-application`
- [[Modules/FactoryApplication/Agents|factory-application::agents]] — Autonomous agent roles (`Rustant`, `ZeroClaw`, `QAObserver`, `Auditor`, `FinOps`, `DocAgent`) (`crates/factory-application/src/agents/mod.rs:L1-L80`).
- [[Modules/FactoryApplication/Bridge|factory-application::bridge]] — Autonomous Agent Development Kit (ADK) driver & state machine (`crates/factory-application/src/bridge/adk_driver.rs:L1-L110`).
- [[Modules/FactoryApplication/Workflows|factory-application::workflows]] — Mission orchestrators (`AutonomousMission`, `DevelopTask`) (`crates/factory-application/src/workflows/mod.rs:L1-L95`).
- [[Modules/FactoryApplication/TelemetryExport|factory-application::telemetry_export]] — Telemetry metrics exporter (`crates/factory-application/src/telemetry_export.rs:L1-L50`).

### 🛠️ `factory-mcp-server`
- [[Modules/FactoryMCPServer/Protocol|factory-mcp-server::protocol]] — MCP JSON-RPC protocol parser and message schemas (`crates/factory-mcp-server/src/protocol.rs:L1-L120`).
- [[Modules/FactoryMCPServer/Sandbox|factory-mcp-server::sandbox]] — Multi-backend execution engine supporting `NativeSurgerySandboxDriver`, `SubprocessDriver`, and `GvisorK8sDriver` (`crates/factory-mcp-server/src/sandbox.rs:L1-L190`).
- [[Modules/FactoryMCPServer/FeedbackRoute|factory-mcp-server::feedback_route]] — HTTP feedback endpoint (`crates/factory-mcp-server/src/feedback_route.rs:L1-L75`).

### 🖥️ `factory-cli`
- [[Modules/FactoryCLI/Main|factory-cli::main]] — Command line interface binary for mission triggers and local factory control (`crates/factory-cli/src/main.rs:L1-L85`).


## Auto-Generated Module Architecture Links
* [[crates/factory-application/src/agents/index.md]] - agents Module Architecture
* [[crates/factory-application/src/bridge/index.md]] - bridge Module Architecture
* [[crates/factory-application/src/index.md]] - src Module Architecture
* [[crates/factory-application/src/utils/index.md]] - utils Module Architecture
* [[crates/factory-application/src/workflows/index.md]] - workflows Module Architecture
* [[crates/factory-application/tests/index.md]] - tests Module Architecture
* [[crates/factory-cli/src/bin/index.md]] - bin Module Architecture
* [[crates/factory-cli/src/index.md]] - src Module Architecture
* [[crates/factory-core/benches/index.md]] - benches Module Architecture
* [[crates/factory-core/index.md]] - factory-core Module Architecture
* [[crates/factory-core/src/index.md]] - src Module Architecture
* [[crates/factory-core/src/security/index.md]] - security Module Architecture
* [[crates/factory-core/tests/index.md]] - tests Module Architecture
* [[crates/factory-infrastructure/src/index.md]] - src Module Architecture
* [[crates/factory-infrastructure/tests/index.md]] - tests Module Architecture
* [[crates/factory-mcp-server/src/index.md]] - src Module Architecture
* [[crates/factory-mcp-server/src/skills/index.md]] - skills Module Architecture
* [[crates/factory-mcp-server/src/tools/index.md]] - tools Module Architecture
* [[crates/factory-mcp-server/tests/index.md]] - tests Module Architecture
