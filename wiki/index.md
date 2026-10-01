---
iso_doc_type: "Description"
iso_viewpoint: "ArchitectureDescription"
type: "architecture"
title: "Master Wiki Index — ISO Architecture Description"
description: "Master index of the Dark Gravity CA/CD Autonomous Factory documentation under ISO/IEC/IEEE 42010 and 15289 standards."
tags: ["iso42010", "iso15289", "index", "architecture_description"]
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# Master Documentation Index — Dark Gravity CA/CD Autonomous Factory

> **Standards Compliance**: [ISO/IEC/IEEE 42010:2022](https://www.iso.org/standard/74428.html) (Architecture Descriptions) & [ISO/IEC/IEEE 15289:2019](https://www.iso.org/standard/72242.html) (Life Cycle Information Items).

---

## Navigation & Entry Points

- [Home](Home.md) · [Glossary](GLOSSARY.md) · [Sidebar](_Sidebar.md)

---

## 1. Architecture Viewpoints (ISO 42010)

| Viewpoint | Document | Description |
|:---|:---|:---|
| **ContextView** | [Business Context](architecture/business_context.md) | Problem statement, ROI, KPIs, autonomous paradigm |
| **ContextView** | [Strategic Design](architecture/strategic_design.md) | C4 Level 1-2, Onion Architecture, Bounded Contexts |
| **ComponentView** | [Tactical Design](architecture/tactical_design.md) | C4 Level 3, Hatchet DAG mapping, tool registry |
| **ComponentView** | [Agent Specifications](architecture/agent_specifications.md) | Specification of all 6 agents, shared trait, tool matrix |
| **SequenceView** | [Runtime Sequences](architecture/runtime_sequences.md) | 6-Phase Hatchet DAG execution, agent thought streaming |
| **ComponentView** | [Infrastructure Adapters](architecture/infrastructure_adapters.md) | Kafka, R2R GraphRAG, OpenZiti, Vault, Sentry, S3, Semantica |
| **ComponentView** | [Mission Data Model](architecture/mission_data_model.md) | Event schemas, protobufs, JSON data contracts |

---

## 2. Security & Governance (ISO 42010 SecurityView & Policy)

| Topic | Document | Description |
|:---|:---|:---|
| **SecurityView** | [Security Architecture](security/security_architecture.md) | Zero Trust, NHI Ed25519, gVisor isolation, STRIDE threat model |
| **Policy** | [HITL Governance](security/hitl_governance.md) | 4-Vertex Human-in-the-Loop governance mesh & enforcement |
| **Policy** | [Verification Triad](security/verification_triad.md) | Logical, Architectural, and Security quality gates |
| **Report** | [Compliance & Audit](security/compliance_audit.md) | Hazitek 2026, EU AI Act conformity assessment |

---

## 3. Operations & Deployment (ISO 42010 DeploymentView & Procedures)

| Topic | Document | Description |
|:---|:---|:---|
| **Procedure** | [User Manual](operations/user_manual.md) | Step-by-step guides for mission creation, PR directives, CLI |
| **DeploymentView** | [Production Operations](operations/production_operations.md) | FluxCD GitOps, K8s namespace topology, LiteLLM proxy, runbook |
| **Report** | [Experiment Logs](operations/experiment_logs.md) | Telemetry schema, Kafka event topics, Prometheus metrics |

---

## 4. Software Quality & Testing (ISO 25010 & ISO 29119)

| Topic | Document | Description |
|:---|:---|:---|
| **QualityView** | [Test Plan & Report](quality/test_plan_report.md) | Test pyramid, 18 integration suites, Criterion benchmarks |

---

## 5. Module Architecture & Codebase Mirror (ISO 42010 ComponentView)

### factory-core (Domain Layer)
- [lib.md](modules/core/lib.md) — Core domain entities, mission & task models
- [config.md](modules/core/config.md) — Model configuration & LiteLLM resolution
- [error.md](modules/core/error.md) — `FactoryError` enum variants
- [executor.md](modules/core/executor.md) — `CodeSurgeryExecutor` trait
- [security.md](modules/core/security.md) — Security primitives & sandbox constraints
- [security_nhi.md](modules/core/security_nhi.md) — Ed25519 Non-Human Identity credentials

### factory-application (Application Layer)
- [lib.md](modules/application/lib.md) — Application crate overview & `Agent` trait
- [gitlab_verifier.md](modules/application/gitlab_verifier.md) — GitLab connectivity & token verifier
- [poller_service.md](modules/application/poller_service.md) — Outbound GitHub/GitLab poller daemon
- [telemetry_export.md](modules/application/telemetry_export.md) — Metrics & telemetry exporter
- **Agents** (`modules/application/agents/`):
  - [mod.md](modules/application/agents/mod.md) — Agent dispatch registry
  - [rustant.md](modules/application/agents/rustant.md) — Product Owner & Planner agent
  - [zeroclaw.md](modules/application/agents/zeroclaw.md) — Developer & Code Surgery agent
  - [auditor.md](modules/application/agents/auditor.md) — Security Auditor agent
  - [finops.md](modules/application/agents/finops.md) — Budget & Token Cost agent
  - [qa_observer.md](modules/application/agents/qa_observer.md) — Quality Gate & Coverage agent
  - [doc_agent.md](modules/application/agents/doc_agent.md) — Documentation & OSR agent
- **Workflows** (`modules/application/workflows/`):
  - [mod.md](modules/application/workflows/mod.md) — Workflow registry
  - [autonomous_mission.md](modules/application/workflows/autonomous_mission.md) — 6-Phase Hatchet DAG
  - [circuit_breaker.md](modules/application/workflows/circuit_breaker.md) — Aethelgard loop breaker
  - [comment_control.md](modules/application/workflows/comment_control.md) — PR/MR directive handler
  - [deep_research.md](modules/application/workflows/deep_research.md) — Multi-step research DAG
  - [develop_task.md](modules/application/workflows/develop_task.md) — ZeroClaw TDD execution
  - [pipeline_remediation.md](modules/application/workflows/pipeline_remediation.md) — CI failure healer
- **Bridge** (`modules/application/bridge/`):
  - [mod.md](modules/application/bridge/mod.md) — Bridge abstractions
  - [adk_driver.md](modules/application/bridge/adk_driver.md) — Agent Development Kit bridge
  - [kafka_bridge.md](modules/application/bridge/kafka_bridge.md) — Kafka bidirectional bridge
  - [semantica_bridge.md](modules/application/bridge/semantica_bridge.md) — Semantica decision recorder
  - [state.md](modules/application/bridge/state.md) — Checkpoint state tracking
- **Utils** (`modules/application/utils/`):
  - [mod.md](modules/application/utils/mod.md) — Application utilities
  - [osr.md](modules/application/utils/osr.md) — Out-of-Sync Rate calculator

### factory-infrastructure (Infrastructure Layer)
- [lib.md](modules/infrastructure/lib.md) — Adapter registry & traits
- [github.md](modules/infrastructure/github.md) — GitHub REST API adapter
- [gitlab.md](modules/infrastructure/gitlab.md) — GitLab API adapter
- [jira.md](modules/infrastructure/jira.md) — Atlassian Jira Cloud adapter
- [kafka.md](modules/infrastructure/kafka.md) — KRaft Kafka producer/consumer
- [ziti.md](modules/infrastructure/ziti.md) — OpenZiti zero-trust overlay
- [vault.md](modules/infrastructure/vault.md) — HashiCorp Vault secrets adapter
- [r2r.md](modules/infrastructure/r2r.md) — R2R GraphRAG vector client
- [semantica.md](modules/infrastructure/semantica.md) — Semantica decision & conflict client
- [aethalgard.md](modules/infrastructure/aethalgard.md) — Circuit breaker adapter
- [mcp_client.md](modules/infrastructure/mcp_client.md) — Model Context Protocol client
- [pipeline_classifier.md](modules/infrastructure/pipeline_classifier.md) — CI/CD error taxonomy
- [s3.md](modules/infrastructure/s3.md) — MinIO / S3 artifact storage
- [sentry.md](modules/infrastructure/sentry.md) — Error tracking & diagnostics
- [security_validator.md](modules/infrastructure/security_validator.md) — Ed25519 signature validator
- [cursor_store.md](modules/infrastructure/cursor_store.md) — Event cursor store (In-memory & Postgres)
- [git_poller.md](modules/infrastructure/git_poller.md) — Change detection daemon

### factory-mcp-server (Interface Layer)
- [lib.md](modules/mcp_server/lib.md) — Axum MCP server core
- [main.md](modules/mcp_server/main.md) — Server entry point & CLI flags
- [protocol.md](modules/mcp_server/protocol.md) — JSON-RPC 2.0 framing & SSE
- [sandbox.md](modules/mcp_server/sandbox.md) — gVisor K8s Job sandbox lifecycle
- [scratch.md](modules/mcp_server/scratch.md) — Scratchpad filesystem manager
- [feedback_route.md](modules/mcp_server/feedback_route.md) — External webhook ingress
- [github_webhook.md](modules/mcp_server/github_webhook.md) — HMAC-SHA256 GitHub event listener
- **Skills** (`modules/mcp_server/skills/`):
  - [mod.md](modules/mcp_server/skills/mod.md) — Skill registry
  - [context.md](modules/mcp_server/skills/context.md) — Spec-Kit execution context
  - [spec_kit_tool.md](modules/mcp_server/skills/spec_kit_tool.md) — SDD Spec-Kit bridge
- **Tools** (`modules/mcp_server/tools/`):
  - [mod.md](modules/mcp_server/tools/mod.md) — Tool registry (17 tools)
  - [bridge.md](modules/mcp_server/tools/bridge.md) — Bridge communication tool
  - [deep_research_tool.md](modules/mcp_server/tools/deep_research_tool.md) — Deep research tool
  - [execute_code.md](modules/mcp_server/tools/execute_code.md) — Code surgery tool
  - [get_factory_status.md](modules/mcp_server/tools/get_factory_status.md) — Factory status tool
  - [index_code.md](modules/mcp_server/tools/index_code.md) — Code indexing tool
  - [inspect_kafka_topic.md](modules/mcp_server/tools/inspect_kafka_topic.md) — Kafka inspector tool
  - [launch_sandbox_pod.md](modules/mcp_server/tools/launch_sandbox_pod.md) — Sandbox launcher tool
  - [list_minio_buckets.md](modules/mcp_server/tools/list_minio_buckets.md) — Bucket discovery tool
  - [list_minio_objects.md](modules/mcp_server/tools/list_minio_objects.md) — Object listing tool
  - [plan_mission.md](modules/mcp_server/tools/plan_mission.md) — Mission planner tool
  - [retrieve_context.md](modules/mcp_server/tools/retrieve_context.md) — R2R context retrieval tool
  - [run_tests.md](modules/mcp_server/tools/run_tests.md) — Test runner tool
  - [search_jira.md](modules/mcp_server/tools/search_jira.md) — Jira query tool
  - [security_review.md](modules/mcp_server/tools/security_review.md) — SAST security gate tool
  - [spec_kit_tasks_to_issues.md](modules/mcp_server/tools/spec_kit_tasks_to_issues.md) — Task-to-issue tool
  - [spec_kit_tool.md](modules/mcp_server/tools/spec_kit_tool.md) — Spec Kit CLI tool
  - [update_mission_status.md](modules/mcp_server/tools/update_mission_status.md) — Mission status updater

### factory-cli (Interface Layer)
- [main.md](modules/cli/main.md) — CLI entry point & subcommands (`worker`, `poller`, `gitlab-verify`, `verify-osr`)
- [trigger_mission.md](modules/cli/trigger_mission.md) — Manual mission trigger binary
- [run_functional_suite.md](modules/cli/run_functional_suite.md) — Functional E2E test runner binary
- [trigger_deep_search.md](modules/cli/trigger_deep_search.md) — Deep research trigger binary
