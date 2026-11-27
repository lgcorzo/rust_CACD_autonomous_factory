# Architecture: `rust_CACD_autonomous_factory` in Dark Gravity

## Ecosystem Role & Tier

- **Tier**: `core_factory`
- **Role**: Core execution brain of the Dark Gravity Autonomous Software Factory — orchestrates issues to verified PRs using Rust (Edition 2024), Hatchet durable workflows, Apache Kafka event streams, Kubernetes `AgentWorkspace` CRDs (`ADR-008`), `gVisor` (`runsc`) sandboxes (`ADR-001`), Ed25519 W3C Verifiable Credentials + Vault JIT memory zeroization, and privileged runtime MCP tooling.
- **Visibility**: `public`

## Internal Component Design

Structured as a strict **5-crate Rust 2024 Onion Architecture** (`Cargo.toml`):
1. `crates/factory-core/`: Pure domain core (zero I/O) defining `Mission`, `Task`, `SddMissionPlan`, `PRDirective`, `SecurityValidator`, `SandboxConstraint`, and `JitToken` (`ZeroizeOnDrop` benchmarked at `54.2 ns` within `< 4.33 µs` budget per `ADR-001`).
2. `crates/factory-application/`: `Agent` trait implementations (`RustantAgent`, `ZeroClawAgent`, `AuditorAgent`, `FinOpsAgent`, `QAObserverAgent`, `DocumentationAgent`), Hatchet 5-step mission DAG (`rustant-plan`, `zeroclaw-execute`, `zeroclaw-validate`, `rustant-review`, `factory-deliver`) with `SwarmConfig` `[P]` fan-out and `CircuitBreakerGuard` (`A-B-A-B` diff halt, 3 retries).
3. `crates/factory-infrastructure/`: External adapters for Confluent Kafka (`mission-ingestion`, `agent-thought`, `budget-exceeded`), CloudNativePG / MinIO checkpoints, HashiCorp Vault JIT, OpenZiti mTLS 1.3 (`ZitiA2aClient`), Semantica, Dark-LiteLLM (`X-Vtag-*`), and Sentry.
4. `crates/factory-mcp-server/`: Privileged runtime Axum HTTP/SSE gateway (port `8100`) exposing runtime skills (`launch_sandbox_pod`, `run_tests`, `index_code`, `retrieve_context`, `/webhooks/github`) protected by bearer auth (`MCP_API_BEARER_TOKEN`). Distinct from public read-only `dark-gravity-mcp`.
5. `crates/factory-cli/`: CLI daemon (`GitPlatformPoller`, 30–60s outbound GitHub/GitLab poll) and `factory-operator` managing `AgentWorkspace` (`darkgravity.io/v1alpha1`, `ADR-008`) and evidence contract validation.

## Upstream & Downstream Ecosystem Interfaces

```text
GitHub/GitLab Issues & PR Directives ──► factory-cli (GitPlatformPoller) ──► Kafka (mission-ingestion)
                                                                                 │
spec-kit + speckit-superpowers-bridge ──► Hatchet 5-Step DAG (factory-worker) ◄──┘
                                              │
          ┌───────────────────────────────────┼───────────────────────────────────┐
          ▼                                   ▼                                   ▼
gVisor Sandboxes (development ns)   R2R / r2r-reader / Semantica (llm-apps)  Dark-LiteLLM (X-Vtag FinOps)
          │                                                                       │
          └──────────────────► Feature Branch + Pull Request ◄────────────────────┘
                                              │
                         Vertex 4 HITL Manual Merge Gate -> Stage 5 index_code
```

## Security Boundary & Governance

- **`ADR-001` & `ADR-008`**: Untrusted code runs inside `gVisor` (`runsc`) pods clamped at `<= 30 MiB` RAM / `250m` CPU (micro-sandboxes) or `512 MiB` / `500m` (compile sandboxes). Note: live gVisor exit signaling (`FR-008` / `CTRL-02`) remains classified **`TESTED`** while host CRI shim finding `AUD-010` is open.
- **Four HITL Vertices**: Vertex 1 (issue trigger), Vertex 2 (Spec-Kit plan approval), Vertex 3 (`Agent-Stuck` override), Vertex 4 (mandatory human PR merge into `main`).
- **Epistemic Integrity**: Controls implement readiness for auditability; no third-party certification (EU AI Act, SOC 2 Type II, or ISO/IEC 25059) is claimed.

## Canonical Cross-Links

- [DARK_GRAVITY_MANIFEST.md](./DARK_GRAVITY_MANIFEST.md)
- [REPOSITORIES.md](./REPOSITORIES.md)
- [ADR-008 Specification](./wiki/architecture/decisions/adr-008-google-ax-crd-workspace-standard-v3.1.md)
- [Global Architecture Portal](https://lgcorzo.github.io/architecture.html)
- [Unified Start-Here Portal](https://lgcorzo.github.io/start-here)
- [AI Auditor Pack](https://lgcorzo.github.io/docs/auditor/)
