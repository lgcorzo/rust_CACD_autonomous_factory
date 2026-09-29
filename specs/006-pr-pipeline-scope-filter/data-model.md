# Data Model: PR Pipeline Scope Filtering for Autonomous Remediation

**Feature**: `006-pr-pipeline-scope-filter`
**Date**: 2026-09-29

## Entity Definitions & Schema Extensions

### 1. `PipelineFailureEvent` Extension (Crate: `factory-core`)

Extends the core domain event representing a pipeline failure to incorporate direct correlation metadata with the originating Pull Request or Merge Request.

```rust
/// A detected CI/CD pipeline failure scoped to an active PR/MR.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PipelineFailureEvent {
    /// Source platform: `"github"` or `"gitlab"`
    pub source_platform: String,
    /// Repository in `owner/repo` or project path format
    pub repository: String,
    /// Platform-specific run/pipeline ID
    pub run_id: u64,
    /// Name of the workflow or pipeline ref
    pub workflow_name: String,
    /// Associated Pull Request or Merge Request number/IID
    pub pr_number: Option<u64>,
    /// Head branch name of the PR/MR
    pub head_branch: Option<String>,
    /// Head commit SHA of the PR/MR run
    pub head_sha: Option<String>,
    /// Name of the failing job
    pub failing_job: String,
    /// Name of the failing step (GitHub Actions only)
    pub failing_step: Option<String>,
    /// Truncated error log output (max 10KB)
    pub error_log: String,
    /// URL to the failed run
    pub run_url: String,
    /// When the failure was detected
    pub detected_at: DateTime<Utc>,
}
```

### 2. GitHub Actions Workflow Run Deserialization (Crate: `factory-infrastructure`)

Extends `GithubWorkflowRun` to capture PR linkage metadata provided by the GitHub API.

```rust
/// GitHub Actions workflow run API representation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWorkflowRun {
    pub id: u64,
    pub name: Option<String>,
    pub status: String,
    pub conclusion: Option<String>,
    pub html_url: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub head_branch: Option<String>,
    pub head_sha: Option<String>,
    pub event: Option<String>,
    #[serde(default)]
    pub pull_requests: Vec<GithubWorkflowRunPr>,
}

/// PR reference embedded in GitHub workflow run payload.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWorkflowRunPr {
    pub id: u64,
    pub number: u64,
    pub head: Option<GithubWorkflowBranchRef>,
    pub base: Option<GithubWorkflowBranchRef>,
}

/// Branch reference in GitHub workflow run PR object.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWorkflowBranchRef {
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub sha: String,
}
```

### 3. GitLab Pipeline and Merge Request Extensions (Crate: `factory-infrastructure`)

Extends `GitlabPipeline` and `GitlabMergeRequest` to capture branch and SHA references.

```rust
/// GitLab CI pipeline run with branch and SHA references.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitlabPipeline {
    pub id: u64,
    pub status: String,
    pub web_url: String,
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    pub sha: Option<String>,
    pub source: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// GitLab Merge Request with source branch and SHA.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitlabMergeRequest {
    pub id: u64,
    pub iid: u64,
    pub title: String,
    pub description: Option<String>,
    pub web_url: String,
    pub state: String,
    pub source_branch: Option<String>,
    pub sha: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
}
```

### 4. `PipelineScopeFilter` Domain Service (Crate: `factory-core` / `factory-application`)

Encapsulates the filtering logic to determine whether a pipeline run belongs to an active PR.

```rust
/// Evaluation result for a candidate pipeline run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineScopeVerdict {
    /// In-scope: Belongs to active PR.
    InScope {
        pr_number: u64,
        head_branch: String,
        head_sha: Option<String>,
    },
    /// Out-of-scope: Not associated with any active tracked PR.
    OutOfScope {
        reason: String,
    },
}

pub struct PipelineScopeFilter;

impl PipelineScopeFilter {
    /// Match a GitHub workflow run against active PRs.
    pub fn evaluate_github(
        run: &GithubWorkflowRun,
        active_prs: &[GithubPullRequest],
    ) -> PipelineScopeVerdict {
        // 1. Direct pull_requests attribute matching
        for pr_ref in &run.pull_requests {
            if let Some(matched) = active_prs.iter().find(|p| p.number == pr_ref.number) {
                let branch = pr_ref
                    .head
                    .as_ref()
                    .map(|h| h.ref_name.clone())
                    .or_else(|| run.head_branch.clone())
                    .unwrap_or_default();
                return PipelineScopeVerdict::InScope {
                    pr_number: matched.number,
                    head_branch: branch,
                    head_sha: run.head_sha.clone(),
                };
            }
        }

        // 2. Branch name matching against active PRs
        if let Some(head_branch) = &run.head_branch {
            // Check if any active PR title/branch matches
            if let Some(matched) = active_prs.iter().find(|p| {
                // Also match if head_sha or branch pattern aligns
                p.body.as_deref().unwrap_or("").contains(head_branch)
            }) {
                return PipelineScopeVerdict::InScope {
                    pr_number: matched.number,
                    head_branch: head_branch.clone(),
                    head_sha: run.head_sha.clone(),
                };
            }
        }

        PipelineScopeVerdict::OutOfScope {
            reason: format!(
                "Run {} on branch '{:?}' has no matching active PR in [{}]",
                run.id,
                run.head_branch,
                active_prs.iter().map(|p| p.number.to_string()).collect::<Vec<_>>().join(", ")
            ),
        }
    }

    /// Match a GitLab pipeline run against active MRs.
    pub fn evaluate_gitlab(
        pipeline: &GitlabPipeline,
        active_mrs: &[GitlabMergeRequest],
    ) -> PipelineScopeVerdict {
        let pipeline_ref = pipeline.ref_.as_deref().unwrap_or("");
        let pipeline_sha = pipeline.sha.as_deref();

        for mr in active_mrs {
            let matches_branch = mr.source_branch.as_deref() == Some(pipeline_ref);
            let matches_sha = pipeline_sha.is_some() && mr.sha.as_deref() == pipeline_sha;

            if matches_branch || matches_sha {
                return PipelineScopeVerdict::InScope {
                    pr_number: mr.iid,
                    head_branch: pipeline_ref.to_string(),
                    head_sha: pipeline.sha.clone(),
                };
            }
        }

        PipelineScopeVerdict::OutOfScope {
            reason: format!(
                "Pipeline {} on ref '{}' has no matching active MR",
                pipeline.id, pipeline_ref
            ),
        }
    }
}
```
