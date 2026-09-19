# Phase 1 Data Model: GitLab Communication End-to-End Verification

**Branch**: `003-gitlab-communication-e2e` | **Date**: 2026-09-19

## Overview

This document specifies the domain data structures, event payloads, and diagnostic scorecard models utilized across the GitLab communication pipeline and its end-to-end verification harness.

---

## Entities & Models

### 1. `GitlabVerificationReport` (Diagnostic Scorecard)

Generated during live or hermetic end-to-end verification runs to report the health of all GitLab interfaces.

```rust
pub struct GitlabVerificationReport {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub gitlab_url: String,
    pub target_projects: Vec<String>,
    pub overall_status: VerificationStatus, // "PASSED" | "FAILED" | "PARTIAL"
    pub checks: Vec<GitlabCheckResult>,
    pub execution_duration_ms: u64,
}

pub struct GitlabCheckResult {
    pub check_name: String,        // e.g., "auth_handshake", "poll_issues", "mr_eyes_reaction"
    pub target: String,            // e.g., "lgcorzo/fastapi-autogen-team"
    pub status: CheckStatus,       // Success | Warning | Failure | Skipped
    pub latency_ms: u64,
    pub message: String,
    pub remediation_hint: Option<String>,
}

pub enum CheckStatus {
    Success,
    Warning,
    Failure,
    Skipped,
}
```

---

### 2. `PolledIssueEvent` (Issue Ingestion Model)

Represents an autonomous mission issue detected by the poller from GitLab.

```rust
pub struct PolledIssueEvent {
    pub source_platform: String,   // Always "gitlab" for this flow
    pub repository: String,        // e.g. "lgcorzo/fastapi-autogen-team" (project path with namespace)
    pub issue_id: u64,             // GitLab internal ID
    pub issue_number: u64,         // GitLab IID (user-visible issue number)
    pub title: String,
    pub body: String,
    pub labels: Vec<String>,       // e.g. ["autonomous-mission", "dark-gravity"]
    pub resource_limits: Option<ResourceLimits>, // CPU, RAM, Timeout extracted from description
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub html_url: String,
}
```

---

### 3. `PRCommentEvent` (MR Discussion Directive Model)

Represents a directive note extracted from a GitLab Merge Request discussion.

```rust
pub struct PRCommentEvent {
    pub source_platform: String,   // Always "gitlab"
    pub repository: String,        // Project path with namespace
    pub pr_number: u64,            // Merge Request IID
    pub comment_id: u64,           // Note ID
    pub author: String,            // Author username
    pub body: String,              // Raw comment content
    pub directive: PRDirective,    // Parsed directive enum
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub html_url: String,          // URL pointing directly to note
}
```

---

### 4. `PRDirective` (Command Parsing Model)

The parsed developer directive actionable by factory agents.

```rust
pub enum PRDirective {
    Spec { prompt: String },        // "/spec <prompt>" -> Rustant PO agent
    Refine { instruction: String },// "/refine <instruction>" -> ZeroClaw Dev agent
    Retry,                         // "/retry" -> Aethelgard verification loop restart
    Status,                        // "/status" -> Telemetry & DAG health report
    Validate,                      // "/validate" -> Full test suites & SAST audit
    Interact { prompt: String },   // "@darkgravity <prompt>" -> Conversational assistant
}
```

---

### 5. `GitlabAwardEmoji` & `GitlabNote` (GitLab REST Wire Entities)

Entities mapped directly to GitLab REST API v4 responses.

```rust
pub struct GitlabAwardEmoji {
    pub id: u64,
    pub name: String,              // "eyes"
    pub user: GitlabAuthor,
}

pub struct GitlabNote {
    pub id: u64,
    pub body: String,
    pub author: GitlabAuthor,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct GitlabAuthor {
    pub username: String,
}
```

---

### 6. `PollerSyncCursor` (State & Idempotency Model)

Tracks synchronization progress per repository and event type to prevent duplicate processing.

```rust
pub struct PollerSyncCursor {
    pub source_key: String,        // "gitlab:{project}:issues" or "gitlab:{project}:mr_notes"
    pub last_polled_at: chrono::DateTime<chrono::Utc>,
    pub last_processed_id: u64,
    pub processed_hashes: Vec<String>,
}
```
