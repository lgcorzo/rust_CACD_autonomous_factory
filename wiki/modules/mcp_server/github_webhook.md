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
verified: "false"
# 2026-11-10 DG-RVP-018 A2: auto-generated "true" withdrawn; pending human review
last_verified_commit: "fbdc5848"
---

# factory-mcp-server::github_webhook — GitHub Webhook Ingress Route

> **Source**: `crates/factory-mcp-server/src/github_webhook.rs`  
> **Layer**: Interface  
> **Role**: Axum route for GitHub webhook events. The router registers `POST /webhooks/github` unconditionally, so it is enabled at the application level by default. No `Service` or `Ingress` manifest is present in `deploy/` or `config/`, so external exposure is **TO CONFIRM**.

The route is included when the application router is built. Protected mode is enabled when `FACTORY_ENV=production`, the production feature is active, or `MCP_API_BEARER_TOKEN` is configured. In protected mode, a webhook secret is required. In unprotected mode, an empty webhook secret does not require signature validation. The default local configuration is therefore not a signed-webhook requirement; this is a code issue to resolve separately.

---

## Webhook Verification Flow

```mermaid
sequenceDiagram
    autonumber
    participant GH as GitHub Webhook
    participant Route as handle_github_webhook
    participant HMAC as HMAC-SHA256 Verification
    participant DAG as Hatchet Mission / Poller

    GH->>Route: POST /webhooks/github (X-Hub-Signature-256)
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

The HTTP server binds to `0.0.0.0`; that bind address does not itself establish that a Kubernetes Service, Ingress, or external route exposes the endpoint.

---

> *Related: [McpServer](lib.md) · [Tactical Design](../../architecture/tactical_design.md)*
