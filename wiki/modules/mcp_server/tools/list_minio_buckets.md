---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-mcp-server::tools::list_minio_buckets — MinIO Buckets MCP Tool"
source_path: "crates/factory-mcp-server/src/tools/list_minio_buckets.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-mcp-server::tools::list_minio_buckets — MinIO Buckets MCP Tool."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-mcp-server::tools::list_minio_buckets — MinIO Buckets MCP Tool

> **Source**: `crates/factory-mcp-server/src/tools/list_minio_buckets.rs`  
> **Layer**: Interface  
> **Role**: MCP tool providing agents with S3 / MinIO storage bucket discovery.

---

## Tool Specification

- **Tool Name**: `list_minio_buckets`
- **Description**: Enumerates all available S3/MinIO buckets configured for the autonomous factory artifact store.
- **Parameters**: None.

---

> *Related: [s3.rs](../../infrastructure/s3.md) · [Tactical Design](../../../architecture/tactical_design.md)*
