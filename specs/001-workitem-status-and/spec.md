# Feature Specification: Work Item Progress Comments & Remote Branch GitOps Delivery

**Feature Branch**: `001-workitem-status-and`

**Created**: 2026-09-18

**Status**: Draft

**Input**: User description: "i want to update the status of the project in the comment of the workitem which starts the process, the agents have to write as comment in the work item the status, and in the test the branch has not being generated, the 5 delivery is done but i cannot see the branch in the gitlab where the project is located"

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Real-Time Status Comments on Originating Work Items (Priority: P1)

As a product owner or engineer tracking an autonomous mission,
I want the autonomous agents to post incremental status updates directly as comments onto the originating work item (GitLab/GitHub issue),
So that I can follow planning, execution, validation, and delivery progress directly from the issue tracker without inspecting cluster logs.

**Why this priority**: Without progress comments on the originating work item, users have zero visibility into whether the mission was received, currently planning, compiling, testing, or stuck, creating an operational black box.

**Independent Test**: Can be tested independently by submitting a work item (e.g. GitLab issue) to the factory, observing the workflow cycle, and verifying that formatted milestone comments appear on the issue at each phase transition.

**Acceptance Scenarios**:
1. **Given** a work item ingested by the outbound poller, **When** the planning phase (`RustantAgent`) begins, **Then** an initial comment is posted to the work item containing the mission ID, target repository, and high-level goal.
2. **Given** an ongoing autonomous mission, **When** each workflow phase completes (`Rustant Plan`, `ZeroClaw Execute`, `ZeroClaw Validate`, `Rustant Review`), **Then** a structured Markdown comment reporting the phase outcome, elapsed time, and next step is posted to the work item.
3. **Given** a phase failure or supervisor escalation, **When** an error occurs (such as validation failure or deadlock), **Then** a diagnostic comment is posted to the work item detailing the failure reason and remediation action.

---

### User Story 2 - Real Remote Git Branch Generation & Merge Request Delivery (Priority: P1)

As a repository maintainer,
I want Phase 5 (`factory-deliver`) to generate a real Git branch, commit the agent-produced code changes, push the branch to the remote repository, and open a real Merge Request / Pull Request,
So that I can review the diff and merge the autonomous mission code directly in GitLab or GitHub.

**Why this priority**: A mock delivery stub that does not push a real branch or open a real Merge Request provides zero actual software delivery value; the code remains stranded in temporary agent memory.

**Independent Test**: Can be tested by running an autonomous development task, waiting for Phase 5 (`factory-deliver`) to finish, and verifying via `git ls-remote` or the GitLab Web UI that the branch exists on the remote repository and the Merge Request is open with the expected commit diff.

**Acceptance Scenarios**:
1. **Given** an approved review from `RustantAgent`, **When** Phase 5 (`factory-deliver`) executes, **Then** the system checks out or creates an isolated branch (`mission/<mission_id>` or `feat/<issue_number>-<slug>`) targeting the target repository.
2. **Given** code modifications and artifacts generated during the mission, **When** delivery runs, **Then** the changes are committed with a semantic commit message referencing the originating issue (e.g. `feat: [Dark Gravity] ... (closes #<issue>)`).
3. **Given** committed modifications, **When** delivery executes git push, **Then** the branch is pushed to the remote repository over authenticated credentials.
4. **Given** a successfully pushed branch, **When** the platform client (GitLab/GitHub) is called, **Then** a real Merge Request is opened with target `main` (or default branch), linked to the originating issue, and the real MR URL is returned.
5. **Given** the opened Merge Request, **When** the delivery finishes, **Then** a final completion comment is posted to the originating work item with the direct link to the Merge Request.

---

### User Story 3 - Stashed Error Recovery & Failure Audit Trail (Priority: P2)

As a DevOps engineer investigating an agent failure,
I want unmerged or failing changes to be stashed with a deterministic reference tag and reported in the issue comments,
So that manual inspection and auto-remediation agents (like Jules) can inspect the exact failing workspace state.

**Why this priority**: Allows fast recovery without loss of partial work when validation fails after maximum retry attempts.

**Independent Test**: Can be tested by simulating a test failure in `ZeroClaw Validate`, ensuring the stashed branch/tag is created, and verifying the issue comment lists the stash reference.

