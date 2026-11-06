# Supporting Repository Review: `fastapi-autogen-team` (`FR-023`, `FR-026`, `FR-030`, `SC-012`)

**Repository**: `https://github.com/lgcorzo/fastapi-autogen-team.git` (`/mnt/F024B17C24B145FE/Repos/fastapi-autogen-team`)  
**Inspected Branch**: `main`  
**Inspected Full Commit SHA**: `6b9c4513eea630a04255a68412f2042cd61dbea1`  
**Reviewed At (UTC)**: `2026-11-06T18:09:17Z`  

---

## 1. Architecture, Rust Edition, and API Surface (`T036`)

- **Crate & Edition (`Cargo.toml`)**:
  - Package name: `rust-agent-team` (`version = "0.1.0"`, library target `rust_agent_team`).
  - Rust Edition: `2021`.
  - Core stack: `axum = "0.7"`, `tokio = "1"`, `tower-http = "0.7"`, `rig-core = "0.34.0"`, `opentelemetry* = "0.33"`, `reqwest = "0.12"` (`rustls-tls`).
- **HTTP API Endpoints (`src/interface/http/routes.rs`, `handlers.rs`)**:
  - `GET /agent`: Returns `303 See Other` redirect to documentation.
  - `GET /agent/api/v1beta/models`: Returns OpenAI-compatible model list (`minimax-m2.7:cloud`).
  - `POST /agent/api/v1beta/chat/completions`: Accepts OpenAI-compatible chat payloads (`ValidatedJson<Input>`), rejects empty `messages` (`422 Unprocessable Entity`) and oversized bodies (`413 Payload Too Large`), and supports both synchronous JSON completion and SSE streaming (`stream: true`).
- **Streaming & Multi-Agent Pipeline (`src/domain/agent/team.rs`)**:
  - Executes parallel language detection + English translation (`ollama/qwen2.5:7b` via LiteLLM), a `Planner` agent generating up to 5 tool-prefixed queries (`[JIRA]`, `[CONFLUENCE]`, `[R2R]`), specialized tool searchers (`JiraTool`, `ConfluenceTool`, `R2RTool`), and a `QA` synthesis agent that translates the final answer back into the user's detected language.
  - On the SSE path (`run_stream`), yields `AgentEvent::Progress` (`reasoning_content` chunks), `AgentEvent::Delta` (stripping `TERMINATE` tokens), and `AgentEvent::Done` (`[DONE]`).
- **Telemetry (`src/infrastructure/telemetry.rs`, `src/main.rs`)**:
  - Initializes OpenTelemetry OTLP HTTP exporter (`DEFAULT_OTEL_ENDPOINT`, default `http://otel-collector:4318/v1`) and `tracing_subscriber`.

---

## 2. Test Execution & Reproducibility Findings

- **Single-Threaded Execution (`--test-threads=1`)**:
  - Command: `cargo test --test unit_tests --test integration_tests -- --test-threads=1`
  - Result: **`12/12 unit tests passed`, `19/19 integration/security tests passed`, `5 smoke tests ignored`** (exit code `0`).
- **Parallel Execution Finding (`AUD-011` — Shared Process Environment Race in Tests)**:
  - Command: `cargo test --test unit_tests --test integration_tests` (default multi-threaded runner)
  - Observation: `security::test_cors_specific_origins` intermittently fails with `left: 405, right: 200` because `security::sanitization_test::test_cors_empty_origins_no_panic` and `test_cors_malformed_origins_no_panic` concurrently call `std::env::set_var("ALLOWED_ORIGINS", ...)` and `std::env::remove_var("ALLOWED_ORIGINS")` in the same test binary (`integration_tests`). When `ALLOWED_ORIGINS` is removed between `set_var` and `create_app(state)` in `test_cors_specific_origins`, `cors_layer()` returns `None` and the `OPTIONS` preflight request hits Axum's route without a `CorsLayer`, returning `405 Method Not Allowed`.
  - Impact: Test-only race condition (P2 quality finding); production `main()` reads `ALLOWED_ORIGINS` once at startup.

---

## 3. Repository Naming & Compatibility Decision (`FR-026`, `SC-012`)

- **Current State**: The GitHub repository URL is `lgcorzo/fastapi-autogen-team` (reflecting its original Python/FastAPI + AutoGen implementation), while the codebase is now 100% Rust (`rust-agent-team` crate in `Cargo.toml`, `Rust-Agent-team Service` in `README.md`).
- **Compatibility Impact of Renaming**:
  - `rust_CACD_autonomous_factory/crates/factory-cli/src/main.rs` (`Poller` default `GITHUB_REPOS` and `GitlabVerify` default `GITLAB_PROJECTS`) hardcodes `lgcorzo/fastapi-autogen-team`.
  - `gitops_internal_lgcorzo` image automation and deployment manifests reference the existing repository coordinates.
- **Decision (`FR-026`)**: **Retain the repository slug `lgcorzo/fastapi-autogen-team` until a coordinated cross-repo migration PR updates `factory-cli` defaults, GitOps Flux sources, and container registry paths simultaneously.** Document in public materials that `lgcorzo/fastapi-autogen-team` hosts the `rust-agent-team` (Axum + Rig) service.
