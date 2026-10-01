---
name: wiki-iso-documentation-standard
description: "Defines the standard ISO/IEC/IEEE 42010:2022 and ISO/IEC/IEEE 15289:2019 architecture documentation rules, directory structure, frontmatter schemas, UML 2.0 Mermaid requirements, and relative link integrity standards for the rust_CACD_autonomous_factory wiki."
---

# Skill: wiki-iso-documentation-standard

## 1. Role & Objective

This skill governs the structure, organization, formatting, and quality of all technical documentation in the `rust_CACD_autonomous_factory` repository under `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/wiki`.

All documentation artifacts must comply with:
- **ISO/IEC/IEEE 42010:2022**: Systems and software engineering — Architecture description.
- **ISO/IEC/IEEE 15289:2019**: Systems and software engineering — Content of life-cycle information items.
- **ISO/IEC 25010:2023**: Systems and software engineering — Systems and software Quality Requirements and Evaluation (SQuaRE).
- **ISO/IEC/IEEE 29119**: Software testing standards.

---

## 2. Canonical Directory Structure

The documentation inside `wiki/` must adhere strictly to the following directory layout:

```text
wiki/
├── Home.md                           # Master Wiki Homepage
├── README.md                         # Repository Documentation Guide
├── _Sidebar.md                       # Hierarchical Gollum/GitHub Sidebar
├── index.md                          # Master Viewpoint Index (ISO 42010 AD)
├── GLOSSARY.md                       # Ubiquitous Language & Domain Acronyms
│
├── architecture/                     # ISO 42010 Architecture Description Viewpoints
│   ├── business_context.md           # ContextView: Problem, ROI, KPIs, Mission Lifecycle
│   ├── strategic_design.md           # ContextView: C4 Level 1-2, Onion Architecture, Bounded Contexts
│   ├── tactical_design.md            # ComponentView: C4 Level 3, Hatchet DAG mapping, Tool Registry
│   ├── agent_specifications.md       # ComponentView: Consolidated 6 agent specs & tool matrix
│   ├── runtime_sequences.md          # SequenceView: 6-Phase Hatchet DAG & agent interaction flows
│   ├── infrastructure_adapters.md    # ComponentView: Kafka, R2R, OpenZiti, Vault, Sentry, Semantica
│   └── mission_data_model.md         # ComponentView: Event schemas, protobufs, JSON data contracts
│
├── security/                         # ISO 42010 SecurityView & Governance Policies
│   ├── security_architecture.md      # SecurityView: Zero Trust, NHI Ed25519, gVisor isolation
│   ├── hitl_governance.md            # Policy: 4-Vertex Human-in-the-Loop governance mesh
│   ├── verification_triad.md         # Policy: Logical, Architectural, Security quality gates
│   └── compliance_audit.md           # Report: Hazitek 2026, EU AI Act conformity assessment
│
├── operations/                       # ISO 42010 DeploymentView & Operating Procedures
│   ├── user_manual.md                # Procedure: Step-by-step guides for missions, directives, CLI
│   ├── production_operations.md      # DeploymentView: FluxCD GitOps, K8s namespace topology, runbook
│   └── experiment_logs.md            # Report: Telemetry schema, Kafka topics, Prometheus metrics
│
├── quality/                          # ISO 25010 & ISO 29119 Quality Views & Reports
│   └── test_plan_report.md           # QualityView: 18 integration suites & Criterion benchmarks
│
└── modules/                          # 1:1 Structural Mirror of Crate Source Architecture
    ├── core/                         # factory-core (Domain Layer)
    │   ├── lib.md, config.md, error.md, executor.md, security.md, security_nhi.md
    ├── application/                  # factory-application (Application Layer)
    │   ├── lib.md, gitlab_verifier.md, poller_service.md, telemetry_export.md
    │   ├── agents/                   # mod.md, rustant.md, zeroclaw.md, auditor.md, finops.md, qa_observer.md, doc_agent.md
    │   ├── workflows/                # mod.md, autonomous_mission.md, circuit_breaker.md, comment_control.md, deep_research.md, develop_task.md, pipeline_remediation.md
    │   ├── bridge/                   # mod.md, adk_driver.md, kafka_bridge.md, semantica_bridge.md, state.md
    │   └── utils/                    # mod.md, osr.md
    ├── infrastructure/               # factory-infrastructure (Infrastructure Layer)
    │   ├── lib.md, github.md, gitlab.md, jira.md, kafka.md, ziti.md, vault.md, r2r.md, semantica.md, aethalgard.md, mcp_client.md, pipeline_classifier.md, s3.md, sentry.md, security_validator.md, cursor_store.md, git_poller.md
    ├── mcp_server/                   # factory-mcp-server (Interface Layer)
    │   ├── lib.md, main.md, protocol.md, sandbox.md, scratch.md, feedback_route.md, github_webhook.md
    │   ├── skills/                   # mod.md, context.md, spec_kit_tool.md
    │   └── tools/                    # mod.md, bridge.md, deep_research_tool.md, execute_code.md, get_factory_status.md, index_code.md, inspect_kafka_topic.md, launch_sandbox_pod.md, list_minio_buckets.md, list_minio_objects.md, plan_mission.md, retrieve_context.md, run_tests.md, search_jira.md, security_review.md, spec_kit_tasks_to_issues.md, spec_kit_tool.md, update_mission_status.md
    └── cli/                          # factory-cli (Interface Layer)
        └── main.md, trigger_mission.md, run_functional_suite.md, trigger_deep_search.md
```

