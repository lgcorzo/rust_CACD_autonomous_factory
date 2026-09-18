# Quickstart Validation Guide: Work Item Comments & Remote GitOps Delivery

**Feature**: `001-workitem-status-and`  
**Date**: 2026-09-18  

---

## 1. Prerequisites

- Rust toolchain: 1.80+ / 1.85+ (`cargo`, `clippy`, `rustfmt`)
- Local or test environment credentials (or mock environment for offline testing):
  - `GITLAB_API_TOKEN` (or mock HTTP server via `wiremock` / `mockall`)
  - `GITHUB_API_TOKEN`
  - `GITLAB_URL` (default: `https://gitlab.com`)

---

## 2. Unit & Integration Tests

### 2.1 Testing GitLab Client Extensions (`factory-infrastructure`)
Verify `post_issue_note`, `create_branch`, `create_commit_files`, and `create_merge_request`:

```bash
cargo test -p factory-infrastructure --lib gitlab::tests
```

Expected output:
- `test gitlab::tests::test_post_issue_note ... ok`
- `test gitlab::tests::test_create_branch ... ok`
- `test gitlab::tests::test_create_commit_files ... ok`
- `test gitlab::tests::test_create_merge_request ... ok`

### 2.2 Testing GitHub Client Extensions (`factory-infrastructure`)
Verify `post_issue_comment` and `create_branch`:

```bash
cargo test -p factory-infrastructure --lib github::tests
```

Expected output:
- `test github::tests::test_post_issue_comment ... ok`
- `test github::tests::test_create_branch ... ok`

### 2.3 Testing End-to-End Workflow Delivery (`factory-application`)
Verify `autonomous_mission` Phase 5 delivery with real Git client invocation:

```bash
cargo test -p factory-application --test functional_e2e_test
```

Expected output:
- `test test_autonomous_mission_full_delivery ... ok`
- All status comments emitted and remote MR generated.

---

## 3. End-to-End Cluster Smoke Test

To verify live against an actual GitLab issue:
1. In GitLab (`https://gitlab.com/lgcorzo/lince-rs`), ensure an issue exists (e.g. Issue #1) with label `autonomous-mission`.
2. Ensure `factory-poller` is running with `GITLAB_API_TOKEN` configured.
3. Observe `factory-poller` log:
   ```text
   Poll cycle: 1 issues ingested
   ```
4. Check GitLab Issue #1:
   - Comment 1: "🚀 **Dark Gravity Mission Ingested**"
   - Comment 2: "📋 **Planning Completed** (RustantAgent)"
   - Comment 3: "🛡️ **Validation & Security Review Passed** (ZeroClawAgent & Aethelgard)"
   - Comment 4: "🎉 **Mission Delivered** - Merge Request opened: `https://gitlab.com/lgcorzo/lince-rs/-/merge_requests/...`"
5. Verify on GitLab repository `lgcorzo/lince-rs`:
   - Remote branch `mission-lgcorzo-lince-rs-1` (or `specs/001-...`) exists.
   - Merge Request is open and targets `main`.
