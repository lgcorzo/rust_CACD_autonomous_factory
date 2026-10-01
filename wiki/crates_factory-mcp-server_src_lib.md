# factory-mcp-server — Crate Overview

> **Source**: `crates/factory-mcp-server/src/lib.rs`
> **Layer**: Interface (Onion Architecture — outermost presentation layer)

---

## MCP Server Architecture

```mermaid
graph TB
    MAIN["main.rs: Axum HTTP Server"]
    LIB["lib.rs: Server Core"]
    PROTO["protocol.rs: JSON-RPC"]
    TOOLS["tools/mod.rs: Tool Registry"]
    SKILLS["skills/mod.rs: Skill Registry"]
    SBX["sandbox.rs: gVisor Lifecycle"]
    FB["feedback_route.rs: Webhooks"]
    GHW["github_webhook.rs: GitHub Events"]

    MAIN --> LIB
    LIB --> PROTO
    LIB --> TOOLS
    LIB --> SKILLS
    LIB --> SBX
    LIB --> FB
    LIB --> GHW

    TOOLS --> T_EXEC["execute_code.rs"]
    TOOLS --> T_LAUNCH["launch_sandbox_pod.rs"]
    TOOLS --> T_RUN["run_tests.rs"]
    TOOLS --> T_SEC["security_review.rs"]
    TOOLS --> T_PLAN["plan_mission.rs"]
    TOOLS --> T_CTX["retrieve_context.rs"]
    TOOLS --> T_IDX["index_code.rs"]
    TOOLS --> T_JIRA["search_jira.rs"]
    TOOLS --> T_SPEC["spec_kit_tool.rs"]
    TOOLS --> T_STAT["update_mission_status.rs"]
    TOOLS --> T_BRIDGE["bridge.rs"]

    style MAIN fill:#9C27B0,stroke:#6A1B9A,color:#fff
```

## Protocol Stack

| Layer | Component | Protocol |
|:---|:---|:---|
| Transport | Axum HTTP Server | HTTP/1.1, HTTP/2 |
| Framing | JSON-RPC 2.0 | `tools/call`, `tools/list` |
| Security | OpenZiti + NHI | Ed25519 + Zero Trust |
| Execution | gVisor Sandbox | K8s Job + RuntimeClass |

---

> *Related: [Tactical Design](TACTICAL-DESIGN.md) · [tools/mod.rs](crates_factory-mcp-server_src_tools_mod.md)*
