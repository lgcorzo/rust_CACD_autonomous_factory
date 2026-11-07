use factory_mcp_server::tools::launch_sandbox_pod::build_sandbox_job_spec;

#[test]
fn test_launch_sandbox_pod_requires_gvisor_and_full_hardened_policy() {
    let job_name = "test-sandbox-01";
    let cmd = "python3 -c 'print(1)'";

    let job_spec =
        build_sandbox_job_spec(job_name, cmd, true, "factory-sandboxes", "python").unwrap();

    let spec = job_spec.get("spec").expect("Job spec must exist");
    assert_eq!(spec["ttlSecondsAfterFinished"], 300);
    assert_eq!(spec["activeDeadlineSeconds"], 60);
    assert_eq!(spec["backoffLimit"], 0);

    let template_spec = spec
        .get("template")
        .and_then(|t| t.get("spec"))
        .expect("Pod template spec must exist");

    assert_eq!(template_spec["runtimeClassName"], "gvisor");
    assert_eq!(template_spec["automountServiceAccountToken"], false);
    assert_eq!(template_spec["securityContext"]["runAsNonRoot"], true);
    assert_eq!(template_spec["securityContext"]["runAsUser"], 65534);
    assert_eq!(template_spec["securityContext"]["runAsGroup"], 65534);
    assert_eq!(
        template_spec["securityContext"]["seccompProfile"]["type"],
        "RuntimeDefault"
    );

    let container = &template_spec["containers"][0];
    assert_eq!(
        container["securityContext"]["allowPrivilegeEscalation"],
        false
    );
    assert_eq!(container["securityContext"]["readOnlyRootFilesystem"], true);
    assert_eq!(
        container["securityContext"]["capabilities"]["drop"][0],
        "ALL"
    );
    assert_eq!(container["resources"]["limits"]["memory"], "512Mi");
    assert_eq!(container["resources"]["limits"]["cpu"], "500m");
    assert_eq!(
        container["resources"]["limits"]["ephemeral-storage"],
        "200Mi"
    );

    // Must omit runtimeClassName if gVisor is disabled in non-production
    let non_gvisor_spec =
        build_sandbox_job_spec(job_name, cmd, false, "factory-sandboxes", "python").unwrap();
    assert!(non_gvisor_spec["spec"]["template"]["spec"]
        .get("runtimeClassName")
        .is_none());
    // Must fail closed if default namespace is used
    assert!(build_sandbox_job_spec(job_name, cmd, true, "default", "python").is_err());
    // Must fail closed on empty command
    assert!(build_sandbox_job_spec(job_name, "   ", true, "factory-sandboxes", "python").is_err());

    // Must use rust image for rust language
    let rust_spec =
        build_sandbox_job_spec(job_name, "cargo test", true, "factory-sandboxes", "rust").unwrap();
    assert_eq!(
        rust_spec["spec"]["template"]["spec"]["containers"][0]["image"],
        "rust:1.85-slim"
    );
}
