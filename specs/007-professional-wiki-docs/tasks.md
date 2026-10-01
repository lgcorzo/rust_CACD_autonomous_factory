# Tasks: Professional Enterprise Wiki Documentation

**Input**: Design documents from `specs/007-professional-wiki-docs/`

**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅, quickstart.md ✅

**Tests**: Not explicitly requested in the feature specification. Validation is via quickstart.md bash scenarios and OSR < 5% quality gate.

**Organization**: Tasks grouped by user story (US1–US4 from spec.md) to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3, US4)
- Includes exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish wiki scaffolding, navigation, and shared conventions

- [X] T001 Audit existing wiki/ directory and catalog all 84 files with current content depth in wiki/index.md
- [X] T002 Define OKF YAML frontmatter template and Mermaid style conventions in wiki/README.md
- [X] T003 [P] Rewrite wiki/Home.md as the global navigation hub with section links, project overview, and quick-start entry points
- [X] T004 [P] Rewrite wiki/_Sidebar.md with the canonical hierarchical navigation tree per research.md Decision 2
- [X] T005 [P] Rewrite wiki/GLOSSARY.md with comprehensive DDD ubiquitous language covering all domain terms, agent names, protocol acronyms, and Kubernetes concepts

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core architectural models and business context that ALL user stories depend on

**⚠️ CRITICAL**: User story implementation depends on accurate C4 and business context being established first

- [X] T006 Read and analyze all 5 workspace crate source files to extract structs, traits, enums, and public functions for AST symbol inventory
- [X] T007 [P] Read and analyze config/ directory for runtime YAML/TOML configuration schemas
- [X] T008 [P] Read and analyze .github/workflows/ for CI/CD pipeline documentation (especially docs-to-wiki.yml)
- [X] T009 [P] Read and analyze Cargo.toml workspace manifest and all crate Cargo.toml files for dependency documentation

**Checkpoint**: Source inventory complete — user story content generation can begin

---

## Phase 3: User Story 1 — Comprehensive Architectural & Business System Overview (Priority: P1) 🎯 MVP

**Goal**: Deliver C4 Context/Container/Component diagrams, business context, strategic design, and bounded context mapping so architects and stakeholders understand the full system

**Independent Test**: Verify wiki/BUSINESS-CONTEXT.md, wiki/STRATEGIC-DESIGN.md, wiki/TACTICAL-DESIGN.md contain valid C4 Mermaid diagrams at C1, C2, C3 levels; all external systems (GitHub, GitLab, Jira, FluxCD, Hatchet, Kafka, OpenZiti, LiteLLM) are named and explained

### Implementation for User Story 1

- [X] T010 [US1] Rewrite wiki/BUSINESS-CONTEXT.md with CA/CD vision, Hazitek 2026 innovation objectives, ROI metrics, KPI matrix, and autonomous workforce value proposition
- [X] T011 [US1] Rewrite wiki/STRATEGIC-DESIGN.md with C4 Level 1 (System Context) Mermaid diagram showing human actors (PO, Tech Lead, Architect, Reviewer), external systems (GitHub, GitLab, Jira, FluxCD, Kafka, Hatchet, OpenZiti, LiteLLM), and system boundary
- [X] T012 [US1] Add C4 Level 2 (Container) Mermaid diagram to wiki/STRATEGIC-DESIGN.md showing the 5 Rust crates as containers, Kubernetes namespaces (agents, orchestrators, llm-apps, confluent), and inter-container communication
- [X] T013 [US1] Rewrite wiki/TACTICAL-DESIGN.md with C4 Level 3 (Component) Mermaid diagrams showing internal components per crate — agent modules, MCP tools, infrastructure adapters, workflow engines
- [X] T014 [US1] Add Onion Architecture layer diagram to wiki/STRATEGIC-DESIGN.md mapping factory-core (Domain) → factory-application (Application) → factory-infrastructure (Infrastructure) → factory-mcp-server (Interface) → factory-cli (CLI)
- [X] T015 [US1] Add Bounded Context Map Mermaid diagram to wiki/STRATEGIC-DESIGN.md delineating Agent Execution, Mission Orchestration, Infrastructure Integration, and Security/Identity contexts

**Checkpoint**: Stakeholders can navigate from Home.md through BUSINESS-CONTEXT → STRATEGIC-DESIGN → TACTICAL-DESIGN and understand the full system without reading source code

