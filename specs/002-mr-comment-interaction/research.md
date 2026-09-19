# Research: Merge Request Comment Interaction & Acknowledgment

**Feature Branch**: `002-mr-comment-interaction`  
**Date**: 2026-09-19  
**Specification**: [specs/002-mr-comment-interaction/spec.md](file:///mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/specs/002-mr-comment-interaction/spec.md)

---

## 1. Comment Ingestion: Outbound Polling vs. Inbound Webhooks

### Decision
Use outbound periodic polling against active GitLab Merge Requests and GitHub Pull Requests within `GitPlatformPoller`, integrated with `CursorStore` for event idempotency.

### Rationale
- Dark Gravity operates inside secure, isolated execution environments (e.g. private Kubernetes clusters, gVisor microVMs) without exposing public ingress ports or requiring reverse proxy tunnels (ngrok/Cloudflare tunnels).
- An outbound polling loop complies with Constitution Principle I (Outbound-Only / Zero-Trust).
- Polling frequency of $\le 10$ seconds satisfies the user experience SLA for comment acknowledgment while respecting Git provider rate limits through `since` timestamps and conditional requests.

### Alternatives Considered
- **Inbound Webhook HTTP Listener**: Rejected per clarification answer (Option B). While webhooks offer immediate millisecond pushes, they require public endpoints, TLS certificate management, and expose inbound attack surfaces.
- **WebSocket / Event Streams**: Neither GitLab nor GitHub provide persistent raw comment WebSockets; they rely on webhooks or polling.

---

## 2. Visual Acknowledgment: Platform Reaction APIs

### Decision
Implement provider-specific reaction endpoints:
1. **GitLab Award Emoji API**:
   - Endpoint: `POST /api/v4/projects/:id/merge_requests/:mr_iid/notes/:note_id/award_emoji`
   - Payload: `{"name": "eyes"}`
   - Header: `PRIVATE-TOKEN: <GITLAB_API_TOKEN>`
2. **GitHub Reactions API**:
   - Endpoint: `POST /repos/:owner/:repo/issues/comments/:comment_id/reactions`
   - Payload: `{"content": "eyes"}`
   - Headers: `Authorization: Bearer <GITHUB_API_TOKEN>`, `Accept: application/vnd.github+json`

### Rationale
- The user explicitly requested an icon of two eyes (`👀`) to mark comments as read.
- Both GitLab and GitHub natively support `eyes` as a recognized reaction identifier.
- Adding an emoji reaction is non-destructive, does not clutter the comment thread with placeholder comments, and gives immediate visual feedback.

### Alternatives Considered
- **Temporary Placeholder Comment ("Seen / Working on it...")**: Rejected per clarification answer (Option B). Users explicitly prefer reaction-only indicators to prevent thread pollution.
- **Editing the User's Comment**: Impossible and violates security/attribution models.

---

## 3. Mention Detection & Directive Parsing

### Decision
Enhance `PRDirective::parse` in `factory-core` to:
1. Support case-insensitive detection of `@darkgravity` and `@antigravity` anywhere in the comment body.
2. Filter out comments authored by the agent itself (`bot_username` matching).
3. Recognize structured commands (`/spec`, `/refine`, `/retry`, `/status`, `/validate`, `/run-tests`).
4. Support unstructured free-form conversation and queries (`PRDirective::Interact { prompt }`).

### Rationale
- Reviewers interact naturally with AI teammates by mentioning `@darkgravity` or `@antigravity` inline (e.g., "Hey @darkgravity can you explain this logic?" or "@darkgravity /validate").
- Preventing self-triggers is critical to avoiding infinite automated feedback loops (SC-002).

### Alternatives Considered
- **Strict Slash Commands Only**: Too rigid; users often ask natural language questions in MR comments.
- **Regex-Only Match Without Parsing**: Lacks typed domain model safety in `factory-core`.

---

## 4. Response Delivery & Error Reporting

### Decision
1. **Success / Completed Discussion**: Post a threaded reply to the originating MR note (`post_merge_request_note` / `post_pull_request_comment`) containing the conversational answer or mission outcome.
2. **Failure / Diagnostics**: Post a structured failure report in markdown with failure category, diagnostic logs snippet, and actionable remediation steps.

### Rationale
- Keeps conversation centralized where the code review is happening.
- Immediate actionable error guidance prevents developer confusion and eliminates silent failures (SC-007).

### Alternatives Considered
- **Separate Bot Issue / PR**: Clutters the project with duplicate tracking items when the user specifically opened the discussion in the MR thread.
