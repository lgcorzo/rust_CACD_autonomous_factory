---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-mcp-server::tools::list_minio_objects — MinIO Objects MCP Tool"
source_path: "crates/factory-mcp-server/src/tools/list_minio_objects.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-mcp-server::tools::list_minio_objects — MinIO Objects MCP Tool."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-mcp-server::tools::list_minio_objects — MinIO Objects MCP Tool

> **Source**: `crates/factory-mcp-server/src/tools/list_minio_objects.rs`  
> **Layer**: Interface  
> **Role**: MCP tool providing agents with inspection of artifact objects within specific S3/MinIO storage buckets.

---

## Tool Specification

- **Tool Name**: `list_minio_objects`
- **Description**: Lists object keys and metadata inside a given bucket with optional prefix filtering.
- **Parameters**:
  - `bucket` (string, required): S3 bucket name.
  - `prefix` (string, optional): Key prefix filter.

---

> *Related: [s3.rs](../../infrastructure/s3.md) · [Tactical Design](../../../architecture/tactical_design.md)*
