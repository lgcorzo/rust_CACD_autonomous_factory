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

> *Related: [s3.rs](crates_factory-infrastructure_src_s3.md) · [Tactical Design](TACTICAL-DESIGN.md)*
