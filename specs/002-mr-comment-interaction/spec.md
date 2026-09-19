# Feature Specification: Merge Request Comment Interaction with Dark Gravity

**Feature Branch**: `002-mr-comment-interaction`

**Created**: 2026-09-19

**Status**: Draft

**Input**: User description: "I want to interact with the darkgravit using the commnets in the merge request using a tag. when a merge request is create i wnato comimicte with darkgravity using the comments, add a mark in the comment to hsow that has bean readed like a ico of two eyes. in /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory"

## Clarifications

### Session 2026-09-19

- Q: Which Git hosting platforms are in-scope for comment-tagging interaction? → A: Both GitLab (Merge Requests) and GitHub (Pull Requests) using a provider-agnostic interface.
- Q: What mechanism should Dark Gravity use to detect new merge request comments? → A: Polling loop (periodically checking active MRs/PRs without requiring inbound webhook ingress).
- Q: How should Dark Gravity identify that a comment is directed at it? → A: Standard `@darkgravity` (or `@antigravity`) mention anywhere in the comment body (case-insensitive).
- Q: How should Dark Gravity notify users of its progress for long-running requests? → A: Reaction-only acknowledgment (`👀`) signals in-progress processing, followed by a single comprehensive reply upon completion to avoid thread clutter.
- Q: If an action fails during execution, how should Dark Gravity communicate the failure? → A: Post a structured error response in the thread detailing the failure reason and suggested remediation.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Instant Acknowledgment of Tagged MR Comments (Priority: P1)

As a software engineer reviewing or authoring a Merge Request (MR) on GitLab or Pull Request (PR) on GitHub, when I mention Dark Gravity anywhere in a comment or review note using `@darkgravity` or `@antigravity`, I want a prompt visual acknowledgment—an "eyes" reaction (`👀`) added to my comment—so that I know Dark Gravity's polling process has detected, read, and queued my request, while keeping the thread clean from temporary status messages.

**Why this priority**: Immediate feedback establishes trust and confirms that the automated system has captured the user's intent without leaving the engineer wondering if their comment was heard.

**Independent Test**: Can be fully tested by submitting a comment containing the mention tag `@darkgravity` or `@antigravity` on a merge request or pull request and verifying that the comment receives an eyes reaction (`👀`) within one polling cycle without generating interim placeholder comments.

**Acceptance Scenarios**:

1. **Given** an open merge request or pull request, **When** a user posts a comment mentioning `@darkgravity` or `@antigravity` anywhere in the text, **Then** the polling system detects the comment on its next cycle and immediately adds the eyes reaction (`👀`) to indicate it has been read.
2. **Given** an open merge request or pull request, **When** a user posts a comment *without* the mention tag, **Then** the system ignores the comment and does not apply an eyes reaction.
3. **Given** a comment where the mention tag was already processed, **When** subsequent polling cycles scan that comment, **Then** the system skips it and does not duplicate reactions or reprocessing.

---

### User Story 2 - Automated Conversation & Task Response in MR Threads (Priority: P2)

As a developer working on a merge request or pull request, when I ask Dark Gravity a question, request code improvements, or ask for clarifications via a tagged comment, I want Dark Gravity to formulate an intelligent response and post it as a single comprehensive reply in the same comment thread once complete, so that discussion and code refinements remain organized, contextual, and uncluttered.

**Why this priority**: Delivering actionable agent feedback directly inside the code review conversation accelerates review cycles and keeps collaboration centralized within the review interface.

**Independent Test**: Can be tested by asking Dark Gravity a query (e.g., `@darkgravity summarize changes in this MR`) and verifying that a structured reply is posted as a single threaded response underneath the user's comment upon mission completion.

**Acceptance Scenarios**:

1. **Given** a tagged user comment asking for analysis or code remediation, **When** Dark Gravity completes processing the request, **Then** a single reply is posted directly in the conversation thread addressing the user's prompt.
2. **Given** an ongoing discussion thread in an MR or PR, **When** a user replies in the thread with a follow-up tagged `@darkgravity`, **Then** Dark Gravity maintains conversation context from preceding thread notes to answer accurately.

---

### User Story 3 - Action Triggering via MR Commands (Priority: P3)

As an engineer collaborating on a merge request, I want to invoke specific automated factory actions (such as triggering targeted test suites, requesting refactor suggestions, or generating architecture documentation updates) through structured commands in tagged comments.

**Why this priority**: Enables conversational CA/CD orchestration without requiring developers to leave the merge request view or use separate CLI/dashboard tools.

**Independent Test**: Can be tested by posting an action-oriented tagged comment (e.g., `@darkgravity run-tests`) and verifying that the corresponding factory mission is scheduled, tracked via the eyes reaction, and its outcome reported back as a single final summary note.

**Acceptance Scenarios**:

1. **Given** a valid action command provided alongside the mention tag (e.g., `@darkgravity validate`), **When** the comment is ingested via polling, **Then** the eyes reaction is applied, the mission is executed, and a final summary of results is posted back to the MR thread.
2. **Given** an unrecognized command or malformed prompt following the mention tag, **When** the comment is ingested, **Then** Dark Gravity replies with helpful usage guidance and available commands.
3. **Given** a requested action that fails during execution (e.g., compilation or test errors), **When** processing concludes, **Then** Dark Gravity posts a structured failure breakdown explaining the root cause and suggesting concrete remediation steps.

