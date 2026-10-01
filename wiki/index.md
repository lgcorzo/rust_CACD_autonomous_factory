# Wiki Index — Dark Gravity CA/CD Autonomous Factory

> **Purpose**: Master index linking all 100+ wiki pages organized by category.

---

## Navigation

- [Home](Home.md) · [Sidebar](_Sidebar.md) · [Glossary](GLOSSARY.md)

---

## Business & Strategy

- [Business Context](BUSINESS-CONTEXT.md) — Vision, ROI, KPIs
- [Strategic Design](STRATEGIC-DESIGN.md) — C4 Level 1-2, Onion Architecture, Bounded Contexts
- [Tactical Design](TACTICAL-DESIGN.md) — C4 Level 3, DAG mapping, MCP tool registry

## Architecture & Agents

- [Agent Specifications](AGENT-SPECIFICATIONS.md) — All 6 agents with UML, tool matrix, behavioral specs
- [Experiment Lifecycle](EXPERIMENT-LIFECYCLE.md) — 6-Phase Hatchet DAG sequence
- [Infrastructure Adapters](INFRASTRUCTURE-ADAPTERS.md) — Kafka, R2R, Ziti, Vault, Sentry, S3, Semantica

## Security & Governance

- [Security Architecture](SECURITY-ARCHITECTURE.md) — Zero Trust, NHI, gVisor, SAST, STRIDE
- [HITL Governance](HITL-GOVERNANCE.md) — 4-Vertex governance mesh
- [Verification Triad](VERIFICATION-TRIAD.md) — Logical, Architectural, Security gates
- [Compliance & Audit](COMPLIANCE-AUDIT.md) — Hazitek 2026, EU AI Act

## Operations & Quality

- [User Manual](USER-MANUAL.md) — Step-by-step guides
- [Production Operations](PRODUCTION-OPERATIONS.md) — FluxCD, K8s, LiteLLM, troubleshooting
- [Experiment Logs](EXPERIMENT-LOGS.md) — Telemetry schema
- [Test Plan Report](Test_Plan_Report.md) — Test strategy & suite catalog
- [Mission Data Model](src-mission.md) — Event-driven architecture

## Crate Reference: factory-core (Domain)

- [lib.rs](crates_factory-core_src_lib.md) — Domain entities, module tree
- [config.rs](crates_factory-core_src_config.md) — Agent model configuration
- [error.rs](crates_factory-core_src_error.md) — FactoryError enum
- [executor.rs](crates_factory-core_src_executor.md) — CodeSurgeryExecutor trait
- [security.rs](crates_factory-core_src_security.md) — Security primitives
- [security/nhi.rs](crates_factory-core_src_security_nhi.md) — Ed25519 NHI credentials

## Crate Reference: factory-application (Application)

- [lib.rs](crates_factory-application_src_lib.md) — Application overview
- [agents/mod.rs](crates_factory-application_src_agents_mod.md) — Agent registry
- [agents/rustant.rs](crates_factory-application_src_agents_rustant.md) — RustantAgent
- [agents/zeroclaw.rs](crates_factory-application_src_agents_zeroclaw.md) — ZeroClawAgent
- [agents/auditor.rs](crates_factory-application_src_agents_auditor.md) — AuditorAgent
- [agents/finops.rs](crates_factory-application_src_agents_finops.md) — FinOpsAgent
- [agents/qa_observer.rs](crates_factory-application_src_agents_qa_observer.md) — QAObserverAgent
- [agents/doc_agent.rs](crates_factory-application_src_agents_doc_agent.md) — DocumentationAgent
- [gitlab_verifier.rs](crates_factory-application_src_gitlab_verifier.md) — GitLab connectivity verifier
- [workflows/mod.rs](crates_factory-application_src_workflows_mod.md) — Workflow registry
- [workflows/autonomous_mission.rs](crates_factory-application_src_workflows_autonomous_mission.md) — 6-Phase DAG
- [workflows/circuit_breaker.rs](crates_factory-application_src_workflows_circuit_breaker.md) — Aethelgard
- [workflows/comment_control.rs](crates_factory-application_src_workflows_comment_control.md) — PR comment control
- [workflows/deep_research.rs](crates_factory-application_src_workflows_deep_research.md) — Deep research DAG
- [workflows/develop_task.rs](crates_factory-application_src_workflows_develop_task.md) — TDD execution
- [workflows/pipeline_remediation.rs](crates_factory-application_src_workflows_pipeline_remediation.md) — CI fix
- [bridge/mod.rs](crates_factory-application_src_bridge_mod.md) — Bridge module
- [bridge/adk_driver.rs](crates_factory-application_src_bridge_adk_driver.md) — ADK integration
- [bridge/kafka_bridge.rs](crates_factory-application_src_bridge_kafka_bridge.md) — Kafka bridge
- [bridge/semantica_bridge.rs](crates_factory-application_src_bridge_semantica_bridge.md) — Semantica bridge
- [bridge/state.rs](crates_factory-application_src_bridge_state.md) — Checkpoint state
- [poller_service.rs](crates_factory-application_src_poller_service.md) — Outbound poller
- [telemetry_export.rs](crates_factory-application_src_telemetry_export.md) — Metrics export
- [utils/mod.rs](crates_factory-application_src_utils_mod.md) — Utilities
- [utils/osr.rs](crates_factory-application_src_utils_osr.md) — OSR calculator

