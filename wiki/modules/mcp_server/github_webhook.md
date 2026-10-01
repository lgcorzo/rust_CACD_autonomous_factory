---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-mcp-server::github_webhook — GitHub Webhook Ingress Route"
source_path: "crates/factory-mcp-server/src/github_webhook.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-mcp-server::github_webhook — GitHub Webhook Ingress Route."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-mcp-server::github_webhook — GitHub Webhook Ingress Route

> **Source**: `crates/factory-mcp-server/src/github_webhook.rs`  
> **Layer**: Interface  
> **Role**: HMAC-SHA256 authenticated Axum webhook endpoint listening for GitHub issue and PR comment events to dispatch into the autonomous factory.

---

## Webhook Verification Flow

```mermaid
sequenceDiagram
    autonumber
    participant GH as GitHub Webhook
    participant Route as handle_github_webhook
    participant HMAC as HMAC-SHA256 Verification
    participant DAG as Hatchet Mission / Poller

    GH->>Route: POST /webhook/github (X-Hub-Signature-256)
    Route->>HMAC: verify_github_signature(secret, sig, body)
    alt Signature Invalid
        HMAC-->>Route: false
        Route-->>GH: 401 Unauthorized
    else Signature Valid
        HMAC-->>Route: true
        Route->>DAG: Dispatch mission / directive event
        Route-->>GH: 200 OK {"status": "accepted"}
    end
```

---

> *Related: [McpServer](lib.md) · [Tactical Design](../../architecture/tactical_design.md)*
