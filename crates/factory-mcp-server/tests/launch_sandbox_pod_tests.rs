use factory_mcp_server::tools::launch_sandbox_pod::build_sandbox_job_spec;

#[test]
fn test_launch_sandbox_pod_requires_gvisor_runtime_class_in_production() {
    let job_name = "test-sandbox-01";
    let cmd = "python3 -c 'print(1)'";

    // In production, runtimeClassName must be "gvisor"
    let job_spec = build_sandbox_job_spec(job_name, cmd, true, "factory-sandboxes").unwrap();

    let template_spec = job_spec
        .get("spec")
        .and_then(|s| s.get("template"))
        .and_then(|t| t.get("spec"))
        .expect("Pod template spec must exist");

    assert_eq!(
        template_spec.get("runtimeClassName").and_then(|r| r.as_str()),
        Some("gvisor"),
        "Production sandbox must mandate runtimeClassName: gvisor"
    );

    assert_eq!(
        template_spec.get("automountServiceAccountToken").and_then(|a| a.as_bool()),
        Some(false),
        "Sandbox must disable automountServiceAccountToken"
    );

    // In production, if gVisor is disabled, it MUST fail closed
    let err_result = build_sandbox_job_spec(job_name, cmd, false, "factory-sandboxes");
    assert!(
        err_result.is_err(),
        "Must fail closed if gVisor is not enabled in production"
    );
}