---

### Edge Cases

- **Self-Trigger Prevention**: How does the system handle comments posted by Dark Gravity itself? (Dark Gravity MUST ignore comments generated by its own service account to prevent infinite feedback loops).
- **Rapid Successive Comments**: What happens when multiple tagged comments are submitted between polling intervals? (All newly detected comments must be individually acknowledged with eyes reactions and queued in chronological order).
- **Polling Frequency & API Rate Limits**: How does the system handle platform rate limits during high-frequency polling? (The poller must utilize conditional requests/ETags, track the last seen comment timestamp, and back off gracefully when approaching rate limits).
- **Execution Failures**: What happens when an executed mission fails? (A structured failure summary is posted in the thread with diagnostic context and remediation steps, preventing silent failures).
- **Edited Comments**: What happens if a user edits an already-read comment to add a tag or modify instructions? (The system records processed comment IDs and hashes/timestamps to avoid redundant reactions while supporting edits if newly tagged).
- **Closed or Merged MRs**: What happens if a tagged comment is posted on a closed or merged MR? (The system should acknowledge the comment with an eyes reaction if readable, and reply informing the user that the MR is closed/merged if actions cannot be applied).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST monitor merge request comments and review notes across target repositories (both GitLab Merge Requests and GitHub Pull Requests) for configured mention tags (`@darkgravity` or alias `@antigravity`, case-insensitive, matched anywhere in comment body).
- **FR-002**: System MUST immediately apply an eyes reaction indicator (`👀`) to any comment containing the mention tag to signal that the message has been read and queued (using GitLab Award Emoji API `:eyes:` or GitHub Reactions API `eyes`).
- **FR-003**: System MUST prevent duplicate reactions or duplicate task executions for the same comment event using persistent state tracking (e.g. processed comment IDs and timestamps).
- **FR-004**: System MUST ignore comments posted by Dark Gravity itself to eliminate potential infinite loops.
- **FR-005**: System MUST extract the intent, instruction, or query from the tagged comment and pass it to the agent execution pipeline.
- **FR-006**: System MUST post the agent's outcome, analysis, or conversational answer as a single comprehensive threaded reply directly under the user's original comment upon completion, avoiding interim progress comments in the thread.
- **FR-007**: System MUST support multi-turn conversation within the same MR/PR thread by including prior thread history as context for the agent.
- **FR-008**: System MUST support configurable mention tags, polling intervals, and reaction emojis per repository configuration.
- **FR-009**: System MUST operate via an outbound polling loop against active merge requests and pull requests, eliminating the requirement for exposed inbound public webhook listeners.
- **FR-010**: System MUST abstract platform-specific APIs via a unified GitProvider interface supporting both GitLab (v4 REST API) and GitHub (REST/GraphQL API).
- **FR-011**: System MUST report task or execution failures with structured error responses directly in the MR thread, providing diagnostic details and actionable remediation instructions.

### Key Entities *(include if feature involves data)*

- **GitProviderType**: Enumeration of supported hosting platforms (`GitLab`, `GitHub`).
- **MergeRequestComment**: Represents an individual note or comment on a merge request or pull request. Key attributes include comment ID, provider type, merge request reference, author, timestamp, body text, thread/discussion ID, and processed status.
- **AgentInteractionSession**: Tracks the ongoing conversational state and history associated with a specific MR discussion thread, linking comments to dispatched factory tasks.
- **CommentReaction**: Represents an acknowledgment mark placed on a comment, capturing target comment ID, provider type, reaction type (e.g. eyes `👀`), and timestamp.
- **PollingCursor**: Stores the last-scanned comment ID or timestamp per repository and MR/PR to ensure efficient incremental polling.
- **ExecutionOutcome**: Represents the result of a dispatched agent action, containing status (`Success`, `Failure`, `InvalidCommand`), generated artifacts, diagnostics, and remediation suggestions.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 95% of tagged comments receive the "eyes" (`👀`) reaction acknowledgment within 1 polling interval (default $\le 10$ seconds) of being posted.
- **SC-002**: 100% of comments posted by the agent itself are ignored, ensuring zero circular loop comments.
- **SC-003**: Dark Gravity delivers a contextual reply to user queries in the merge request thread within 30 seconds of processing start for standard queries.
- **SC-004**: Zero duplicated reactions or multiple duplicate replies for a single user comment across consecutive polling intervals.
- **SC-005**: Operates with 0 inbound open firewall ports or ingress webhooks, functioning completely from inside secure private networks.
- **SC-006**: Thread cleanliness: exactly 0 ephemeral status or placeholder comments posted during execution; only the final comprehensive outcome note is published.
- **SC-007**: 100% of execution failures produce an explanatory diagnosis and actionable remediation advice in the thread.

## Assumptions

- Target git hosting platforms (GitLab and GitHub) provide API support for adding comment reactions (`👀`) and threaded notes via polling.
- The default trigger handles are `@darkgravity` and `@antigravity`, case-insensitive.
- Dark Gravity operates with dedicated bot or service account credentials with permissions to read MRs/PRs, add reactions, and post notes.
- Comments that do not mention the configured tag are treated as regular human-to-human discussion and are bypassed without reaction or response.
