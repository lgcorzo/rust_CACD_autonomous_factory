---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-mcp-server::tools — Tool Dispatch Registry"
source_path: "crates/factory-mcp-server/src/tools/mod.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-mcp-server::tools — Tool Dispatch Registry."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-mcp-server::tools — Tool Dispatch Registry

> **Source**: `crates/factory-mcp-server/src/tools/mod.rs`  
> **Layer**: Interface  
> **Role**: Tool dispatch registry managing dynamic tool registration, JSON-RPC schema contracts, authorization checks, and request routing to concrete tool implementations.

---

## Tool Trait & Dispatch Architecture

```mermaid
classDiagram
    class Tool {
        <<trait>>
        +name() String
        +description() String
        +input_schema() Value
        +call(params: Value) Result~CallToolResult~
    }

    class BridgeTool
    class DeepResearchTool
    class ExecuteCodeTool
    class GetFactoryStatusTool
    class IndexCodeTool
    class InspectKafkaTopicTool
    class LaunchSandboxPodTool
    class ListMinioBucketsTool
    class ListMinioObjectsTool
    class PlanMissionTool
    class RetrieveContextTool
    class RunTestsTool
    class SearchJiraTool
    class SecurityReviewTool
    class SpecKitTasksToIssuesTool
    class SpecKitTool
    class UpdateMissionStatusTool

    Tool <|.. BridgeTool
    Tool <|.. DeepResearchTool
    Tool <|.. ExecuteCodeTool
    Tool <|.. GetFactoryStatusTool
    Tool <|.. IndexCodeTool
    Tool <|.. InspectKafkaTopicTool
    Tool <|.. LaunchSandboxPodTool
    Tool <|.. ListMinioBucketsTool
    Tool <|.. ListMinioObjectsTool
    Tool <|.. PlanMissionTool
    Tool <|.. RetrieveContextTool
    Tool <|.. RunTestsTool
    Tool <|.. SearchJiraTool
    Tool <|.. SecurityReviewTool
    Tool <|.. SpecKitTasksToIssuesTool
    Tool <|.. SpecKitTool
    Tool <|.. UpdateMissionStatusTool
```

## Registered MCP Tools Catalog

| Tool Name | Module | Primary Purpose | Authorized Agents |
|:---|:---|:---|:---|
| `bridge` | [`bridge.rs`](bridge.md) | Bidirectional bridge communication | All Agents |
| `deep_research` | [`deep_research_tool.rs`](deep_research_tool.md) | Multi-step Hatchet background research | Rustant, DocAgent |
| `execute_code` | [`execute_code.rs`](execute_code.md) | Surgical code execution in sandbox | ZeroClaw |
| `get_factory_status` | [`get_factory_status.rs`](get_factory_status.md) | System health and metrics inspection | All Agents |
| `index_code` | [`index_code.rs`](index_code.md) | AST parsing and vector indexing | ZeroClaw, DocAgent |
| `inspect_kafka_topic` | [`inspect_kafka_topic.rs`](inspect_kafka_topic.md) | Live Kafka topic diagnostics | QAObserver, Auditor |
| `launch_sandbox_pod` | [`launch_sandbox_pod.rs`](launch_sandbox_pod.md) | Provision gVisor/K8s sandbox container | ZeroClaw |
| `list_minio_buckets` | [`list_minio_buckets.rs`](list_minio_buckets.md) | Storage bucket discovery | All Agents |
| `list_minio_objects` | [`list_minio_objects.rs`](list_minio_objects.md) | S3 artifact object listing | All Agents |
| `plan_mission` | [`plan_mission.rs`](plan_mission.md) | Decomposes mission requirements into DAG | Rustant |
| `retrieve_context` | [`retrieve_context.rs`](retrieve_context.md) | R2R hybrid semantic context retrieval | All Agents |
| `run_tests` | [`run_tests.rs`](run_tests.md) | Automated test execution in sandbox | QAObserver, ZeroClaw |
| `search_jira` | [`search_jira.rs`](search_jira.md) | Enterprise Jira issue queries | Rustant |
| `security_review` | [`security_review.rs`](security_review.md) | Static SAST & secret analysis | Auditor |
| `spec_kit_tasks_to_issues` | [`spec_kit_tasks_to_issues.rs`](spec_kit_tasks_to_issues.md) | Converts tasks.md into GitHub issues | Rustant |
| `spec_kit_tool` | [`spec_kit_tool.rs`](spec_kit_tool.md) | Spec-driven code intelligence | Rustant, ZeroClaw |
| `update_mission_status` | [`update_mission_status.rs`](update_mission_status.md) | State transitions in DAG | All Agents |

---

> *Related: [lib.rs](../lib.md) · [Tactical Design](../../../architecture/tactical_design.md)*