---

## Phase 4: User Story 2 — Deep Tactical Code & Class Architecture with UML 2.0 (Priority: P2)

**Goal**: Provide rigorous UML 2.0 class diagrams, trait hierarchies, and sequence/execution flow diagrams for every crate, so developers understand types, state machines, and call paths

**Independent Test**: Each crate documentation page contains at least one classDiagram and one sequenceDiagram in valid Mermaid; all public structs, enums, and traits from the AST inventory (T006) are referenced

### Implementation for User Story 2

#### factory-core Crate Documentation

- [X] T016 [P] [US2] Rewrite wiki/crates_factory-core_src_lib.md with full crate overview, module tree, and re-export map
- [X] T017 [P] [US2] Rewrite wiki/crates_factory-core_src_error.md with UML class diagram for FactoryError enum variants and thiserror derivations
- [X] T018 [P] [US2] Rewrite wiki/crates_factory-core_src_executor.md with UML class diagram for TaskExecutor trait and Mermaid sequence diagram for task dispatch flow
- [X] T019 [P] [US2] Rewrite wiki/crates_factory-core_src_security.md with UML class diagram for security primitives and trust boundary model
- [X] T020 [P] [US2] Rewrite wiki/crates_factory-core_src_security_nhi.md with Ed25519 NHI credential issuance/verification sequence diagram

#### factory-application Crate Documentation

- [X] T021 [P] [US2] Rewrite wiki/crates_factory-application_src_lib.md with crate overview and module dependency graph
- [X] T022 [P] [US2] Rewrite wiki/crates_factory-application_src_agents_mod.md with UML class diagram showing all 6 agent trait implementations and shared interface
- [X] T023 [P] [US2] Rewrite wiki/crates_factory-application_src_agents_rustant.md with RustantAgent behavioral specification, tool authorization table, and planning sequence diagram
- [X] T024 [P] [US2] Rewrite wiki/crates_factory-application_src_agents_zeroclaw.md with ZeroClawAgent TDD loop state machine (Red → Green → Refactor) and sandbox execution sequence
- [X] T025 [P] [US2] Rewrite wiki/crates_factory-application_src_agents_auditor.md with AuditorAgent security review flow and SAST gate integration
- [X] T026 [P] [US2] Rewrite wiki/crates_factory-application_src_agents_finops.md with FinOpsAgent token tracking, budget enforcement, and hardstop sequence diagram
- [X] T027 [P] [US2] Rewrite wiki/crates_factory-application_src_agents_qa_observer.md with QAObserverAgent test coverage analysis and quality gate enforcement
- [X] T028 [P] [US2] Rewrite wiki/crates_factory-application_src_agents_doc_agent.md with DocAgent wiki generation, OSR calculation, and doc-sync trigger flow
- [X] T029 [P] [US2] Rewrite wiki/crates_factory-application_src_workflows_mod.md with workflow module overview and inter-workflow dependency map
- [X] T030 [P] [US2] Rewrite wiki/crates_factory-application_src_workflows_autonomous_mission.md with 6-phase Hatchet DAG sequence diagram (Ingestion → Plan → Code → Validation → Review → Delivery)
- [X] T031 [P] [US2] Rewrite wiki/crates_factory-application_src_workflows_circuit_breaker.md with Aethelgard circuit breaker state machine (Active → Retry → Deadlock → Stuck → Override)
- [X] T032 [P] [US2] Rewrite wiki/crates_factory-application_src_workflows_develop_task.md with TDD task development sequence and sandbox lifecycle
- [X] T033 [P] [US2] Rewrite wiki/crates_factory-application_src_workflows_pipeline_remediation.md with CI/CD pipeline auto-remediation flow diagram
- [X] T034 [P] [US2] Rewrite wiki/crates_factory-application_src_bridge_mod.md with bridge module overview
- [X] T035 [P] [US2] Rewrite wiki/crates_factory-application_src_bridge_adk_driver.md with ADK driver UML and integration sequence
- [X] T036 [P] [US2] Rewrite wiki/crates_factory-application_src_bridge_kafka_bridge.md with Kafka bridge UML and message flow
- [X] T037 [P] [US2] Rewrite wiki/crates_factory-application_src_bridge_state.md with state management UML class diagram
- [X] T038 [P] [US2] Rewrite wiki/crates_factory-application_src_poller_service.md with outbound poller ingestion sequence diagram and GitHub/GitLab/Jira polling flow
- [X] T039 [P] [US2] Rewrite wiki/crates_factory-application_src_telemetry_export.md with telemetry export UML and data pipeline diagram
- [X] T040 [P] [US2] Rewrite wiki/crates_factory-application_src_utils_mod.md with utils module overview
- [X] T041 [P] [US2] Rewrite wiki/crates_factory-application_src_utils_osr.md with OSR calculator class diagram and verification algorithm flow

