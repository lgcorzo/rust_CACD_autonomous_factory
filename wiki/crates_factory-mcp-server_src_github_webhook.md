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

> *Related: [McpServer](crates_factory-mcp-server_src_lib.md) · [Tactical Design](TACTICAL-DESIGN.md)*
