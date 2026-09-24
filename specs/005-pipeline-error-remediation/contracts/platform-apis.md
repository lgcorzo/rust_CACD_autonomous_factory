# Contract: Extended Platform Client APIs

**Feature**: `005-pipeline-error-remediation`

This document defines the new methods added to the existing `GithubClient` and `GitlabClient` traits.

## GithubClient Trait Extensions

### `list_failed_workflow_runs`

```
GET /repos/{owner}/{repo}/actions/runs?status=failure&created=>={since}
Authorization: Bearer {token}
Accept: application/vnd.github.v3+json
```

**Parameters**:
- `repo`: `&str` — Repository in `owner/repo` format
- `since`: `Option<DateTime<Utc>>` — Only return runs created after this timestamp

**Returns**: `Vec<GithubWorkflowRun>` — List of failed workflow runs

**Error Conditions**:
- `404` → Repository not found or Actions not enabled
- `403` → Token lacks `actions:read` scope
- `422` → Invalid query parameters

### `get_workflow_run_jobs`

```
GET /repos/{owner}/{repo}/actions/runs/{run_id}/jobs
Authorization: Bearer {token}
Accept: application/vnd.github.v3+json
```

**Returns**: `Vec<GithubWorkflowJob>` — Jobs within the run, each with steps and conclusions

### `get_job_log`

```
GET /repos/{owner}/{repo}/actions/jobs/{job_id}/logs
Authorization: Bearer {token}
Accept: application/vnd.github.v3+json
```

**Returns**: `String` — Raw text log output (can be large; truncate to 10KB)

**Important**: This endpoint returns a redirect (302) to a temporary download URL. The `reqwest` client must follow redirects (default behavior).

---

## GitlabClient Trait Extensions

### `list_failed_pipelines`

```
GET /projects/{id}/pipelines?status=failed&updated_after={since}
PRIVATE-TOKEN: {token}
```

**Parameters**:
- `project_id`: `&str` — URL-encoded project path or numeric ID
- `since`: `Option<DateTime<Utc>>` — Only return pipelines updated after this timestamp

**Returns**: `Vec<GitlabPipeline>` — List of failed pipelines

### `get_pipeline_jobs`

```
GET /projects/{id}/pipelines/{pipeline_id}/jobs
PRIVATE-TOKEN: {token}
```

**Returns**: `Vec<GitlabPipelineJob>` — Jobs within the pipeline with name, stage, and status

### `get_job_trace`

```
GET /projects/{id}/jobs/{job_id}/trace
PRIVATE-TOKEN: {token}
```

**Returns**: `String` — Raw job trace log output (truncate to 10KB)

**Note**: Returns `Content-Type: text/plain` directly (no redirect).
