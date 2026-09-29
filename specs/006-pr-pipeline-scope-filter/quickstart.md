# Quickstart: PR Pipeline Scope Filtering for Autonomous Remediation

**Feature**: `006-pr-pipeline-scope-filter`
**Date**: 2026-09-29

## Overview
This feature restricts the Dark Gravity autonomous pipeline error remediation engine to only detect, remediate, and escalate CI/CD pipeline failures that originate from **active, tracked Pull Requests (GitHub) and Merge Requests (GitLab)**.

## How It Works

1. **Active PR Discovery**: The poller inspects open PRs/MRs in the monitored repositories before evaluating pipelines.
2. **Early Exit**: If a repository has no active open PRs, pipeline polling is bypassed immediately, saving API quota and time.
3. **Correlation**: When failed workflow runs or CI pipelines are detected, the system checks whether the run's branch, commit SHA, or PR list matches an active tracked PR.
4. **Idempotent Cursor Marking**: Out-of-scope runs (e.g. failures on `main` or untracked branches) are recorded in the PostgreSQL/InMemory `CursorStore` as processed, preventing re-evaluation on subsequent poll cycles.
5. **No Spurious Issues**: Only in-scope failures can trigger remediation missions or human escalation issues.

## Testing & Verification

### Run Automated Unit & Contract Tests

```bash
# Test the pipeline scope filter domain unit tests
cargo test -p factory-core --lib

# Test poller pipeline scoping in factory-infrastructure
cargo test -p factory-infrastructure --lib git_poller::tests::test_poll_github_pipelines_scoped_to_active_pr
cargo test -p factory-infrastructure --lib git_poller::tests::test_poll_github_pipelines_ignores_non_pr_runs

# Test remediation service safety gate
cargo test -p factory-application --lib workflows::pipeline_remediation::tests
```

### End-to-End Local Verification

```bash
# Run a single polling cycle across configured repositories in dry-run mode
cargo run -p factory-cli -- poller --github-repos "lgcorzo/rust_CACD_autonomous_factory" --polling-interval-secs 30
```

Verify the logs output:
```text
INFO factory_infrastructure::git_poller: Discovered 2 active PRs for lgcorzo/rust_CACD_autonomous_factory
DEBUG factory_infrastructure::git_poller: Run 35138683447 on 'main' does not match any active PR; skipping.
INFO factory_cli: Poll cycle: 0 issues ingested, 0 directives processed, 0 pipelines remediated, 0 pipelines escalated
```
