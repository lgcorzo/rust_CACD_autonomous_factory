# Research: Work Item Progress Comments & Remote Branch GitOps Delivery

**Feature**: `001-workitem-status-and`  
**Date**: 2026-09-18  
**Author**: Antigravity (Advanced Agentic Coding) & NotebookLM Research Engine

---

## 1. Context & Problem Statement

In the Dark Gravity autonomous software factory (`rust_CACD_autonomous_factory`), missions are ingested from remote code platforms (GitLab / GitHub) by `GitPlatformPoller` and executed across a 5-phase Hatchet DAG:
1. `rustant-plan`
2. `zeroclaw-execute`
3. `zeroclaw-validate`
4. `rustant-review`
5. `factory-deliver`

Currently, two severe operational deficiencies exist:
1. **Work Item Black Box**: During execution, the originating issue (e.g. GitLab Issue #1 in `lgcorzo/lince-rs`) receives no progress comments or milestone markers. Users cannot observe planning, test validation, or review progress without inspecting raw Kubernetes pod logs.
2. **Phantom Branch & Mock PR Delivery**: In Phase 5 (`factory-deliver`), the code previously emitted a mock URL (`https://gitlab.com/repo/merge_requests/{mission_id}`) without creating a remote Git branch, without committing changes to GitLab/GitHub, and without opening a real Merge Request. Consequently, the target repository shows no delivery branch and no MR.

---

## 2. Research Decisions & Rationale

### Decision 1: Issue Commenting Strategy (Incremental Milestones)
- **Decision**: Post a discrete, structured Markdown comment at each major DAG phase transition (Start/Ingestion, Planning Approved, Testing Validated, Review Verdict, and Final Delivery/Failure).
- **Rationale**: 
  - Preserves an immutable, audit-compliant timeline on the issue tracker.
  - Triggers native notification feeds (email, webhook notifications) on GitLab/GitHub for human maintainers.
  - Matches Dark Gravity's HITL (Human-in-the-Loop) Governance Model (Vertex 1 and Vertex 4).
- **Alternatives Considered**:
  - *Single in-place updated comment*: Overwrites earlier state, loses historical execution timing, and does not trigger notification events on platforms like GitLab.
  - *PR-only comments*: Before Phase 5, no PR exists yet, leaving the first 4 phases completely invisible.

### Decision 2: Delivery Mechanism (Platform API vs Git CLI Shell Execution)
- **Decision**: Deliver code changes via authenticated platform REST APIs (`HttpGitlabClient` and `HttpGithubClient`):
  - GitLab: `POST /api/v4/projects/:id/repository/branches` to create branch, `POST /api/v4/projects/:id/repository/commits` to commit file actions atomically, and `POST /api/v4/projects/:id/merge_requests` to create the MR.
  - GitHub: `POST /repos/:owner/:repo/git/refs` to create ref, `POST /repos/:owner/:repo/git/commits` (or Git Trees/Blobs) or Contents API, and `POST /repos/:owner/:repo/pulls` to create PR.
- **Rationale**:
  - Eliminates reliance on host shell `git` binary and SSH/HTTPS credential helper configuration inside worker pods.
  - Prevents credential leaks in shell process tables or temporary disk footprints.
  - Supports atomic multi-file commit actions directly in GitLab REST API in a single HTTP request.
  - Fully compliant with gVisor sandbox memory clamps (`RAM <= 30Mi`).
- **Alternatives Considered**:
  - *Raw `git push` via shell*: Requires git CLI installed, disk workspace cloning, personal access token embedded in URL (security risk), and SSH key mounting.

### Decision 3: Work Item Context Propagation
- **Decision**: Extend `MissionInput` and `PolledIssueEvent` to carry provenance metadata:
  - `source_platform`: `"gitlab"` | `"github"`
  - `repository`: `"owner/repo"` or `"group/project"` (e.g., `"lgcorzo/lince-rs"`)
  - `issue_id`: `u64`
  - `issue_number`: `u64`
- **Rationale**:
  - Downstream Hatchet tasks need to know which platform, repository, and issue number to post milestone comments to and where to push the delivery branch.
  - Currently `MissionInput` only has `mission_id`, `goal`, and `repository_path`. Propagating `source_platform`, `repository`, and `issue_number` solves this cleanly and deterministically.

### Decision 4: Resilience & Non-Blocking Failure
- **Decision**: Milestone comments must be executed with graceful error logging: if a comment API call fails (e.g. platform rate limit or transient network glitch), the failure must be logged as a warning, but must NOT terminate or fail the mission DAG.
- **Rationale**: Progress telemetry must never become a single point of failure for core algorithm synthesis and verification.

---

## 3. Best Practices & API Specifications

### GitLab REST v4 Endpoints
1. **Issue Note Creation**:
   - `POST /api/v4/projects/:id/issues/:issue_iid/notes`
   - Body: `{"body": "markdown_text"}`
2. **Branch Creation**:
   - `POST /api/v4/projects/:id/repository/branches`
   - Body: `{"branch": "branch_name", "ref": "main"}`
3. **Commit Creation (Atomic Multi-File)**:
   - `POST /api/v4/projects/:id/repository/commits`
   - Body:
     ```json
     {
       "branch": "branch_name",
       "commit_message": "feat: [Dark Gravity] autonomous mission delivery (closes #1)",
       "actions": [
         {
           "action": "create",
           "file_path": "path/to/file.rs",
           "content": "..."
         }
       ]
     }
     ```
4. **Merge Request Creation**:
   - `POST /api/v4/projects/:id/merge_requests`
   - Body:
     ```json
     {
       "source_branch": "branch_name",
       "target_branch": "main",
       "title": "feat: [Dark Gravity] ...",
       "description": "Closes #1\n\nAutomated delivery by Dark Gravity autonomous factory."
     }
     ```

### GitHub REST v3 Endpoints
1. **Issue Comment Creation**:
   - `POST /repos/:owner/:repo/issues/:issue_number/comments`
   - Body: `{"body": "markdown_text"}`
2. **Branch (Ref) Creation**:
   - `POST /repos/:owner/:repo/git/refs`
   - Body: `{"ref": "refs/heads/branch_name", "sha": "base_sha"}`
3. **Pull Request Creation**:
   - `POST /repos/:owner/:repo/pulls`
   - Body: `{"title": "...", "head": "branch_name", "base": "main", "body": "..."}`