**Acceptance Scenarios**:
1. **Given** a mission where validation fails 3 consecutive times, **When** the workflow terminates, **Then** the uncommitted workspace diff is stashed or tagged with `stuck-mission-<mission_id>`.
2. **Given** the stashed failure, **When** the error handler runs, **Then** the issue comment is updated with the stash name and test error snippet.

---

### Edge Cases

- **Platform API Rate-Limiting or Network Transient Failures**: When commenting on GitLab/GitHub issues, API calls must use retry with exponential backoff and must not crash the core mission pipeline if a status comment fails.
- **Branch Name Collision**: If an issue triggers multiple mission runs, the branch name must include a timestamp or short UUID suffix to avoid rejecting the git push.
- **No File Modifications / Empty Diff**: If the agent execution produced no code diff, Phase 5 must abort branch creation gracefully, avoid pushing an empty branch, and post an explanatory comment on the work item.
- **Missing or Invalid Repository Push Credentials**: If push permissions are denied (e.g., read-only token), Phase 5 must post a clear diagnostic comment indicating the permission error rather than failing silently with mock links.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST identify the originating platform (GitLab / GitHub), project/repository identifier, and issue number from the ingested work item metadata.
- **FR-002**: System MUST post an initial acknowledging comment to the originating work item immediately upon mission ingestion and DAG start.
- **FR-003**: System MUST post phase-transition status comments to the originating work item for:
  - Planning completion (`Rustant Plan` summary and artifact references)
  - Code execution completion (`ZeroClaw Execute` status and SAST forensic score)
  - Validation completion (`ZeroClaw Validate` test suite results)
  - Review completion (`Rustant Review` approval/rejection outcome)
- **FR-004**: System MUST format all status comments in clean, readable Markdown with standard icons, timestamps, and stage checklists.
- **FR-005**: Phase 5 (`factory-deliver`) MUST execute real Git operations against the target repository:
  - Create/checkout a dedicated mission branch
  - Commit all generated files and patches
  - Push the branch to the remote origin using platform authentication
- **FR-006**: Phase 5 (`factory-deliver`) MUST create a real remote Merge Request (GitLab) or Pull Request (GitHub) connecting the pushed branch to the default repository branch.
- **FR-007**: Phase 5 (`factory-deliver`) MUST post the final clickable Merge Request URL directly into the originating work item comment thread.
- **FR-008**: System MUST handle git credential authentication dynamically using configured platform tokens (`GITLAB_API_TOKEN` / `GITHUB_API_TOKEN`).
- **FR-009**: System MUST prevent mock or dummy PR URLs (`https://gitlab.com/repo/merge_requests/...`) from being emitted in delivery outputs.

---

## Success Criteria *(mandatory)*

- **SC-001**: 100% of autonomous missions initiated from a work item have at least 3 verified progress comments (Start, Validation, Delivery/Terminal) posted to the originating issue.
- **SC-002**: 100% of completed missions produce a visible, verifiable remote branch on the platform repository (e.g., `git ls-remote` confirms branch existence).
- **SC-003**: A genuine Merge Request / Pull Request is created on the target platform and linked in the originating work item within 30 seconds of review approval.
- **SC-004**: Zero mock URL strings or phantom branches are produced by the delivery engine.
- **SC-005**: In the event of validation failure, the originating work item displays a diagnostic summary and stash tag within 15 seconds of escalation.

---

## Key Entities *(mandatory)*

- **WorkItemContext**: Encapsulates platform (`gitlab` | `github`), repository path (`owner/repo`), issue ID, issue number, and security credential.
- **MissionProgressComment**: Structured comment payload containing mission ID, current DAG phase, execution status, forensic/test metrics, and elapsed duration.
- **GitDeliveryArtifact**: Encapsulates target repository URL, branch name, commit SHA, merge request ID, and merge request web URL.
- **PlatformGitClient**: Interface abstracting issue comments, branch creation/push, and merge request creation across GitLab and GitHub.

---

## Assumptions

- Target repositories provide push access through configured project or personal access tokens (`GITLAB_API_TOKEN` or `GITHUB_API_TOKEN`).
- Outbound poller passes the full work item provenance (`source_platform`, `repository`, `issue_number`) inside the `MissionInput` payload so downstream DAG tasks know where to post comments.
- Repositories allow branch creation and Merge Request opening via standard REST / GraphQL APIs.
