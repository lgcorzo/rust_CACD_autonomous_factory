---
iso_doc_type: "Description"
iso_viewpoint: "SequenceView"
type: "architecture"
title: "ISO 42010 Sequence View: Execution Flows & Interaction Diagrams"
description: "Sequence View depicting message dispatches, code surgery execution, sandbox runs, and Aethalgard remediation."
tags: ["iso42010", "sequence_view", "sequence_diagram", "runtime_flows"]
timestamp: "2026-07-31T16:35:00Z"
---

# ISO 42010 Sequence View: Execution Flows & Interaction Diagrams

## 1. Code Surgery Execution Flow

This sequence depicts an autonomous agent requesting surgical patch execution through the MCP server interface down to the `NativeSurgerySandboxDriver` and `CodeSurgeryExecutor`:

```mermaid
sequenceDiagram
    autonumber
    participant Agent as Autonomous Agent / LLM Client
    participant MCP as MCP Protocol Server (factory-mcp-server)
    participant Driver as NativeSurgerySandboxDriver
    participant Executor as CodeSurgeryExecutor
    participant Validator as Ed25519SecurityValidator
    participant Audit as Aethalgard Client

    Agent->>MCP: Call execute_surgery(mission_id, SurgicalPatch)
    MCP->>Validator: validate_signature(data, signature)
    alt Signature Valid
        Validator-->>MCP: Ok(true)
        MCP->>Driver: execute_surgery(mission_id, patch)
        Driver->>Executor: apply_patch(mission_id, patch)
        Executor->>Executor: verify_syntax(file_path)
        alt Syntax Verification Pass
            Executor-->>Driver: Ok(ExecutionResult { success: true, commit_sha, lines_modified })
            Driver-->>MCP: Ok(ExecutionResult)
            MCP-->>Agent: JSON-RPC Success Response
        else Syntax Verification Failure
            Executor-->>Driver: Err(FactoryError::SyntaxError)
            Driver->>Audit: notify_remediation(mission_id, error_details)
            Audit-->>Driver: Ok(())
            Driver-->>MCP: Err(FactoryError)
            MCP-->>Agent: JSON-RPC Error Response
        end
    else Signature Invalid
        Validator-->>MCP: Ok(false)
        MCP-->>Agent: Security Error (Unauthorized)
    end
```

---

## 2. gVisor Sandboxed Code Execution Sequence

When untrusted code is executed via the `GvisorK8sDriver`, the system launches a Kubernetes sandboxed pod isolated via gVisor kernel virtualization:

```mermaid
sequenceDiagram
    autonumber
    participant Client as MCP Client
    participant Driver as GvisorK8sDriver
    participant Tool as LaunchSandboxPodTool
    participant Pod as gVisor K8s Sandbox Pod

    Client->>Driver: execute(code, language)
    Driver->>Tool: call({ code, language })
    Tool->>Pod: Launch K8s Pod (runtimeClassName: gvisor)
    Pod->>Pod: Execute Code in Isolated Kernel Sandbox
    Pod-->>Tool: Return stdout / stderr / exit_code
    Tool-->>Driver: McpResult { content, is_error }
    Driver-->>Client: ExecutionResult { stdout, stderr, is_success }
```

---

## 3. Key Interaction Line Citations

- **MCP Tool Dispatch**: `crates/factory-mcp-server/src/sandbox.rs:L35-L47`
- **Signature & Security Audit**: `crates/factory-core/src/security.rs:L31-L54`
- **Aethalgard Remediation Webhook**: `crates/factory-infrastructure/src/aethalgard.rs:L27-L54`
- **Subprocess Driver Timeout Handling**: `crates/factory-mcp-server/src/sandbox.rs:L52-L111`
