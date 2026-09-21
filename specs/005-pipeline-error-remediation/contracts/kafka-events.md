# Contract: Pipeline Remediation Kafka Events

**Feature**: `005-pipeline-error-remediation`

This document defines the Kafka event schemas for pipeline remediation events published to existing topics.

## Topic: `mission-input`

### Event: Pipeline Remediation Mission

Published when a classified pipeline error is deemed remediable and a remediation mission is triggered.

```json
{
  "mission_id": "remediation-lgcorzo-rust_CACD_autonomous_factory-12345",
  "goal": "Fix lint_violation error in lgcorzo/rust_CACD_autonomous_factory: error[clippy::unused_variable] in src/main.rs:42",
  "repository_path": "lgcorzo/rust_CACD_autonomous_factory",
  "source_platform": "pipeline_remediation",
  "pipeline_failure": {
    "source_platform": "github",
    "repository": "lgcorzo/rust_CACD_autonomous_factory",
    "run_id": 12345,
    "workflow_name": "CI/CD Pipeline",
    "failing_job": "Rust CI (Lint & Test)",
    "failing_step": "Lint with Clippy",
    "run_url": "https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/runs/12345",
    "detected_at": "2026-09-21T23:00:00Z"
  },
  "error_classification": {
    "category": "lint_violation",
    "sub_type": "clippy_rule",
    "file_path": "src/main.rs",
    "line_number": 42,
    "rule_name": "unused_variable",
    "test_name": null,
    "error_fingerprint": "a1b2c3d4e5f6",
    "is_remediable": true
  },
  "verifiable_credential": {
    "id": "vc:remediation:lgcorzo-rust_CACD_autonomous_factory-12345",
    "issuer": "did:factory:dark-gravity-euskadi",
    "credential_subject": {
      "id": "nhi:pipeline:github:12345",
      "roles": ["remediation_executor"],
      "allowed_namespaces": ["agents", "production"]
    },
    "proof": { "..." : "Ed25519 signature" }
  }
}
```

## Topic: `mission-artifact`

### Event: Remediation Outcome

Published when a remediation mission completes (success or failure).

```json
{
  "mission_id": "remediation-lgcorzo-rust_CACD_autonomous_factory-12345",
  "event_type": "remediation_outcome",
  "result": "success",
  "error_category": "lint_violation",
  "repository": "lgcorzo/rust_CACD_autonomous_factory",
  "pr_url": "https://github.com/lgcorzo/rust_CACD_autonomous_factory/pull/99",
  "fix_duration_secs": 180,
  "pipeline_passed": true,
  "completed_at": "2026-09-21T23:03:00Z"
}
```

## Topic: `agent-thought`

### Event: Classification Reasoning

Published when the classifier analyzes a pipeline error log.

```json
{
  "mission_id": "remediation-lgcorzo-rust_CACD_autonomous_factory-12345",
  "agent": "pipeline-classifier",
  "thought": "Classified as lint_violation (clippy::unused_variable) with confidence: regex_match. File: src/main.rs:42. Remediable: true.",
  "timestamp": "2026-09-21T23:00:01Z"
}
```
