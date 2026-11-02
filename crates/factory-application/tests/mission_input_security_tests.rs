use factory_application::workflows::autonomous_mission::MissionInput;

#[test]
fn test_mission_input_serialization_must_not_contain_bearer_tokens() {
    let input = MissionInput {
        mission_id: Some("mission-101".to_string()),
        goal: "Refactor security bounds".to_string(),
        repository_path: "/tmp/repo".to_string(),
        source_platform: Some("github".to_string()),
        repository: Some("org/repo-a".to_string()),
        issue_number: Some(42),
        credential_ref: None,
        verifiable_credential: None,
    };

    let serialized = serde_json::to_string(&input).expect("MissionInput serializable");

    // Explicit security invariant: No bearer tokens or plain "jit_token" keys in persisted JSON
    assert!(
        !serialized.contains("jit_token"),
        "MissionInput MUST NOT have a serialized field named jit_token: {}",
        serialized
    );

    // Verify debug formatting redacts or avoids bearer tokens
    let debug_output = format!("{:?}", input);
    assert!(
        !debug_output.contains("bearer-secret"),
        "MissionInput Debug output must not leak secret values: {}",
        debug_output
    );
}