## Crate Reference: factory-infrastructure (Infrastructure)

- [lib.rs](crates_factory-infrastructure_src_lib.md) — Adapter registry
- [github.rs](crates_factory-infrastructure_src_github.md) — GitHub adapter
- [gitlab.rs](crates_factory-infrastructure_src_gitlab.md) — GitLab adapter
- [jira.rs](crates_factory-infrastructure_src_jira.md) — Jira adapter
- [kafka.rs](crates_factory-infrastructure_src_kafka.md) — Kafka client
- [ziti.rs](crates_factory-infrastructure_src_ziti.md) — OpenZiti client
- [vault.rs](crates_factory-infrastructure_src_vault.md) — Vault secret manager
- [r2r.rs](crates_factory-infrastructure_src_r2r.md) — R2R GraphRAG
- [aethalgard.rs](crates_factory-infrastructure_src_aethalgard.md) — Circuit breaker
- [mcp_client.rs](crates_factory-infrastructure_src_mcp_client.md) — MCP client
- [pipeline_classifier.rs](crates_factory-infrastructure_src_pipeline_classifier.md) — Error taxonomy
- [s3.rs](crates_factory-infrastructure_src_s3.md) — S3 storage
- [semantica.rs](crates_factory-infrastructure_src_semantica.md) — Semantica decision client
- [sentry.rs](crates_factory-infrastructure_src_sentry.md) — Error tracking
- [security_validator.rs](crates_factory-infrastructure_src_security_validator.md) — Ed25519 validator
- [cursor_store.rs](crates_factory-infrastructure_src_cursor_store.md) — Polling state
- [git_poller.rs](crates_factory-infrastructure_src_git_poller.md) — Change detection

## Crate Reference: factory-mcp-server (Interface)

- [lib.rs](crates_factory-mcp-server_src_lib.md) — MCP server overview
- [main.rs](crates_factory-mcp-server_src_main.md) — HTTP server entry
- [protocol.rs](crates_factory-mcp-server_src_protocol.md) — JSON-RPC protocol
- [sandbox.rs](crates_factory-mcp-server_src_sandbox.md) — gVisor lifecycle
- [feedback_route.rs](crates_factory-mcp-server_src_feedback_route.md) — Webhook handler
- [github_webhook.rs](crates_factory-mcp-server_src_github_webhook.md) — GitHub webhook handler
- [scratch.rs](crates_factory-mcp-server_src_scratch.md) — Temp workspace
- [skills/context.rs](crates_factory-mcp-server_src_skills_context.md) — Spec-Kit context
- [skills/mod.rs](crates_factory-mcp-server_src_skills_mod.md) — Skill registry
- [tools/mod.rs](crates_factory-mcp-server_src_tools_mod.md) — Tool dispatch
- [tools/bridge.rs](crates_factory-mcp-server_src_tools_bridge.md) — State bridge
- [tools/deep_research_tool.rs](crates_factory-mcp-server_src_tools_deep_research_tool.md) — Deep research
- [tools/execute_code.rs](crates_factory-mcp-server_src_tools_execute_code.md) — Code execution
- [tools/get_factory_status.rs](crates_factory-mcp-server_src_tools_get_factory_status.md) — Factory status
- [tools/index_code.rs](crates_factory-mcp-server_src_tools_index_code.md) — R2R ingestion
- [tools/inspect_kafka_topic.rs](crates_factory-mcp-server_src_tools_inspect_kafka_topic.md) — Kafka inspector
- [tools/launch_sandbox_pod.rs](crates_factory-mcp-server_src_tools_launch_sandbox_pod.md) — K8s Job
- [tools/list_minio_buckets.rs](crates_factory-mcp-server_src_tools_list_minio_buckets.md) — MinIO buckets
- [tools/list_minio_objects.rs](crates_factory-mcp-server_src_tools_list_minio_objects.md) — MinIO objects
- [tools/plan_mission.rs](crates_factory-mcp-server_src_tools_plan_mission.md) — Mission planning
- [tools/retrieve_context.rs](crates_factory-mcp-server_src_tools_retrieve_context.md) — GraphRAG query
- [tools/run_tests.rs](crates_factory-mcp-server_src_tools_run_tests.md) — Test runner
- [tools/search_jira.rs](crates_factory-mcp-server_src_tools_search_jira.md) — Jira search
- [tools/security_review.rs](crates_factory-mcp-server_src_tools_security_review.md) — SAST gate
- [tools/spec_kit_tool.rs](crates_factory-mcp-server_src_tools_spec_kit_tool.md) — SDD bridge
- [tools/spec_kit_tasks_to_issues.rs](crates_factory-mcp-server_src_tools_spec_kit_tasks_to_issues.md) — Issue creator
- [tools/update_mission_status.rs](crates_factory-mcp-server_src_tools_update_mission_status.md) — Status tracking
- [Spec Kit Tool](spec_kit_tool.md) — Comprehensive tool docs

## Crate Reference: factory-cli (Interface)

- [main.rs](crates_factory-cli_src_main.md) — CLI entry point & subcommands
- [bin/trigger_mission.rs](crates_factory-cli_src_bin_trigger_mission.md) — Manual trigger
- [bin/run_functional_suite.rs](crates_factory-cli_src_bin_run_functional_suite.md) — E2E tests
- [bin/trigger_deep_search.rs](crates_factory-cli_src_bin_trigger_deep_search.md) — Deep search trigger
