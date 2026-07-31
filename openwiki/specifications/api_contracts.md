---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "api_contracts"
title: "ISO 15289 Specification: Complete API & Interface Contracts"
description: "Specification of core Rust traits, MCP JSON-RPC schemas, and proto interface definitions."
tags: ["iso15289", "api", "contracts", "traits", "mcp"]
timestamp: "2026-07-31T16:35:00Z"
---

# ISO 15289 Specification: Complete API & Interface Contracts

## 1. Rust Trait Contracts (`factory-core` & `factory-infrastructure`)

### 1. `CodeSurgeryExecutor` Trait
- **Location**: `crates/factory-core/src/executor.rs:L19-L27`
- **Visibility**: Public (`pub`)
- **Async Runtime**: `#[async_trait]` (`Send + Sync`)

```rust
#[async_trait]
pub trait CodeSurgeryExecutor: Send + Sync {
    async fn apply_patch(
        &self,
        mission_id: &str,
        patch: &SurgicalPatch,
    ) -> Result<ExecutionResult, FactoryError>;

    async fn verify_syntax(&self, file_path: &std::path::Path) -> Result<bool, FactoryError>;
}
```

---

### 2. `SecurityValidator` Trait
- **Location**: `crates/factory-core/src/security.rs:L15-L19`
- **Visibility**: Public (`pub`)

```rust
#[async_trait]
pub trait SecurityValidator {
    async fn validate_signature(&self, data: &[u8], signature: &str) -> Result<bool>;
    async fn audit_content(&self, content: &str) -> Result<AuditResult>;
}
```

---

### 3. `SandboxDriver` Trait
- **Location**: `crates/factory-mcp-server/src/sandbox.rs:L15-L28`
- **Visibility**: Public (`pub`)

```rust
#[async_trait]
pub trait SandboxDriver: Send + Sync {
    async fn execute(&self, code: &str, language: &str) -> anyhow::Result<ExecutionResult>;
    async fn execute_surgery(
        &self,
        id: &str,
        patch: &factory_core::executor::SurgicalPatch,
    ) -> factory_core::error::Result<factory_core::executor::ExecutionResult>;
}
```

---

### 4. `AethalgardClient` Trait
- **Location**: `crates/factory-infrastructure/src/aethalgard.rs:L5-L9`
- **Visibility**: Public (`pub`)

```rust
#[async_trait]
pub trait AethalgardClient: Send + Sync {
    async fn notify_remediation(&self, mission_id: &str, error_details: &str) -> anyhow::Result<()>;
}
```

---

## 2. MCP Server JSON-RPC Tool Contracts

The MCP server exposes standard JSON-RPC tools for agent interactions (`crates/factory-mcp-server/src/tools/`):

1. **`execute_code`**: Runs Python, Rust, Go, or TypeScript code blocks inside configured sandboxes.
2. **`launch_sandbox_pod`**: Dynamically provisions Kubernetes gVisor pods for un-trusted code executions.
3. **`index_code`**: Triggers local AST graph extraction and updates graphify nodes.
4. **`plan_mission`**: Submits mission task trees and agent assignments.
5. **`search_jira`**: Queries external Jira API endpoints.
6. **`security_review`**: Runs SAST rules and content audits.
