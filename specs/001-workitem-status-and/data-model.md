# Data Model: Work Item Status Comments & Remote GitOps Delivery

**Feature**: `001-workitem-status-and`  
**Date**: 2026-09-18  

---

## 1. Entities & Data Structures

### 1.1 `WorkItemContext`
Represents the provenance of the issue that initiated the mission.

```rust
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WorkItemContext {
    pub source_platform: String, // "gitlab" | "github"
    pub repository: String,      // e.g. "lgcorzo/lince-rs"
    pub issue_id: Option<u64>,
    pub issue_number: u64,
}
```

### 1.2 `MissionInput` (Extended)
Updated to propagate work item context across Hatchet DAG tasks.

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MissionInput {
    pub mission_id: Option<String>,
    pub goal: String,
    pub repository_path: String,
    pub source_platform: Option<String>,
    pub repository: Option<String>,
    pub issue_number: Option<u64>,
}
```

### 1.3 `GitlabCommitAction`
Used for atomic multi-file commits in GitLab REST API (`POST /projects/:id/repository/commits`).

```rust
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GitlabCommitAction {
    pub action: String, // "create" | "update" | "delete"
    pub file_path: String,
    pub content: Option<String>,
}
```

### 1.4 `GitDeliveryResult`
Contains details of the created branch, commit, and opened Merge Request / Pull Request.

```rust
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GitDeliveryResult {
    pub branch_name: String,
    pub commit_sha: Option<String>,
    pub mr_iid: u64,
    pub mr_web_url: String,
}
```

---

## 2. Platform Client Trait Extensions

### 2.1 `GitlabClient` Extension
In `crates/factory-infrastructure/src/gitlab.rs`:

```rust
#[async_trait]
pub trait GitlabClient: Send + Sync {
    // Existing methods:
    async fn create_issue(&self, project_id: &str, title: &str, description: &str) -> anyhow::Result<GitlabIssue>;
    async fn create_issue_with_labels(&self, project_id: &str, title: &str, description: &str, labels: &[String]) -> anyhow::Result<GitlabIssue>;
    async fn list_open_issues(&self, project_id: &str, labels: Option<String>) -> anyhow::Result<Vec<GitlabIssue>>;
    async fn list_issues_updated_since(&self, project_id: &str, labels: Option<String>, since: Option<DateTime<Utc>>) -> anyhow::Result<Vec<GitlabIssue>>;
    async fn list_active_merge_requests(&self, project_id: &str) -> anyhow::Result<Vec<GitlabMergeRequest>>;
    async fn list_merge_request_notes(&self, project_id: &str, mr_iid: u64, since: Option<DateTime<Utc>>) -> anyhow::Result<Vec<GitlabNote>>;
    async fn post_merge_request_note(&self, project_id: &str, mr_iid: u64, body: &str) -> anyhow::Result<GitlabNote>;

    // NEW METHODS:
    async fn post_issue_note(&self, project_id: &str, issue_iid: u64, body: &str) -> anyhow::Result<GitlabNote>;
    async fn create_branch(&self, project_id: &str, branch: &str, ref_branch: &str) -> anyhow::Result<String>;
    async fn create_commit_files(
        &self,
        project_id: &str,
        branch: &str,
        commit_message: &str,
        actions: &[GitlabCommitAction],
    ) -> anyhow::Result<String>;
    async fn create_merge_request(
        &self,
        project_id: &str,
        source_branch: &str,
        target_branch: &str,
        title: &str,
        description: &str,
    ) -> anyhow::Result<GitlabMergeRequest>;
}
```

### 2.2 `GithubClient` Extension
In `crates/factory-infrastructure/src/github.rs`:

```rust
#[async_trait]
pub trait GithubClient: Send + Sync {
    // Existing methods:
    async fn create_issue(&self, repo: &str, title: &str, body: &str) -> anyhow::Result<GithubIssue>;
    async fn list_open_issues(&self, repo: &str, labels: Option<String>) -> anyhow::Result<Vec<GithubIssue>>;
    async fn list_issues_updated_since(&self, repo: &str, labels: Option<String>, since: Option<DateTime<Utc>>) -> anyhow::Result<Vec<GithubIssue>>;
    async fn list_active_pull_requests(&self, repo: &str) -> anyhow::Result<Vec<GithubPullRequest>>;
    async fn list_pull_request_comments(&self, repo: &str, pr_number: u64, since: Option<DateTime<Utc>>) -> anyhow::Result<Vec<GithubComment>>;
    async fn post_pull_request_comment(&self, repo: &str, pr_number: u64, body: &str) -> anyhow::Result<GithubComment>;
    async fn create_pull_request(&self, repo: &str, title: &str, head: &str, base: &str, body: &str) -> anyhow::Result<String>;

    // NEW METHODS:
    async fn post_issue_comment(&self, repo: &str, issue_number: u64, body: &str) -> anyhow::Result<GithubComment>;
    async fn create_branch(&self, repo: &str, branch: &str, base_branch: &str) -> anyhow::Result<String>;
}
```

---

## 3. State & Milestone Transitions

```
[Issue Ingested]
       │
       ▼
[Comment: Mission Ingested & Scheduled]
       │
       ▼
[rustant-plan] ──► [Comment: Planning Completed & Architecture Formulated]
       │
       ▼
[zeroclaw-execute] ──► [Comment: Code Mutations Generated]
       │
       ▼
[zeroclaw-validate] ──► [Comment: Test Suite Verified (100% Pass)]
       │
       ▼
[rustant-review] ──► [Comment: Security Verdict Approved (SAST >= 8.0)]
       │
       ▼
[factory-deliver]
       ├──► Create Remote Branch (`specs/NNN-...` or `mission-...`)
       ├──► Commit Generated Code Files
       ├──► Open Merge Request / Pull Request
       └──► [Comment: Final Delivery Delivered + Clickable MR Link]
```
