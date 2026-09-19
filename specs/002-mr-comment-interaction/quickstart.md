# Quickstart: Merge Request Comment Interaction & Acknowledgment

**Feature Branch**: `002-mr-comment-interaction`  
**Date**: 2026-09-19  
**Specification**: [specs/002-mr-comment-interaction/spec.md](file:///mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/specs/002-mr-comment-interaction/spec.md)

---

## Overview

This guide provides end-to-end verification steps for the Merge Request Comment Interaction feature, validating:
1. Mention tag detection (`@darkgravity` / `@antigravity`) anywhere in MR/PR comments.
2. Immediate application of the eyes reaction (`👀`).
3. Execution of the query/command.
4. Publication of the single threaded response back to the merge request.

---

## Prerequisites

- Rust 1.80+ installed (`rustc --version`).
- GitLab API Token (`GITLAB_API_TOKEN`) or GitHub API Token (`GITHUB_API_TOKEN`) configured in environment or `.env`.

---

## 1. Unit & Mock Testing

Run the isolated unit tests covering tag parsing, reaction APIs, and self-mention suppression:

```bash
# Test directive parsing and tag detection in factory-core
cargo test -p factory-core pr_directive

# Test reaction posting via mock clients in factory-infrastructure
cargo test -p factory-infrastructure mr_comment_reactions
```

**Expected Outcome**:
All tests pass with 0 failures and 0 clippy warnings.

---

## 2. Integration Verification with Mock Git Platform

Run the integrated mock test verifying the full polling, acknowledgment, and reply cycle:

```bash
cargo test -p factory-infrastructure test_poll_mr_notes_applies_eyes_reaction
```

**Expected Outcome**:
1. Poller detects comment containing `@darkgravity explain the architecture`.
2. Mock GitLab server receives `POST /projects/:id/merge_requests/:iid/notes/:note_id/award_emoji` with payload `{"name": "eyes"}`.
3. Event is returned with `PRDirective::Interact { prompt: "explain the architecture" }`.
4. Subsequent polling cycles ignore the already-processed comment ID.

---

## 3. Real Environment Verification

When testing against a real GitLab project (e.g. `gitlab.com/lgcorzo/lince-rs`):

1. Submit a comment on an open MR:
   > "Hey @darkgravity can you check the test results for this MR?"
2. Observe within 10 seconds:
   - An eyes reaction (`👀`) appears on the comment.
3. Observe after processing:
   - A threaded reply from Dark Gravity is posted directly below the comment with the requested analysis.
