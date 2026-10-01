## 🏠 Home

- [Home](Home)
- [Master Index](index)
- [Glossary](GLOSSARY)

## 📊 Business & Architecture

- [Business Context](BUSINESS-CONTEXT)
- [Strategic Design](STRATEGIC-DESIGN)
- [Tactical Design](TACTICAL-DESIGN)

## 🤖 Agents & Lifecycle

- [Agent Specifications](AGENT-SPECIFICATIONS)
- [Experiment Lifecycle](EXPERIMENT-LIFECYCLE)
- [Experiment Logs](EXPERIMENT-LOGS)
- [Mission Data Model](src-mission)

## 🔌 Infrastructure

- [Infrastructure Adapters](INFRASTRUCTURE-ADAPTERS)
- [Production Operations](PRODUCTION-OPERATIONS)

## 🔒 Security & Governance

- [Security Architecture](SECURITY-ARCHITECTURE)
- [HITL Governance](HITL-GOVERNANCE)
- [Verification Triad](VERIFICATION-TRIAD)
- [Compliance & Audit](COMPLIANCE-AUDIT)

## 📖 Operations

- [User Manual](USER-MANUAL)
- [Test Plan Report](Test_Plan_Report)
- [Spec Kit Tool](spec_kit_tool)

## 📦 Crate Reference

### factory-core
- [lib.rs](crates_factory-core_src_lib)
- [error.rs](crates_factory-core_src_error)
- [executor.rs](crates_factory-core_src_executor)
- [security.rs](crates_factory-core_src_security)
- [security/nhi.rs](crates_factory-core_src_security_nhi)

### factory-application
- [lib.rs](crates_factory-application_src_lib)
- **Agents**
  - [mod.rs](crates_factory-application_src_agents_mod)
  - [rustant.rs](crates_factory-application_src_agents_rustant)
  - [zeroclaw.rs](crates_factory-application_src_agents_zeroclaw)
  - [auditor.rs](crates_factory-application_src_agents_auditor)
  - [finops.rs](crates_factory-application_src_agents_finops)
  - [qa_observer.rs](crates_factory-application_src_agents_qa_observer)
  - [doc_agent.rs](crates_factory-application_src_agents_doc_agent)
- **Workflows**
  - [mod.rs](crates_factory-application_src_workflows_mod)
  - [autonomous_mission.rs](crates_factory-application_src_workflows_autonomous_mission)
  - [circuit_breaker.rs](crates_factory-application_src_workflows_circuit_breaker)
  - [develop_task.rs](crates_factory-application_src_workflows_develop_task)
  - [pipeline_remediation.rs](crates_factory-application_src_workflows_pipeline_remediation)
- **Bridge**
  - [mod.rs](crates_factory-application_src_bridge_mod)
  - [adk_driver.rs](crates_factory-application_src_bridge_adk_driver)
  - [kafka_bridge.rs](crates_factory-application_src_bridge_kafka_bridge)
  - [state.rs](crates_factory-application_src_bridge_state)
- [poller_service.rs](crates_factory-application_src_poller_service)
- [telemetry_export.rs](crates_factory-application_src_telemetry_export)
- [utils/osr.rs](crates_factory-application_src_utils_osr)

### factory-infrastructure
- [lib.rs](crates_factory-infrastructure_src_lib)
- [github.rs](crates_factory-infrastructure_src_github)
- [gitlab.rs](crates_factory-infrastructure_src_gitlab)
- [jira.rs](crates_factory-infrastructure_src_jira)
- [kafka.rs](crates_factory-infrastructure_src_kafka)
- [ziti.rs](crates_factory-infrastructure_src_ziti)
- [vault.rs](crates_factory-infrastructure_src_vault)
- [r2r.rs](crates_factory-infrastructure_src_r2r)
- [aethalgard.rs](crates_factory-infrastructure_src_aethalgard)
- [mcp_client.rs](crates_factory-infrastructure_src_mcp_client)
- [pipeline_classifier.rs](crates_factory-infrastructure_src_pipeline_classifier)
- [s3.rs](crates_factory-infrastructure_src_s3)
- [sentry.rs](crates_factory-infrastructure_src_sentry)
- [security_validator.rs](crates_factory-infrastructure_src_security_validator)

### factory-mcp-server
- [lib.rs](crates_factory-mcp-server_src_lib)
- [main.rs](crates_factory-mcp-server_src_main)
- [protocol.rs](crates_factory-mcp-server_src_protocol)
- [sandbox.rs](crates_factory-mcp-server_src_sandbox)
- **Tools**
  - [execute_code.rs](crates_factory-mcp-server_src_tools_execute_code)
  - [launch_sandbox_pod.rs](crates_factory-mcp-server_src_tools_launch_sandbox_pod)
  - [run_tests.rs](crates_factory-mcp-server_src_tools_run_tests)
  - [security_review.rs](crates_factory-mcp-server_src_tools_security_review)
  - [spec_kit_tool.rs](crates_factory-mcp-server_src_tools_spec_kit_tool)
  - [plan_mission.rs](crates_factory-mcp-server_src_tools_plan_mission)
  - [retrieve_context.rs](crates_factory-mcp-server_src_tools_retrieve_context)

### factory-cli
- [main.rs](crates_factory-cli_src_main)
- [trigger_mission.rs](crates_factory-cli_src_bin_trigger_mission)
- [run_functional_suite.rs](crates_factory-cli_src_bin_run_functional_suite)