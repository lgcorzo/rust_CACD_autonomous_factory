use factory_cli::evidence::{
    EvidenceRecord, RequirementState, VerificationPolicyConfig, validate_evidence_record,
    validate_requirement_state,
};
use serde_json::json;

fn valid_evidence_json() -> serde_json::Value {
    json!({
        "schema_version": "1.0",
        "evidence_id": "550e8400-e29b-41d4-a716-446655440000",
        "requirement_id": "FR-003",
        "collected_at": "2026-11-06T17:30:00Z",
        "source_revision": {
            "repository": "lgcorzo/rust_CACD_autonomous_factory",
            "branch": "011-verified-autonomous-execution",
            "commit": "00b5f08ab7d2ea39b6862f13a78a932ca70d6154"
        },
        "deployment_revision": null,
        "environment": {
            "kind": "ci",
            "identifier": "github-actions-ubuntu-latest"
        },
        "verification_class": "integration",
        "method": {
            "tool": "cargo-test",
            "command_or_test": "cargo test -p factory-mcp-server --test run_tests_security_tests"
        },
        "outcome": "passed",
        "exit_code": 0,
        "artifacts": [
            {
                "uri": "file://crates/factory-mcp-server/tests/run_tests_security_tests.rs",
                "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "media_type": "text/x-rust"
            }
        ],
        "collector": {
            "id": "ci-runner-01",
            "role": "test_runner"
        },
        "limitations": []
    })
}

fn valid_verified_state_json() -> serde_json::Value {
    json!({
        "schema_version": "1.0",
        "requirement_id": "FR-003",
        "status": "VERIFIED",
        "implementation_agent_id": "antigravity-impl-01",
        "evidence_refs": [
            "550e8400-e29b-41d4-a716-446655440000"
        ],
        "transitions": [
            {
                "from": null,
                "to": "OPEN",
                "at": "2026-11-06T16:00:00Z",
                "actor_id": "spec-author",
                "reason": "Initial requirement registration"
            },
            {
                "from": "OPEN",
                "to": "IMPLEMENTED",
                "at": "2026-11-06T16:30:00Z",
                "actor_id": "antigravity-impl-01",
                "reason": "Implemented SandboxDriver routing in run_tests.rs"
            },
            {
                "from": "IMPLEMENTED",
                "to": "TESTED",
                "at": "2026-11-06T17:00:00Z",
                "actor_id": "ci-runner-01",
                "reason": "Passed integration security test suite"
            },
            {
                "from": "TESTED",
                "to": "VERIFIED",
                "at": "2026-11-06T17:35:00Z",
                "actor_id": "independent-auditor-01",
                "reason": "Independently verified code, test output, and revision"
            }
        ],
        "verified_by": {
            "id": "independent-auditor-01",
            "role": "independent_verifier",
            "verified_at": "2026-11-06T17:35:00Z",
            "evidence_refs": [
                "550e8400-e29b-41d4-a716-446655440000"
            ]
        },
        "invalidated_at": null
    })
}

#[test]
fn test_valid_evidence_record_and_verified_state_pass() {
    let ev: EvidenceRecord = serde_json::from_value(valid_evidence_json()).unwrap();
    validate_evidence_record(&ev, None).expect("valid evidence record should pass");

    let state: RequirementState = serde_json::from_value(valid_verified_state_json()).unwrap();
    let policy = VerificationPolicyConfig {
        expected_source_commit: Some("00b5f08ab7d2ea39b6862f13a78a932ca70d6154".to_string()),
        required_classes: vec!["integration".to_string()],
    };
    validate_requirement_state(&state, &[ev], &policy)
        .expect("valid verified state with matching evidence should pass");
}

#[test]
fn test_rejects_short_or_non_hex_commit_sha_and_invalid_requirement_id() {
    let mut bad_sha = valid_evidence_json();
    bad_sha["source_revision"]["commit"] = json!("00b5f08a"); // short SHA
    let ev: EvidenceRecord = serde_json::from_value(bad_sha).unwrap();
    let err = validate_evidence_record(&ev, None).unwrap_err().to_string();
    assert!(
        err.contains("commit") || err.contains("40"),
        "expected commit SHA validation failure, got: {err}"
    );

    let mut bad_req = valid_evidence_json();
    bad_req["requirement_id"] = json!("REQ-99");
    let ev2: EvidenceRecord = serde_json::from_value(bad_req).unwrap();
    assert!(validate_evidence_record(&ev2, None).is_err());
}

