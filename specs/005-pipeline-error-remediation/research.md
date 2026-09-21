# Research: Pipeline Error Remediation

**Feature**: `005-pipeline-error-remediation`
**Date**: 2026-09-21

## R1: GitHub Actions Workflow Runs API

**Decision**: Use GitHub REST API v3 endpoints for workflow runs and job logs.

**Rationale**: The existing `HttpGithubClient` already uses `reqwest` + bearer token auth against `api.github.com`. The workflow runs API follows the same auth and pagination patterns.

**Key Endpoints**:
- `GET /repos/{owner}/{repo}/actions/runs?status=failure` — List failed workflow runs
- `GET /repos/{owner}/{repo}/actions/runs/{run_id}/jobs` — List jobs for a run
- `GET /repos/{owner}/{repo}/actions/jobs/{job_id}/logs` — Download job log (returns plaintext)

**Rate Limits**: 5,000 requests/hour with bearer token. Polling 4 repos every 5 minutes = 48 requests/hour (well within limits). Add `If-Modified-Since` header for efficiency.

**Alternatives Considered**:
- GitHub Webhooks: Would require public endpoint exposure; rejected because the factory is a polling-based system by design (no inbound webhook receiver)
- GitHub GraphQL API: More efficient but doesn't expose Actions workflow run logs; rejected

---

## R2: GitLab CI Pipelines API

**Decision**: Use GitLab REST API v4 endpoints for pipelines and job traces.

**Rationale**: The existing `HttpGitlabClient` already uses `reqwest` + `PRIVATE-TOKEN` against the GitLab API. Pipeline endpoints follow the same patterns.

**Key Endpoints**:
- `GET /projects/{id}/pipelines?status=failed&updated_after={timestamp}` — List failed pipelines
- `GET /projects/{id}/pipelines/{pipeline_id}/jobs` — List jobs for a pipeline
- `GET /projects/{id}/jobs/{job_id}/trace` — Download job trace log (plaintext)

**Rate Limits**: Configurable per-instance; GitLab.com defaults to 2,000 requests/minute for authenticated users.

**Alternatives Considered**:
- GitLab Webhooks: Same rejection rationale as GitHub
- GitLab CI/CD Events API (newer): Not yet stable across self-hosted instances; rejected for compatibility

---

## R3: Error Log Classification Strategy

**Decision**: Rule-based regex pattern matching with ordered priority rules.

**Rationale**: The initial error categories are well-defined with distinctive log signatures. Machine-learning classifiers would add complexity without proportional value for the initial set of ~7 categories. The classifier can be extended by adding new regex patterns without changing the architecture.

**Classification Rules (ordered by specificity)**:

| Category | Pattern | Example Match |
|----------|---------|---------------|
| `lint_violation` | `error\[.*\].*\(clippy\)` or `warning:.*\[clippy::` | `error[E0000]: (clippy) unused_variable` |
| `code_compilation` | `error\[E\d{4}\]:` | `error[E0308]: mismatched types` |
| `test_failure` | `test .* \.\.\. FAILED` or `failures:` | `test core::test_parse ... FAILED` |
| `security_audit` | `Crate:.*\nVersion:.*\nWarning:` or `cargo audit` + `vulnerability` | `Crate: openssl\nVersion: 0.10.1\nWarning: RUSTSEC-2026-0001` |
| `infrastructure_build` | `docker build` + `(COPY failed\|apt-get.*E:)` or `protoc.*not found` | `E: Unable to locate package libprotobuf-dev` |
| `infrastructure_transient` | `Connection timed out\|rate limit\|503 Service Unavailable\|OOM` | `fatal: unable to access 'https://github.com/...': Connection timed out` |
| `unknown` | (fallback — no pattern matched) | Any unrecognized error |

**Metadata Extraction**: After classification, a second pass extracts structured data:
- `code_compilation`: file path + line number from `→ src/foo.rs:42:5`
- `lint_violation`: clippy rule name from `[clippy::unused_variable]`
- `test_failure`: test name from `test module::test_name`
- `security_audit`: crate name + RUSTSEC ID

**Alternatives Considered**:
- LLM-based classification: Too expensive for high-frequency, deterministic error patterns; would add latency and cost per classification
- Tree-sitter AST parsing of logs: Overkill for plain-text log output; rejected

---

## R4: Remediation Mission Integration

**Decision**: Reuse the existing `PollerDaemonService::ingest_issue()` pattern to publish remediation missions to the `mission-input` Kafka topic.

**Rationale**: The factory already processes missions from the `mission-input` topic through the Hatchet DAG. Pipeline failures can be formatted as synthetic mission payloads with the same schema, allowing the existing Rustant planner and ZeroClaw executor to process them without changes.

**Mission Payload Format** (matching existing `ingest_issue` schema):
```json
{
  "mission_id": "remediation-{repo}-{run_id}",
  "goal": "Fix {category} error in {repo}: {error_summary}",
  "repository_path": "{repo}",
  "source_platform": "pipeline_remediation",
  "pipeline_failure": { ... },
  "error_classification": { ... },
  "verifiable_credential": { ... }
}
```

**Alternatives Considered**:
- Direct Hatchet API call: Would bypass NHI signing and Kafka audit trail; rejected for security/traceability
- Separate Hatchet workflow: Would require new DAG definition; rejected because existing mission DAG already handles code fixes

---

## R5: Self-Referential Safety Guard

**Decision**: Hardcoded check in the remediation mission trigger: if `repository == "lgcorzo/rust_CACD_autonomous_factory"`, the system creates a GitHub issue tagged `pipeline-remediation-pending` instead of auto-triggering a mission.

**Rationale**: Autonomous self-modification could create recursive failure loops. The safety guard is a simple, auditable conditional check rather than a configurable rule, because the risk is catastrophic and the check is cheap.

**Alternatives Considered**:
- Configurable allowlist of "safe" repos: Adds unnecessary complexity; the self-referential case is the only one needing a guard
- Two-phase approval via Kafka: Over-engineered for a simple boolean check

---

## R6: Recurring Failure Detection

**Decision**: Track `(repository, error_category, error_fingerprint)` tuples in the `CursorStore`. Increment a counter on each occurrence. Escalate to human review after 3 consecutive identical failures.

**Rationale**: The `CursorStore` already provides durable, idempotent event tracking. Extending it with a counter per error fingerprint is a minimal change. The "fingerprint" is a hash of `(category, file_path, rule_name_or_test_name)` — enough to detect the same error recurring without being sensitive to unrelated log noise.

**Alternatives Considered**:
- Sliding time window: More complex and harder to reason about; 3-strike counter is simpler and more predictable
- External deduplication service: Unnecessary architectural complexity for a simple counter

---

## R7: Concurrency Limiting

**Decision**: Use a `tokio::sync::Semaphore` in `PipelineRemediationService` to limit concurrent remediation missions. Default permit count: 5 (configurable via `MAX_CONCURRENT_REMEDIATIONS` env var).

**Rationale**: Prevents overwhelming the Hatchet DAG scheduler and downstream resources. The semaphore is local to the remediation service, which runs as a single instance per factory worker.

**Alternatives Considered**:
- Kafka consumer group partitioning: The factory doesn't consume from Kafka for missions; it produces to it
- Hatchet-level concurrency controls: Exists but operates at a different granularity (per-step, not per-mission-type)
