## 🏠 Navigation

- [Home](Home.md)
- [Master Index](index.md)
- [Glossary](GLOSSARY.md)

## 📊 Architecture (ISO 42010)

- [Business Context](architecture/business_context.md)
- [Strategic Design](architecture/strategic_design.md)
- [Tactical Design](architecture/tactical_design.md)
- [Agent Specifications](architecture/agent_specifications.md)
- [Runtime Sequences](architecture/runtime_sequences.md)
- [Infrastructure Adapters](architecture/infrastructure_adapters.md)
- [Mission Data Model](architecture/mission_data_model.md)

## 🔒 Security & Governance

- [Security Architecture](security/security_architecture.md)
- [HITL Governance](security/hitl_governance.md)
- [Verification Triad](security/verification_triad.md)
- [Compliance & Audit](security/compliance_audit.md)

## 📖 Operations & Quality

- [User Manual](operations/user_manual.md)
- [Production Operations](operations/production_operations.md)
- [Experiment Logs](operations/experiment_logs.md)
- [Test Plan & Report](quality/test_plan_report.md)

## 📦 Crate Modules (1:1 Mirror)

### factory-core (Domain)
- [lib.md](modules/core/lib.md)
- [config.md](modules/core/config.md)
- [error.md](modules/core/error.md)
- [executor.md](modules/core/executor.md)
- [security.md](modules/core/security.md)
- [security_nhi.md](modules/core/security_nhi.md)

### factory-application
- [lib.md](modules/application/lib.md)
- [gitlab_verifier.md](modules/application/gitlab_verifier.md)
- [poller_service.md](modules/application/poller_service.md)
- [telemetry_export.md](modules/application/telemetry_export.md)
- **Agents**
  - [mod.md](modules/application/agents/mod.md)
  - [rustant.md](modules/application/agents/rustant.md)
  - [zeroclaw.md](modules/application/agents/zeroclaw.md)
  - [auditor.md](modules/application/agents/auditor.md)
  - [finops.md](modules/application/agents/finops.md)
  - [qa_observer.md](modules/application/agents/qa_observer.md)
  - [doc_agent.md](modules/application/agents/doc_agent.md)
- **Workflows**
  - [mod.md](modules/application/workflows/mod.md)
  - [autonomous_mission.md](modules/application/workflows/autonomous_mission.md)
  - [circuit_breaker.md](modules/application/workflows/circuit_breaker.md)
  - [comment_control.md](modules/application/workflows/comment_control.md)
  - [deep_research.md](modules/application/workflows/deep_research.md)
  - [develop_task.md](modules/application/workflows/develop_task.md)
  - [pipeline_remediation.md](modules/application/workflows/pipeline_remediation.md)
- **Bridge**
  - [mod.md](modules/application/bridge/mod.md)
  - [adk_driver.md](modules/application/bridge/adk_driver.md)
  - [kafka_bridge.md](modules/application/bridge/kafka_bridge.md)
  - [semantica_bridge.md](modules/application/bridge/semantica_bridge.md)
  - [state.md](modules/application/bridge/state.md)
- **Utils**
  - [mod.md](modules/application/utils/mod.md)
  - [osr.md](modules/application/utils/osr.md)

### factory-infrastructure
- [lib.md](modules/infrastructure/lib.md)
- [github.md](modules/infrastructure/github.md)
- [gitlab.md](modules/infrastructure/gitlab.md)
- [jira.md](modules/infrastructure/jira.md)
- [kafka.md](modules/infrastructure/kafka.md)
- [ziti.md](modules/infrastructure/ziti.md)
- [vault.md](modules/infrastructure/vault.md)
- [r2r.md](modules/infrastructure/r2r.md)
- [semantica.md](modules/infrastructure/semantica.md)
- [aethalgard.md](modules/infrastructure/aethalgard.md)
- [mcp_client.md](modules/infrastructure/mcp_client.md)
- [pipeline_classifier.md](modules/infrastructure/pipeline_classifier.md)
- [s3.md](modules/infrastructure/s3.md)
- [sentry.md](modules/infrastructure/sentry.md)
- [security_validator.md](modules/infrastructure/security_validator.md)
- [cursor_store.md](modules/infrastructure/cursor_store.md)
- [git_poller.md](modules/infrastructure/git_poller.md)

### factory-mcp-server
- [lib.md](modules/mcp_server/lib.md)
- [main.md](modules/mcp_server/main.md)
- [protocol.md](modules/mcp_server/protocol.md)
- [sandbox.md](modules/mcp_server/sandbox.md)
- [scratch.md](modules/mcp_server/scratch.md)
- [feedback_route.md](modules/mcp_server/feedback_route.md)
- [github_webhook.md](modules/mcp_server/github_webhook.md)
- **Skills**
  - [mod.md](modules/mcp_server/skills/mod.md)
  - [context.md](modules/mcp_server/skills/context.md)
  - [spec_kit_tool.md](modules/mcp_server/skills/spec_kit_tool.md)
- **Tools**
  - [mod.md](modules/mcp_server/tools/mod.md)
  - [bridge.md](modules/mcp_server/tools/bridge.md)
  - [deep_research_tool.md](modules/mcp_server/tools/deep_research_tool.md)
  - [execute_code.md](modules/mcp_server/tools/execute_code.md)
  - [get_factory_status.md](modules/mcp_server/tools/get_factory_status.md)
  - [index_code.md](modules/mcp_server/tools/index_code.md)
  - [inspect_kafka_topic.md](modules/mcp_server/tools/inspect_kafka_topic.md)
  - [launch_sandbox_pod.md](modules/mcp_server/tools/launch_sandbox_pod.md)
  - [list_minio_buckets.md](modules/mcp_server/tools/list_minio_buckets.md)
  - [list_minio_objects.md](modules/mcp_server/tools/list_minio_objects.md)
  - [plan_mission.md](modules/mcp_server/tools/plan_mission.md)
  - [retrieve_context.md](modules/mcp_server/tools/retrieve_context.md)
  - [run_tests.md](modules/mcp_server/tools/run_tests.md)
  - [search_jira.md](modules/mcp_server/tools/search_jira.md)
  - [security_review.md](modules/mcp_server/tools/security_review.md)
  - [spec_kit_tasks_to_issues.md](modules/mcp_server/tools/spec_kit_tasks_to_issues.md)
  - [spec_kit_tool.md](modules/mcp_server/tools/spec_kit_tool.md)
  - [update_mission_status.md](modules/mcp_server/tools/update_mission_status.md)

### factory-cli
- [main.md](modules/cli/main.md)
- [trigger_mission.md](modules/cli/trigger_mission.md)
- [run_functional_suite.md](modules/cli/run_functional_suite.md)
- [trigger_deep_search.md](modules/cli/trigger_deep_search.md)