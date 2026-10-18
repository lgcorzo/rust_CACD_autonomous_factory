use factory_core::security::SastScanResult;

#[test]
fn test_sast_diff_scanner_added_lines_and_word_boundaries() {
    // 1. False positive resistance: "retrieval(" and "filesystem(" should NOT flag
    let safe_diff = "@@ -1,3 +1,3 @@\n+ let data = retrieval(param);\n+ let fs = filesystem(path);";
    let res = SastScanResult::inspect_diff(safe_diff);
    assert!(
        res.is_safe,
        "Safe function names (retrieval/filesystem) must not trigger false positive RCE findings"
    );
    assert_eq!(res.findings.len(), 0);

    // 2. Secret removal should NOT flag
    let secret_removal_diff =
        "@@ -10,3 +10,2 @@\n- password = \"supersecret123\"\n+ password = get_env_var()";
    let res2 = SastScanResult::inspect_diff(secret_removal_diff);
    assert!(
        !res2.critical_vulnerabilities_detected,
        "Removing a secret must not be flagged as a critical leak"
    );

    // 3. True positive: actual command execution
    let rce_diff =
        "@@ -1,2 +1,3 @@\n+ let _ = std::process::Command::new(\"bash\").arg(\"-c\").spawn();";
    let res3 = SastScanResult::inspect_diff(rce_diff);
    assert!(!res3.is_safe);
    assert!(res3.critical_vulnerabilities_detected);
}

#[test]
fn test_sast_diff_scanner_broadened_secrets_and_network_calls() {
    // Hardcoded API key or private key with colon or assignment
    let secret_diff = "@@ -1,2 +1,3 @@\n+ api_key: 'sk-proj-12345abcdef'";
    let res = SastScanResult::inspect_diff(secret_diff);
    assert!(res.critical_vulnerabilities_detected);

    // Outbound network call in diff
    let net_diff = "@@ -1,2 +1,3 @@\n+ let resp = reqwest::get(\"https://malicious.site\").await;";
    let res2 = SastScanResult::inspect_diff(net_diff);
    assert!(res2.critical_vulnerabilities_detected);
}