#### factory-infrastructure Crate Documentation

- [X] T042 [P] [US2] Rewrite wiki/crates_factory-infrastructure_src_lib.md with crate overview and adapter registry
- [X] T043 [P] [US2] Rewrite wiki/crates_factory-infrastructure_src_github.md with GitHub adapter UML class diagram and API sequence flows
- [X] T044 [P] [US2] Rewrite wiki/crates_factory-infrastructure_src_gitlab.md with GitLab adapter UML class diagram and MR/pipeline interaction
- [X] T045 [P] [US2] Rewrite wiki/crates_factory-infrastructure_src_jira.md with Jira adapter UML and issue lifecycle sequence
- [X] T046 [P] [US2] Rewrite wiki/crates_factory-infrastructure_src_kafka.md with Kafka producer/consumer UML and KRaft topic architecture
- [X] T047 [P] [US2] Rewrite wiki/crates_factory-infrastructure_src_ziti.md with OpenZiti zero-trust overlay UML and network path diagram
- [X] T048 [P] [US2] Rewrite wiki/crates_factory-infrastructure_src_vault.md with Vault secret management UML and credential rotation flow
- [X] T049 [P] [US2] Rewrite wiki/crates_factory-infrastructure_src_r2r.md with R2R GraphRAG adapter UML and knowledge retrieval sequence
- [X] T050 [P] [US2] Rewrite wiki/crates_factory-infrastructure_src_aethalgard.md with Aethelgard circuit breaker infrastructure adapter UML
- [X] T051 [P] [US2] Rewrite wiki/crates_factory-infrastructure_src_mcp_client.md with MCP client adapter UML class diagram and tool invocation protocol
- [X] T052 [P] [US2] Rewrite wiki/crates_factory-infrastructure_src_pipeline_classifier.md with pipeline classifier UML and CI error taxonomy
- [X] T053 [P] [US2] Rewrite wiki/crates_factory-infrastructure_src_s3.md with S3 storage adapter UML
- [X] T054 [P] [US2] Rewrite wiki/crates_factory-infrastructure_src_sentry.md with Sentry error tracking adapter UML
- [X] T055 [P] [US2] Rewrite wiki/crates_factory-infrastructure_src_security_validator.md with security validation UML and Ed25519 verification sequence
- [X] T056 [P] [US2] Rewrite wiki/crates_factory-infrastructure_src_cursor_store.md with cursor store UML for polling state persistence
- [X] T057 [P] [US2] Rewrite wiki/crates_factory-infrastructure_src_git_poller.md with git poller UML and repository change detection flow

#### factory-mcp-server Crate Documentation

