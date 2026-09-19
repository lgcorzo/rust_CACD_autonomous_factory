# Implementation Plan: Merge Request Comment Interaction & Acknowledgment

**Branch**: `002-mr-comment-interaction` | **Date**: 2026-09-19 | **Spec**: [specs/002-mr-comment-interaction/spec.md](file:///mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/specs/002-mr-comment-interaction/spec.md)

**Input**: Feature specification from `specs/002-mr-comment-interaction/spec.md`

---

## Summary

Enable developers to communicate with the Dark Gravity autonomous agent directly inside Merge Requests (GitLab) and Pull Requests (GitHub) via tagged comments (e.g., `@darkgravity` or `@antigravity`):
1. **Detection & Acknowledgment**: The outbound Git poller detects tagged comments, ignores self-authored comments, and immediately applies an "eyes" reaction mark (`👀`) using platform APIs (GitLab Award Emojis / GitHub Reactions) to confirm receipt.
2. **Directive Execution**: The comment intent is parsed into domain directives (`PRDirective`), supporting both structured commands (`/spec`, `/refine`, `/validate`) and natural conversational interaction (`Interact { prompt }`).
3. **Clean Thread Delivery**: The agent executes the request and posts a single comprehensive reply directly in the MR discussion thread, including structured diagnostic breakdowns and actionable remediation hints on failure.

---

## Technical Context

**Language/Version**: Rust 1.80+ (2021 Edition)  
**Primary Dependencies**: `reqwest` (with `json`), `serde`, `serde_json`, `tokio`, `async-trait`, `hatchet-sdk`, `tracing`, `mockall` (test-utils)  
**Storage**: Ephemeral stateless execution in worker pods; cursor and processed comment tracking via `CursorStore` (`InMemoryCursorStore` / `PostgresCursorStore`)  
**Testing**: `cargo test --workspace`, mock HTTP unit tests (`wiremock` / `mockito`), integration tests for poller and reaction flows  
**Target Platform**: Linux (gVisor containerized workloads in Kubernetes)  
**Project Type**: Autonomous CA/CD Agent Factory (Multi-crate Cargo workspace)  
**Performance Goals**: Comment acknowledgment reaction applied within 1 polling interval ($\le 10$s); conversation reply posted within 30s for standard queries  
**Constraints**: Zero-trust air-gapped outbound-only communication (no inbound webhooks or exposed public ingress ports); zero self-triggering loops; thread cleanliness (no ephemeral placeholder comments)  
**Scale/Scope**: Operates across configured GitLab projects and GitHub repositories with active MRs/PRs  

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Gate / Principle | Status | Evaluation & Compliance Notes |
| :--- | :--- | :--- |
| **I. Outbound-Only / Zero-Trust** | PASS | Operates via outbound polling against GitLab/GitHub APIs. Zero open inbound ports or webhook ingress. |
| **II. Verification Triad** | PASS | All new endpoints, parser rules, and reaction methods backed by 100% passing tests and clippy-clean code. |
| **III. Credential Security** | PASS | Uses standard API tokens (`GITLAB_API_TOKEN`, `GITHUB_API_TOKEN`) via Authorization headers. |
| **IV. Clean Observability** | PASS | Uses eyes reaction (`👀`) instead of polluting comments for in-progress indication; structured logs for telemetry. |
| **V. Infinite-Loop Immunity** | PASS | Explicitly ignores comments authored by Dark Gravity / Antigravity bot accounts. |

---

## Project Structure

### Documentation (this feature)

```text
specs/002-mr-comment-interaction/
├── spec.md              # Feature specification with clarifications Q1-Q5
├── plan.md              # This implementation plan
├── research.md          # Phase 0 research findings & decisions
├── data-model.md        # Phase 1 data models & domain types
├── quickstart.md        # Phase 1 runnable verification guide
├── contracts/
│   └── mr_comment_interaction.json # JSON Schema for comment interaction events
└── checklists/
    └── requirements.md  # Specification quality checklist
```

### Source Code Impact (repository crates)

```text
crates/
├── factory-core/
│   └── src/
│       └── lib.rs       # Extend PRDirective with Interact variant, tag parsing (@darkgravity/@antigravity)
├── factory-infrastructure/
│   ├── src/
│   │   ├── gitlab.rs    # add_merge_request_note_award_emoji
│   │   ├── github.rs    # add_comment_reaction
│   │   ├── git_poller.rs# Poller triggers eyes reaction & suppresses self-comments
│   │   └── lib.rs
│   └── tests/           # Unit & mock tests for reaction APIs and poller acknowledgment
├── factory-application/
│   └── src/
│       └── workflows/   # Handler to process PRDirective::Interact and post final threaded replies
└── factory-cli/
    └── src/             # CLI options for polling intervals and bot username
```

---

## Proposed Changes by Component

### 1. `factory-core` (`crates/factory-core/src/lib.rs`)
- Expand `PRDirective`:
  - Add `Interact { prompt: String }` and `Validate` variants.
  - Update `PRDirective::parse(text: &str)` to:
    - Match `@darkgravity`, `@dark-gravity`, and `@antigravity` (case-insensitive) anywhere in `text`.
    - If a slash command (`/spec`, `/refine`, `/retry`, `/status`, `/validate`) follows, parse it.
    - Otherwise, treat the stripped prompt as `PRDirective::Interact { prompt }`.

### 2. `factory-infrastructure`
- **`crates/factory-infrastructure/src/gitlab.rs`**:
  - Add `add_merge_request_note_award_emoji(&self, project_id: &str, mr_iid: u64, note_id: u64, emoji_name: &str) -> anyhow::Result<GitlabAwardEmoji>` to `GitlabClient` trait and `HttpGitlabClient`.
  - Endpoint: `POST /api/v4/projects/{id}/merge_requests/{mr_iid}/notes/{note_id}/award_emoji` with body `{"name": emoji_name}`.
- **`crates/factory-infrastructure/src/github.rs`**:
  - Add `add_comment_reaction(&self, repo: &str, comment_id: u64, reaction: &str) -> anyhow::Result<GithubReaction>` to `GithubClient` trait and `HttpGithubClient`.
  - Endpoint: `POST /repos/{owner}/{repo}/issues/comments/{comment_id}/reactions` with body `{"content": reaction}` and header `Accept: application/vnd.github+json`.
- **`crates/factory-infrastructure/src/git_poller.rs`**:
  - Add configurable `bot_username: Option<String>` to `GitPlatformPoller` to skip self-authored comments.
  - In `poll_github_pr_comments`:
    - Check if author is bot; if so, skip.
    - If `PRDirective` matches, call `github_client.add_comment_reaction(repo, comment.id, "eyes")` immediately.
  - In `poll_gitlab_mr_notes`:
    - Check if author is bot; if so, skip.
    - If `PRDirective` matches, call `gitlab_client.add_merge_request_note_award_emoji(project_id, mr.iid, note.id, "eyes")` immediately.

### 3. `factory-application`
- Implement conversational dispatch for `PRDirective::Interact` that processes the prompt using factory reasoning agents and posts the final threaded reply via `post_merge_request_note` or `post_pull_request_comment`.
- On error, format failure diagnostics and remediation steps in markdown.

---

## Verification Plan

### Automated Tests
1. **`factory-core` unit tests**:
   ```bash
   cargo test -p factory-core test_parse_pr_directive_mentions
   ```
   - Verify parsing `@darkgravity`, `@DarkGravity`, `@antigravity`, `/validate`, and conversational prompts.
2. **`factory-infrastructure` unit & mock tests**:
   ```bash
   cargo test -p factory-infrastructure test_gitlab_award_emoji
   cargo test -p factory-infrastructure test_github_comment_reaction
   cargo test -p factory-infrastructure test_poller_adds_eyes_reaction_and_ignores_bot
   ```
3. **Workspace check**:
   ```bash
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   ```

### Manual / Staging Verification
- Trigger comment on a test MR with `@darkgravity hello`:
  - Verify eyes reaction (`👀`) appears within 10s.
  - Verify threaded reply is posted with response.
