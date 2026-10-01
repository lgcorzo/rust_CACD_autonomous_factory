# factory-application::gitlab_verifier — GitLab Integration Verifier

> **Source**: `crates/factory-application/src/gitlab_verifier.rs`  
> **Layer**: Application  
> **Role**: Automated health, authentication, permissions, and CI/CD verification for GitLab Enterprise and cloud instances.

---

## Verification Sequence

```mermaid
sequenceDiagram
    autonumber
    participant Op as CLI / Poller
    participant GV as GitlabVerifier
    participant GC as GitlabClient
    participant GL as GitLab API

    Op->>GV: verify_all(projects)
    GV->>GC: check_current_user()
    GC->>GL: GET /api/v4/user
    GL-->>GV: User profile / token validity
    loop For each target project
        GV->>GC: get_project(project_path)
        GV->>GC: list_merge_requests(project_path)
        GV->>GC: verify_webhook_endpoints()
    end
    GV-->>Op: GitlabVerificationReport
```

## Verification Status Types

- `VerificationStatus::Passed`: All connectivity, token, and project queries succeeded.
- `VerificationStatus::Partial`: Authentication succeeded but specific repositories or permissions failed.
- `VerificationStatus::Failed`: Critical authentication or network failure.

---

> *Related: [gitlab.rs](crates_factory-infrastructure_src_gitlab.md) · [Tactical Design](TACTICAL-DESIGN.md)*