- [X] T058 [P] [US2] Rewrite wiki/crates_factory-mcp-server_src_lib.md with MCP server overview, tool registry, and protocol UML
- [X] T059 [P] [US2] Rewrite wiki/crates_factory-mcp-server_src_main.md with server startup sequence and configuration
- [X] T060 [P] [US2] Rewrite wiki/crates_factory-mcp-server_src_protocol.md with MCP protocol UML class diagram and JSON-RPC sequence
- [X] T061 [P] [US2] Rewrite wiki/crates_factory-mcp-server_src_sandbox.md with sandbox pod lifecycle UML and gVisor isolation architecture
- [X] T062 [P] [US2] Rewrite wiki/crates_factory-mcp-server_src_feedback_route.md with feedback route handler UML
- [X] T063 [P] [US2] Rewrite wiki/crates_factory-mcp-server_src_scratch.md with scratch workspace UML
- [X] T064 [P] [US2] Rewrite wiki/crates_factory-mcp-server_src_skills_context.md with skills context manager UML and Spec Kit integration
- [X] T065 [P] [US2] Rewrite wiki/crates_factory-mcp-server_src_skills_mod.md with skills module overview
- [X] T066 [P] [US2] Rewrite wiki/crates_factory-mcp-server_src_tools_mod.md with tool registration UML and dynamic dispatch flow
- [X] T067 [P] [US2] Rewrite wiki/crates_factory-mcp-server_src_tools_bridge.md with bridge tool UML
- [X] T068 [P] [US2] Rewrite wiki/crates_factory-mcp-server_src_tools_execute_code.md with code execution tool UML and sandbox invocation sequence
- [X] T069 [P] [US2] Rewrite wiki/crates_factory-mcp-server_src_tools_index_code.md with code indexing tool UML and R2R ingestion flow
- [X] T070 [P] [US2] Rewrite wiki/crates_factory-mcp-server_src_tools_launch_sandbox_pod.md with sandbox pod launch tool UML and K8s Job/Pod lifecycle
- [X] T071 [P] [US2] Rewrite wiki/crates_factory-mcp-server_src_tools_plan_mission.md with mission planning tool UML
- [X] T072 [P] [US2] Rewrite wiki/crates_factory-mcp-server_src_tools_retrieve_context.md with context retrieval tool UML and GraphRAG query sequence
- [X] T073 [P] [US2] Rewrite wiki/crates_factory-mcp-server_src_tools_run_tests.md with test runner tool UML and result parsing flow
- [X] T074 [P] [US2] Rewrite wiki/crates_factory-mcp-server_src_tools_search_jira.md with Jira search tool UML
- [X] T075 [P] [US2] Rewrite wiki/crates_factory-mcp-server_src_tools_security_review.md with security review tool UML and Semgrep SAST integration
- [X] T076 [P] [US2] Rewrite wiki/crates_factory-mcp-server_src_tools_spec_kit_tool.md with Spec Kit tool UML and SDD workflow bridge
- [X] T077 [P] [US2] Rewrite wiki/crates_factory-mcp-server_src_tools_spec_kit_tasks_to_issues.md with tasks-to-issues tool UML and GitHub issue creation sequence
- [X] T078 [P] [US2] Rewrite wiki/crates_factory-mcp-server_src_tools_update_mission_status.md with mission status update tool UML
- [X] T079 [P] [US2] Rewrite wiki/spec_kit_tool.md with comprehensive Spec Kit MCP tool documentation

#### factory-cli Crate Documentation

- [X] T080 [P] [US2] Rewrite wiki/crates_factory-cli_src_main.md with CLI entry point UML and argument parsing flow
- [X] T081 [P] [US2] Rewrite wiki/crates_factory-cli_src_bin_trigger_mission.md with trigger_mission binary documentation and usage examples
- [X] T082 [P] [US2] Rewrite wiki/crates_factory-cli_src_bin_run_functional_suite.md with functional test suite binary documentation

#### Cross-Crate Agent Specification Page

- [X] T083 [US2] Rewrite wiki/AGENT-SPECIFICATIONS.md with consolidated UML class diagram for all 6 agents showing shared AgentTrait interface, tool authorization tables, and comparative behavioral matrix

**Checkpoint**: Every crate module has a dedicated OKF source map with UML class/sequence diagrams; AGENT-SPECIFICATIONS.md provides a unified cross-crate agent view

---

## Phase 5: User Story 3 — Operational Use Cases, User Guides & Step-by-Step Manuals (Priority: P3)

**Goal**: Provide clear operational guides for DevOps engineers covering all supported use cases (issue-triggered missions, PR interactive comments, CI pipeline remediation, CLI triggers)

**Independent Test**: An operator can follow USER-MANUAL.md to trigger an autonomous mission; all bot commands (@darkgravity /status, /interact, /refine, /validate) are documented with expected behaviors

### Implementation for User Story 3