---

## 3. Mandatory ISO 42010 / 15289 Frontmatter Schema

Every `.md` file in `wiki/` (with the exception of `_Sidebar.md`) must open with standard YAML frontmatter:

```yaml
---
iso_doc_type: "Description"        # ISO 15289 Type: Description | Specification | Plan | Policy | Procedure | Report
iso_viewpoint: "ComponentView"     # ISO 42010 Viewpoint: ArchitectureDescription | ContextView | ComponentView | SequenceView | DeploymentView | SecurityView | QualityView
type: "module"                     # Concept Type: architecture | security | operations | quality | module | guide
title: "Exact Component or Document Title"
source_path: "crates/crate-name/src/path.rs"  # Repository-relative source path (for module pages)
description: "Concise, unambiguous summary of functionality, responsibility, and scope."
tags: ["iso42010", "okf", "crate-name"]
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "8e57bb0a"
---
```

### ISO 15289 Document Types
- `Description`: High-level overviews, architectural descriptions, bounded context summaries.
- `Specification`: AST contracts, structs, traits, tool schemas, and data model definitions.
- `Plan`: Roadmaps, testing plans, execution strategies.
- `Policy`: Governance rules, security constraints, human-in-the-loop policies.
- `Procedure`: Operational runbooks, user manuals, deployment steps.
- `Report`: Quality assessments, benchmark findings, telemetry logs, compliance audits.

### ISO 42010 Viewpoints
- `ArchitectureDescription`: Root architectural indexes and whole-system overviews (`index.md`).
- `ContextView`: System boundaries, domain models, business ROI, user roles.
- `ComponentView`: C4 Level 3 subsystem components, module crates, UML class diagrams.
- `SequenceView`: Runtime interactions, message passing, Hatchet DAG lifecycles.
- `DeploymentView`: FluxCD GitOps, Kubernetes namespaces, runtime topology.
- `SecurityView`: Zero-trust network overlays, NHI credentials, sandbox boundaries.
- `QualityView`: ISO 25010 software quality, test execution pyramid, benchmarks.

---

## 4. Documentation Quality & Readability Standards

To ensure documentation is **understandable, clear, and professional**:

### 1. Executive Summary Blockquote
Under the main `# Title`, every page must feature an executive summary blockquote:
```markdown
> **Source**: `crates/factory-core/src/config.rs`  
> **Layer**: Domain (innermost onion architecture layer)  
> **Role**: Centralized configuration management for LLM model endpoints and LiteLLM proxy resolution.
```

### 2. Mandatory Mermaid.js Visualizations
Never produce purely text-based documentation. Every page must contain at least one appropriate Mermaid diagram:
- **Component / Module Pages**: UML 2.0 Class Diagram (`classDiagram`) showing public structs, enums, traits, methods with visibility markers (`+`, `-`), types, and relationships (`<|--`, `<|..`, `-->`).
- **Workflow / Process Pages**: Sequence Diagram (`sequenceDiagram`) with autonumbering, lifelines, activation blocks, and structured flow controls (`alt`, `loop`, `par`).
- **Architecture Pages**: C4 Model diagrams (`C4Context`, `C4Container`, `C4Component`) or Graph TD/TB execution graphs.

### 3. Concrete Code Signatures & Line Spans
Document concrete Rust types, traits, and error variants directly extracted from source:
- Avoid vague hand-waving descriptions.
- Reference exact project-relative source paths with line spans when relevant (e.g. `crates/factory-core/src/config.rs:L10-L38`).

### 4. Absolute Path Prohibition (Zero `/mnt/` or `/home/`)
- **Strict Rule**: Never write system-specific absolute paths (`/mnt/...`, `/home/...`, `C:\...`) inside documentation.
- Use strictly repository-relative paths (`crates/factory-core/...`) or document-relative markdown links.

### 5. Relative Link Resolution Integrity
- All cross-references between pages must use relative Markdown links that work both in web browsers and local IDE file view.
  - From `modules/application/agents/rustant.md` to `modules/core/lib.md`:  
    `[Core Domain](../../core/lib.md)`
  - From `architecture/strategic_design.md` to `security/hitl_governance.md`:  
    `[HITL Governance](../security/hitl_governance.md)`
- Never use double brackets `[[Page]]`. Use standard GitHub-flavored Markdown: `[Text](relative/path.md)`.

### 6. Zero Truncation & Zero Placeholders
- Placeholders such as `TODO`, `TBD`, `PLACEHOLDER`, `// ... existing code ...`, or `/* rest of method */` are strictly prohibited.
- All diagrams and tables must be fully populated with real codebase entities.

---

## 5. Automated Verification Checklist

Before committing documentation updates, execute the following verification steps:

1. **Verify No Broken Links**:
   ```bash
   python3 -c "
   import os, glob, re
   wiki_dir = 'wiki'
   files = glob.glob(f'{wiki_dir}/**/*.md', recursive=True)
   link_regex = re.compile(r'\[([^\]]+)\]\(([^)]+)\)')
   broken = []
   for fpath in files:
       source_dir = os.path.dirname(fpath)
       with open(fpath, 'r', encoding='utf-8') as fp:
           for idx, line in enumerate(fp, 1):
               for m in link_regex.finditer(line):
                   target = m.group(2).split('#')[0]
                   if not target or target.startswith(('http:', 'https:', 'mailto:')): continue
                   cand = [os.path.normpath(os.path.join(source_dir, target)), os.path.normpath(os.path.join(source_dir, target + '.md'))]
                   if not any(os.path.exists(c) for c in cand): broken.append((fpath, idx, m.group(1), target))
   print(f'Broken links: {len(broken)}')
   assert len(broken) == 0, f'Found {len(broken)} broken links!'
   "
   ```

2. **Verify 1:1 Crate Source Coverage**:
   - Ensure every `.rs` file in `crates/*/src/` has its corresponding `.md` page in `wiki/modules/`.

3. **Verify Frontmatter Compliance**:
   - Ensure every page (except `_Sidebar.md`) begins with valid YAML declaring `iso_doc_type` and `iso_viewpoint`.

4. **Verify Navigation Synchronization**:
   - Ensure `wiki/index.md` and `wiki/_Sidebar.md` include any newly added pages.
