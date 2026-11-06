use factory_mcp_server::sandbox::{GvisorK8sDriver, SandboxDriver};
use factory_mcp_server::tools::run_tests::RunTestsTool;
use factory_mcp_server::tools::Tool;
use serde_json::{json, Value};
use std::sync::Arc;

#[tokio::test]
async fn test_gvisor_k8s_driver_fails_closed_without_host_fallback_when_cluster_unavailable() {
    let _ = rustls::crypto::ring::default_provider().install_default();

    // Point KUBECONFIG to a guaranteed non-existent endpoint so K8s isolation is unavailable
    let sentinel_path = "/tmp/factory_gvisor_host_fallback_sentinel_should_never_exist";
    let _ = std::fs::remove_file(sentinel_path);

    std::env::set_var("KUBECONFIG", "/tmp/non_existent_kubeconfig_011_test.yaml");
    let driver = Arc::new(GvisorK8sDriver);
    let tool = RunTestsTool::new(driver);

    let res = tool
        .call(json!({
            "test_command": "python3 --version",
            "language": "python"
        }))
        .await
        .unwrap();

    std::env::remove_var("KUBECONFIG");

    assert!(
        res.is_error,
        "When gVisor K8s cluster is unavailable, RunTestsTool must fail closed as an error"
    );
    let text = match &res.content[0] {
        factory_mcp_server::protocol::McpContent::Text { text } => text,
        _ => panic!("Expected text content"),
    };
    let parsed: Value = serde_json::from_str(text).unwrap();
    assert_eq!(
        parsed["outcome"], "infrastructure_error",
        "Unavailable gVisor cluster must report infrastructure_error, never passed"
    );
    assert!(
        !std::path::Path::new(sentinel_path).exists(),
        "Sandbox failure must never fall back to host execution"
    );
}

/// Opt-in live cluster test: runs only when `RUN_LIVE_GVISOR_TEST=1` is set and an authorized
/// Kubernetes cluster with `RuntimeClass: gvisor` (`runsc`) and namespace `factory-sandboxes` is reachable.
#[tokio::test]
async fn test_gvisor_k8s_driver_live_sandbox_execution_opt_in() {
    if std::env::var("RUN_LIVE_GVISOR_TEST").unwrap_or_default() != "1" {
        eprintln!(
            "Skipping opt-in live gVisor cluster test (set RUN_LIVE_GVISOR_TEST=1 with cluster access to execute)."
        );
        return;
    }

    let _ = rustls::crypto::ring::default_provider().install_default();
    let driver = GvisorK8sDriver;
    let result = driver
        .execute("import platform; print('gvisor-ok:', platform.system())", "python")
        .await
        .expect("Authorized live gVisor execution must succeed on target cluster");

    assert!(result.is_success);
    assert_eq!(result.exit_code, Some(0));
    assert!(result.stdout.contains("gvisor-ok:"));
}