- [X] T084 [US3] Rewrite wiki/USER-MANUAL.md with comprehensive step-by-step guides for: (a) issue-triggered autonomous missions with label/resource-limit syntax, (b) PR interactive directive commands with examples, (c) CLI trigger_mission usage, (d) environment configuration with LiteLLM model switching, (e) Kubernetes manifest setup
- [X] T085 [US3] Rewrite wiki/PRODUCTION-OPERATIONS.md with: (a) FluxCD GitOps deployment topology, (b) Kubernetes namespace layout with node affinity/taints, (c) LiteLLM routing configuration, (d) troubleshooting runbook for common failure modes (pod crash loops, Kafka lag, Ziti tunnel drops, Hatchet DAG failures)
- [X] T086 [US3] Rewrite wiki/EXPERIMENT-LIFECYCLE.md with comprehensive 6-phase Hatchet DAG documentation: phase-by-phase breakdown with responsible agent, input/output artifacts, success/failure transitions, and Mermaid sequence diagram
- [X] T087 [P] [US3] Rewrite wiki/EXPERIMENT-LOGS.md with experiment log format documentation, telemetry schema, and dashboard integration guide
- [X] T088 [P] [US3] Rewrite wiki/Test_Plan_Report.md with test strategy documentation covering unit, integration, functional, and SAST security testing across all crates
- [X] T089 [P] [US3] Rewrite wiki/src-mission.md with mission data model documentation and event-driven architecture flow

**Checkpoint**: An operator with zero prior context can deploy, configure, trigger, and troubleshoot autonomous missions using only wiki documentation

---

## Phase 6: User Story 4 — End-to-End Mission Lifecycle & Zero Trust Security Governance (Priority: P4)

**Goal**: Document the complete autonomous mission lifecycle with defense-in-depth security model, HITL governance vertices, and R&D compliance audit trail

**Independent Test**: A security auditor can trace Ed25519 credential creation through ingestion to sandbox execution; all 4 HITL governance vertices are formally specified; Hazitek/EU AI Act compliance evidence is documented

### Implementation for User Story 4

- [X] T090 [US4] Create new wiki/SECURITY-ARCHITECTURE.md with: (a) Zero Trust security model overview, (b) OpenZiti overlay networking architecture with Mermaid diagram, (c) Ed25519 NHI credential issuance/verification lifecycle sequence diagram, (d) sandbox containment architecture (gVisor, resource quotas, network isolation), (e) Semgrep SAST gate integration, (f) circuit breaker anti-deadlock safeguards, (g) STRIDE threat model mapping
- [X] T091 [US4] Create new wiki/HITL-GOVERNANCE.md with: (a) 4-vertex governance mesh specification (Strategic Injection, Sprint Mobilization, Exception Override, Categorical Scrutiny), (b) human roles and responsibilities matrix, (c) trigger conditions and enforcement mechanisms, (d) docs-to-wiki.yml sync pipeline documentation, (e) governance Mermaid state diagram
- [X] T092 [US4] Create new wiki/COMPLIANCE-AUDIT.md with: (a) Hazitek 2026 / SPRI / EU AI Act Art. 12 & 14 compliance framework, (b) automated telemetry packager specification (AST deltas, compute core-hours, FinOps token logs), (c) Ed25519 cryptographic NHI claims and verifiable credentials, (d) audit trail data model and export format
- [X] T093 [US4] Rewrite wiki/VERIFICATION-TRIAD.md with: (a) Logical verification gate (unit/integration tests), (b) Architectural verification gate (linting, dependency analysis), (c) Security verification gate (SAST, NHI credential checks), (d) Mermaid diagram showing triad gate pipeline
- [X] T094 [US4] Rewrite wiki/INFRASTRUCTURE-ADAPTERS.md with: (a) Kafka KRaft event bus architecture, (b) R2R GraphRAG knowledge retrieval, (c) OpenZiti zero-trust networking, (d) Vault secret management, (e) Sentry error tracking, (f) S3 artifact storage — each with UML class diagrams and integration sequence diagrams

**Checkpoint**: Security auditor can trace the complete trust chain; compliance officer can extract R&D evidence; all 4 HITL vertices are formally modeled

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Final quality assurance, link validation, navigation coherence, and OSR quality gate