#[test]
fn test_live_runtime_requires_deployment_revision() {
    let mut live_no_deploy = valid_evidence_json();
    live_no_deploy["verification_class"] = json!("live_runtime");
    live_no_deploy["deployment_revision"] = json!(null);
    let ev: EvidenceRecord = serde_json::from_value(live_no_deploy).unwrap();
    let err = validate_evidence_record(&ev, None).unwrap_err().to_string();
    assert!(
        err.contains("deployment_revision"),
        "expected deployment_revision error for live_runtime, got: {err}"
    );
}

#[test]
fn test_rejects_forbidden_implemented_to_verified_transition_jump() {
    let ev: EvidenceRecord = serde_json::from_value(valid_evidence_json()).unwrap();
    let mut bad_state = valid_verified_state_json();
    bad_state["transitions"] = json!([
        {
            "from": null,
            "to": "OPEN",
            "at": "2026-11-06T16:00:00Z",
            "actor_id": "spec-author",
            "reason": "Init"
        },
        {
            "from": "OPEN",
            "to": "IMPLEMENTED",
            "at": "2026-11-06T16:30:00Z",
            "actor_id": "antigravity-impl-01",
            "reason": "Code written"
        },
        {
            "from": "IMPLEMENTED",
            "to": "VERIFIED",
            "at": "2026-11-06T17:35:00Z",
            "actor_id": "independent-auditor-01",
            "reason": "Skipped TESTED!"
        }
    ]);
    let state: RequirementState = serde_json::from_value(bad_state).unwrap();
    let err = validate_requirement_state(&state, &[ev], &VerificationPolicyConfig::default())
        .unwrap_err()
        .to_string();
    assert!(
        err.contains("IMPLEMENTED") && err.contains("VERIFIED"),
        "expected forbidden transition error, got: {err}"
    );
}

#[test]
fn test_rejects_self_reported_verifier_matching_implementer() {
    let ev: EvidenceRecord = serde_json::from_value(valid_evidence_json()).unwrap();
    let mut self_verified = valid_verified_state_json();
    self_verified["verified_by"]["id"] = json!("antigravity-impl-01"); // same as implementation_agent_id
    let state: RequirementState = serde_json::from_value(self_verified).unwrap();
    let err = validate_requirement_state(&state, &[ev], &VerificationPolicyConfig::default())
        .unwrap_err()
        .to_string();
    assert!(
        err.contains("independent") || err.contains("implementation_agent_id"),
        "expected independent verifier rejection, got: {err}"
    );
}

#[test]
fn test_rejects_mocked_failed_or_stale_evidence_for_verified_requirement() {
    // 1. Mocked outcome cannot promote to VERIFIED
    let mut mocked_ev_json = valid_evidence_json();
    mocked_ev_json["verification_class"] = json!("mock");
    mocked_ev_json["outcome"] = json!("mocked");
    let mocked_ev: EvidenceRecord = serde_json::from_value(mocked_ev_json).unwrap();
    let state: RequirementState = serde_json::from_value(valid_verified_state_json()).unwrap();
    let policy = VerificationPolicyConfig {
        expected_source_commit: Some("00b5f08ab7d2ea39b6862f13a78a932ca70d6154".to_string()),
        required_classes: vec!["integration".to_string()],
    };
    assert!(
        validate_requirement_state(&state, &[mocked_ev], &policy).is_err(),
        "mocked evidence must not satisfy VERIFIED state"
    );

    // 2. Failed outcome cannot promote to VERIFIED
    let mut failed_ev_json = valid_evidence_json();
    failed_ev_json["outcome"] = json!("failed");
    failed_ev_json["exit_code"] = json!(1);
    let failed_ev: EvidenceRecord = serde_json::from_value(failed_ev_json).unwrap();
    assert!(
        validate_requirement_state(&state, &[failed_ev], &policy).is_err(),
        "failed evidence must not satisfy VERIFIED state"
    );

    // 3. Stale commit SHA cannot promote to VERIFIED
    let stale_ev: EvidenceRecord = serde_json::from_value(valid_evidence_json()).unwrap();
    let stale_policy = VerificationPolicyConfig {
        expected_source_commit: Some("ffffffffffffffffffffffffffffffffffffffff".to_string()),
        required_classes: vec!["integration".to_string()],
    };
    let err = validate_requirement_state(&state, &[stale_ev], &stale_policy)
        .unwrap_err()
        .to_string();
    assert!(
        err.contains("stale") || err.contains("revision") || err.contains("commit"),
        "expected stale revision rejection, got: {err}"
    );
}
