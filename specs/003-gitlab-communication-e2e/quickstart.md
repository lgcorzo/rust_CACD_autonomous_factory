# Quickstart Validation Guide: GitLab Communication End-to-End Verification

**Branch**: `003-gitlab-communication-e2e` | **Date**: 2026-09-19

## Overview

This guide details how to execute and validate the end-to-end GitLab communication test suite across both hermetic mock environments (for CI/CD) and live GitLab instances (for pre-flight cluster operational verification).

---

## Prerequisites

1. **Rust Toolchain**: `rustc` and `cargo` 1.80+ installed.
2. **Repository Workspace**: `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory`.
3. **For Live Verification (Optional)**:
   - `GITLAB_API_TOKEN`: Valid personal or project access token with `api` or `read_api` scopes.
   - `GITLAB_PROJECTS`: Comma-separated project paths (e.g. `lgcorzo/fastapi-autogen-team`).
   - `GITLAB_URL`: Default is `https://gitlab.com`.

---

## Scenario 1: Execute Hermetic E2E Test Suite (Local / CI)

Runs the fully automated wire-level E2E integration test simulating GitLab REST API v4 using `wiremock`. Validates issue polling, MR note detection, eyes emoji reaction (`POST .../award_emoji`), directive execution, and discussion replies.

```bash
cargo test --package factory-application --test gitlab_e2e_integration_test -- --nocapture
```

### Expected Output

```text
running 1 test
test test_gitlab_communication_full_e2e_roundtrip ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s
```

All communication flows are verified:
- [x] Issue detection & resource limit extraction
- [x] NHI credential issuance
- [x] MR directive detection (`/status`, `/interact`)
- [x] Instant `👀` reaction posted to GitLab API
- [x] Thread reply posted back to MR discussion
- [x] Self-authored bot note filtering
- [x] Idempotency cursor persistence

---

## Scenario 2: Run Live Platform Verification via CLI

When running inside Kubernetes (`factory-poller` pod) or from a developer machine configured with GitLab credentials, execute the diagnostic verification command:

```bash
cargo run --bin factory-cli -- gitlab-verify \
  --gitlab-projects "lgcorzo/fastapi-autogen-team"
```

Or run via environment variables:

```bash
export GITLAB_API_TOKEN="glpat-xxxxxxxxxxxxxxxx"
export GITLAB_PROJECTS="lgcorzo/fastapi-autogen-team"
export GITLAB_URL="https://gitlab.com"

factory-cli gitlab-verify
```

### Expected Scorecard Output

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│              Dark Gravity GitLab Communication Health Scorecard              │
├───────────────────────────────┬───────────────────────────┬─────────────────┤
│ Check                         │ Target                    │ Status          │
├───────────────────────────────┼───────────────────────────┼─────────────────┤
│ GitLab Authentication         │ https://gitlab.com        │ PASSED (85ms)   │
│ Project Metadata Access       │ lgcorzo/fastapi-autogen-..│ PASSED (120ms)  │
│ Open Issues Polling           │ lgcorzo/fastapi-autogen-..│ PASSED (110ms)  │
│ Active Merge Requests Polling │ lgcorzo/fastapi-autogen-..│ PASSED (95ms)   │
│ Note & Award Capability Test  │ lgcorzo/fastapi-autogen-..│ PASSED (140ms)  │
└───────────────────────────────┴───────────────────────────┴─────────────────┘
Overall Status: HEALTHY (All 5 communication interfaces verified)
```

---

## Scenario 3: Verify Poller Daemon Reaction in Live Cluster

To inspect the live poller running in the Kubernetes cluster:

```bash
# Check poller container logs in the agents namespace
kubectl logs -n agents deployment/factory-poller -c poller --tail=100 -f
```

Look for confirmation logs:
- `Poller active: interval=30s, gitlab_projects=["lgcorzo/fastapi-autogen-team", "lgcorzo/lince-rs"]`
- `Detected directive: Status from note <id> on MR !<iid>`
- `Added eyes reaction to GitLab MR note <id>`
- `Posted response back to GitLab MR !<iid>`
