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
fn test_live_runtime_rejects_unit_test_runner_and_markdown_only_artifacts() {
    // 1. Rejects cargo-test / local environment for live_runtime even when deployment_revision is set
    let mut fake_live = valid_evidence_json();
    fake_live["verification_class"] = json!("live_runtime");
    fake_live["deployment_revision"] = json!({
        "repository": "lgcorzo/gitops_internal_lgcorzo",
        "branch": "main",
        "commit": "2716e548fa5bd7563ebfce232ee68a6a24bb11b2"
    });
    let ev1: EvidenceRecord = serde_json::from_value(fake_live.clone()).unwrap();
    let err1 = validate_evidence_record(&ev1, None)
        .unwrap_err()
        .to_string();
    assert!(
        err1.contains("environment.kind") || err1.contains("unit/integration test runner"),
        "expected rejection of ci/local cargo-test for live_runtime, got: {err1}"
    );

    // 2. Rejects Markdown-only artifact even when environment=cluster and tool=kubectl
    fake_live["environment"] = json!({
        "kind": "cluster",
        "identifier": "microk8s-server.internal.lgcorzo"
    });
    fake_live["method"] = json!({
        "tool": "kubectl-hatchet-e2e",
        "command_or_test": "kubectl get job,pod -n factory-sandboxes -o json"
    });
    fake_live["artifacts"] = json!([
        {
            "uri": "file://docs/audit/GITOPS_RUNTIME_VERIFICATION.md",
            "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "media_type": "text/markdown"
        }
    ]);
    let ev2: EvidenceRecord = serde_json::from_value(fake_live.clone()).unwrap();
    let err2 = validate_evidence_record(&ev2, None)
        .unwrap_err()
        .to_string();
    assert!(
        err2.contains("Markdown") || err2.contains("structured runtime telemetry"),
        "expected rejection of Markdown-only artifact for live_runtime, got: {err2}"
    );

    // 3. Passes when environment=cluster, tool=kubectl-hatchet-e2e, and artifact is structured .json
    fake_live["artifacts"] = json!([
        {
            "uri": "file://evidence/security/live-gvisor-isolation.json",
            "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "media_type": "application/json"
        }
    ]);
    let ev3: EvidenceRecord = serde_json::from_value(fake_live).unwrap();
    validate_evidence_record(&ev3, None)
        .expect("valid cluster live_runtime evidence with JSON telemetry artifact should pass");
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

#[test]
fn test_dg_rvp_014_all_ten_mandatory_evidence_gate_negative_cases() {
    let policy = VerificationPolicyConfig {
        expected_source_commit: Some("00b5f08ab7d2ea39b6862f13a78a932ca70d6154".to_string()),
        required_classes: vec![],
    };

    // Negative 1: LIVE_RUNTIME record backed only by `cargo test`
    let mut neg1 = valid_evidence_json();
    neg1["verification_class"] = json!("live_runtime");
    neg1["deployment_revision"] = json!({
        "repository": "lgcorzo/gitops_internal_lgcorzo",
        "branch": "main",
        "commit": "2716e548fa5bd7563ebfce232ee68a6a24bb11b2"
    });
    neg1["environment"] = json!({"kind": "cluster", "identifier": "microk8s"});
    neg1["method"] = json!({"tool": "cargo-test", "command_or_test": "cargo test --workspace"});
    neg1["artifacts"] = json!([{"uri": "file://evidence/security/live-gvisor-isolation.json"}]);
    let ev_neg1: EvidenceRecord = serde_json::from_value(neg1).unwrap();
    let err1 = validate_evidence_record(&ev_neg1, None)
        .unwrap_err()
        .to_string();
    assert!(
        err1.contains("unit/integration test runner"),
        "Neg 1 failed: {err1}"
    );

    // Negative 2: LIVE_RUNTIME record with environment `local`
    let mut neg2 = valid_evidence_json();
    neg2["verification_class"] = json!("live_runtime");
    neg2["deployment_revision"] = json!({
        "repository": "lgcorzo/gitops_internal_lgcorzo",
        "branch": "main",
        "commit": "2716e548fa5bd7563ebfce232ee68a6a24bb11b2"
    });
    neg2["environment"] = json!({"kind": "local", "identifier": "workstation"});
    neg2["method"] = json!({"tool": "kubectl", "command_or_test": "kubectl get pods"});
    neg2["artifacts"] = json!([{"uri": "file://evidence/security/live-gvisor-isolation.json"}]);
    let ev_neg2: EvidenceRecord = serde_json::from_value(neg2).unwrap();
    let err2 = validate_evidence_record(&ev_neg2, None)
        .unwrap_err()
        .to_string();
    assert!(
        err2.contains("environment.kind='local'"),
        "Neg 2 failed: {err2}"
    );

    // Negative 3: LIVE_RUNTIME record backed only by Markdown
    let mut neg3 = valid_evidence_json();
    neg3["verification_class"] = json!("live_runtime");
    neg3["deployment_revision"] = json!({
        "repository": "lgcorzo/gitops_internal_lgcorzo",
        "branch": "main",
        "commit": "2716e548fa5bd7563ebfce232ee68a6a24bb11b2"
    });
    neg3["environment"] = json!({"kind": "cluster", "identifier": "microk8s"});
    neg3["method"] = json!({"tool": "kubectl", "command_or_test": "kubectl get pods"});
    neg3["artifacts"] = json!([{"uri": "file://docs/audit/GITOPS_RUNTIME_VERIFICATION.md", "media_type": "text/markdown"}]);
    let ev_neg3: EvidenceRecord = serde_json::from_value(neg3).unwrap();
    let err3 = validate_evidence_record(&ev_neg3, None)
        .unwrap_err()
        .to_string();
    assert!(err3.contains("Markdown"), "Neg 3 failed: {err3}");

    // Negative 4: Evidence record with a modified/tampered artifact hash on disk
    let tmp_dir = std::env::temp_dir().join(format!("ev_tamper_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&tmp_dir).unwrap();
    std::fs::write(tmp_dir.join("artifact.json"), b"{\"tampered\": true}").unwrap();
    let mut neg4 = valid_evidence_json();
    neg4["artifacts"] = json!([{
        "uri": "file://artifact.json",
        "sha256": "0000000000000000000000000000000000000000000000000000000000000000",
        "media_type": "application/json"
    }]);
    let ev_neg4: EvidenceRecord = serde_json::from_value(neg4).unwrap();
    let err4 = validate_evidence_record(&ev_neg4, Some(&tmp_dir))
        .unwrap_err()
        .to_string();
    let _ = std::fs::remove_dir_all(&tmp_dir);
    assert!(err4.contains("SHA-256 mismatch"), "Neg 4 failed: {err4}");

    // Negative 5: Record referring to an old commit or stale timestamp (> max_age_days)
    let mut neg5_ev_json = valid_evidence_json();
    neg5_ev_json["collected_at"] = json!("2025-01-01T00:00:00Z"); // > 90 days before verified_at (2026-11-06)
    let ev_neg5: EvidenceRecord = serde_json::from_value(neg5_ev_json).unwrap();
    let st_neg5: RequirementState = serde_json::from_value(valid_verified_state_json()).unwrap();
    let err5 = validate_requirement_state(&st_neg5, &[ev_neg5], &policy)
        .unwrap_err()
        .to_string();
    assert!(err5.contains("Stale evidence"), "Neg 5 failed: {err5}");

    // Negative 6: Record with missing exit code claiming `passed`
    let mut neg6 = valid_evidence_json();
    neg6["outcome"] = json!("passed");
    neg6["exit_code"] = json!(null);
    let ev_neg6: EvidenceRecord = serde_json::from_value(neg6).unwrap();
    let err6 = validate_evidence_record(&ev_neg6, None)
        .unwrap_err()
        .to_string();
    assert!(err6.contains("missing exit_code"), "Neg 6 failed: {err6}");

    // Negative 7: Record with a fabricated or missing verifier
    let ev_valid: EvidenceRecord = serde_json::from_value(valid_evidence_json()).unwrap();
    let mut neg7_state = valid_verified_state_json();
    neg7_state["verified_by"] = json!(null);
    let st_neg7: RequirementState = serde_json::from_value(neg7_state).unwrap();
    let err7 = validate_requirement_state(&st_neg7, std::slice::from_ref(&ev_valid), &policy)
        .unwrap_err()
        .to_string();
    assert!(
        err7.contains("requires verified_by"),
        "Neg 7 failed: {err7}"
    );

    // Negative 8: A VERIFIED state with no qualifying evidence
    let mut neg8_state = valid_verified_state_json();
    neg8_state["evidence_refs"] = json!([]);
    let st_neg8: RequirementState = serde_json::from_value(neg8_state).unwrap();
    let err8 = validate_requirement_state(&st_neg8, std::slice::from_ref(&ev_valid), &policy)
        .unwrap_err()
        .to_string();
    assert!(err8.contains("evidence_refs"), "Neg 8 failed: {err8}");

    // Negative 9: Successful unit test incorrectly used to satisfy a Kubernetes runtime requirement (FR-008)
    let mut neg9_ev = valid_evidence_json();
    neg9_ev["requirement_id"] = json!("FR-008");
    neg9_ev["verification_class"] = json!("unit");
    let ev_neg9: EvidenceRecord = serde_json::from_value(neg9_ev).unwrap();
    let mut neg9_state = valid_verified_state_json();
    neg9_state["requirement_id"] = json!("FR-008");
    let st_neg9: RequirementState = serde_json::from_value(neg9_state).unwrap();
    let err9 = validate_requirement_state(&st_neg9, &[ev_neg9], &policy)
        .unwrap_err()
        .to_string();
    assert!(
        err9.contains("LiveRuntime") && err9.contains("cannot be satisfied by lower-level"),
        "Neg 9 failed: {err9}"
    );

    // Negative 10: Record whose command claims success (`passed`) but whose captured exit code is non-zero
    let mut neg10 = valid_evidence_json();
    neg10["outcome"] = json!("passed");
    neg10["exit_code"] = json!(101);
    let ev_neg10: EvidenceRecord = serde_json::from_value(neg10).unwrap();
    let err10 = validate_evidence_record(&ev_neg10, None)
        .unwrap_err()
        .to_string();
    assert!(
        err10.contains("non-zero exit_code=101"),
        "Neg 10 failed: {err10}"
    );
}