- [X] T095 Update wiki/index.md with comprehensive master index linking all 84+ wiki files organized by category (Navigation, Business, Architecture, Agents, Lifecycle, Operations, Security, Compliance, Crate References)
- [X] T096 Update wiki/Home.md with final navigation links reflecting all new pages (SECURITY-ARCHITECTURE.md, HITL-GOVERNANCE.md, COMPLIANCE-AUDIT.md)
- [X] T097 Update wiki/_Sidebar.md with final hierarchical sidebar navigation including all pages and sections
- [X] T098 [P] Validate all internal markdown cross-references — zero broken links across all wiki/ files
- [X] T099 [P] Validate all Mermaid diagram blocks for syntax correctness — zero rendering errors
- [X] T100 [P] Verify OKF frontmatter completeness on all applicable wiki pages
- [X] T101 Run quickstart.md validation scenarios (7 scenarios + OSR gate) per specs/007-professional-wiki-docs/quickstart.md
- [X] T102 Final review: Verify wiki contract compliance against specs/007-professional-wiki-docs/contracts/wiki-contract.json

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup (Phase 1) completion — BLOCKS all user stories (T006 AST inventory required)
- **User Story 1 (Phase 3)**: Depends on Foundational (Phase 2) — architectural context needed first
- **User Story 2 (Phase 4)**: Depends on Foundational (Phase 2) and benefits from US1 architectural context
- **User Story 3 (Phase 5)**: Depends on Foundational (Phase 2); can run in parallel with US2
- **User Story 4 (Phase 6)**: Depends on Foundational (Phase 2); can run in parallel with US2/US3
- **Polish (Phase 7)**: Depends on ALL user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational — No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational — Benefits from US1 C4 diagrams but independently testable
- **User Story 3 (P3)**: Can start after Foundational — Independent from US1/US2
- **User Story 4 (P4)**: Can start after Foundational — Independent from US1/US2/US3

### Within Each User Story

- Core document content before cross-references
- Mermaid diagrams embedded inline during content generation
- Per-crate OKF maps can be parallelized ([P] marked)

### Parallel Opportunities

- All Setup T003–T005 can run in parallel
- All Foundational T006–T009 can run in parallel
- **Within US2**: All 66 crate source map rewrites (T016–T082) are fully parallelizable — different files, no dependencies
- US3 and US4 can run concurrently with US2
- All Polish validation tasks T098–T100 can run in parallel

---

## Parallel Example: User Story 2

```bash
# Launch all factory-core crate docs in parallel:
Task: "T016 Rewrite wiki/crates_factory-core_src_lib.md"
Task: "T017 Rewrite wiki/crates_factory-core_src_error.md"
Task: "T018 Rewrite wiki/crates_factory-core_src_executor.md"
Task: "T019 Rewrite wiki/crates_factory-core_src_security.md"
Task: "T020 Rewrite wiki/crates_factory-core_src_security_nhi.md"

# Launch all agent docs in parallel:
Task: "T023 Rewrite wiki/crates_factory-application_src_agents_rustant.md"
Task: "T024 Rewrite wiki/crates_factory-application_src_agents_zeroclaw.md"
Task: "T025 Rewrite wiki/crates_factory-application_src_agents_auditor.md"
Task: "T026 Rewrite wiki/crates_factory-application_src_agents_finops.md"
Task: "T027 Rewrite wiki/crates_factory-application_src_agents_qa_observer.md"
Task: "T028 Rewrite wiki/crates_factory-application_src_agents_doc_agent.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001–T005)
2. Complete Phase 2: Foundational (T006–T009)
3. Complete Phase 3: User Story 1 (T010–T015) — C4 diagrams + business context
4. **STOP and VALIDATE**: Run quickstart.md Scenarios 1–2
5. Wiki is immediately useful for stakeholders

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add US1 (C4 Architecture) → Validate → Deploy (MVP!)
3. Add US2 (UML Code Docs) → Validate → Deploy — Full developer reference
4. Add US3 (Operations) → Validate → Deploy — Operator-ready
5. Add US4 (Security/Compliance) → Validate → Deploy — Audit-ready
6. Polish (Phase 7) → Final validation → PR for human review

### Parallel Team Strategy

With multiple agents/developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Agent A: User Story 1 (6 tasks, strategic/architectural)
   - Agent B: User Story 2 (68 tasks, crate-level OKF rewrites — highly parallelizable)
   - Agent C: User Story 3 (6 tasks, operational docs)
   - Agent D: User Story 4 (5 tasks, security/compliance)
3. Stories complete and integrate via Phase 7 Polish

---

## Notes

- [P] tasks = different files, no dependencies between them
- [Story] label maps task to specific user story for traceability
- Each user story is independently completable and testable
- All content generation reads source code (read-only) and writes to wiki/ (write-only)
- Commit after each completed phase or logical task group
- Stop at any checkpoint to validate story independently
- Target: 102 total tasks across 7 phases
