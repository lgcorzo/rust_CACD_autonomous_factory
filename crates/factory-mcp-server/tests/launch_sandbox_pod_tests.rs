use factory_mcp_server::sandbox::{
    ObservedRuntimeFacts, RequestedSandboxSpec, SandboxAttestation, VerifiedSandboxChecks,
};
use factory_mcp_server::tools::launch_sandbox_pod::build_sandbox_job_spec;

const IMG: &str = "docker.io/library/rust@sha256:9f841bbe9e7d8e37ceb96ed907265a3a0df7f44e3737d0b100e7907a679acb36";

fn fully_observed() -> SandboxAttestation {
    SandboxAttestation {
        cluster_id: Some("microk8s-server.internal.lgcorzo".into()),
        node_name: Some("server.internal.lgcorzo".into()),
        job_name: Some("sandbox-job-a68ab421".into()),
        pod_name: Some("sandbox-job-a68ab421-mmzwz".into()),
        pod_uid: Some("af7316c2-e003-4110-b9f3-1117992cf4e3".into()),
        container_id: Some("containerd://eeaacc79".into()),
        container_sha: Some(IMG.into()),
        runtime_class: "gvisor".into(),
        runtime_type: "unobserved_from_k8s_pod_api".into(),
        seccomp_profile: "RuntimeDefault".into(),
        is_root: false,
        capabilities_dropped: vec!["ALL".into()],
        exit_code: Some(0),
        command_hash: None,
        observed_at: Some(chrono::Utc::now()),
        requested_spec: RequestedSandboxSpec {
            runtime_class_name: Some("gvisor".into()),
            runtime_handler: Some("runsc".into()),
            seccomp_profile: "RuntimeDefault".into(),
            run_as_non_root: true,
            run_as_user: Some(65534),
            read_only_root_filesystem: true,
            capabilities_dropped: vec!["ALL".into()],
            image_digest: Some(IMG.into()),
            ..Default::default()
        },
        observed_runtime: ObservedRuntimeFacts {
            job_name: Some("sandbox-job-a68ab421".into()),
            pod_uid: Some("af7316c2-e003-4110-b9f3-1117992cf4e3".into()),
            pod_phase: Some("Succeeded".into()),
            container_id: Some("containerd://eeaacc79".into()),
            container_image_digest: Some(IMG.into()),
            exit_code: Some(0),
            observed_runtime_class_name: Some("gvisor".into()),
            observed_run_as_non_root: Some(true),
            observed_run_as_user: Some(65534),
            observed_read_only_root_filesystem: Some(true),
            observed_seccomp_profile: Some("RuntimeDefault".into()),
            observed_capabilities_dropped: vec!["ALL".into()],
            host_runtime_handler_observed: Some("runsc".into()),
            ..Default::default()
        },
        verified_checks: VerifiedSandboxChecks::default(),
    }
}

fn eval(a: &SandboxAttestation) -> VerifiedSandboxChecks {
    a.evaluate_verification(Some("sandbox-job-a68ab421"), 600)
}

fn assert_unverified(a: SandboxAttestation, needle: &str) {
    let v = eval(&a);
    assert_eq!(v.verification_status, "UNVERIFIED");
    assert!(
        v.failures.iter().any(|f| f.contains(needle)),
        "expected failure containing '{needle}', got {:?}",
        v.failures
    );
}

#[test]
fn test_attestation_fully_corroborated_is_verified() {
    let v = eval(&fully_observed());
    assert_eq!(v.verification_status, "VERIFIED", "{:?}", v.failures);
    assert!(v.effective_runtime_is_gvisor);
}

#[test]
fn test_sandbox_attestation_distinguishes_requested_spec_from_observed_runtime() {
    let mut a = fully_observed();
    a.observed_runtime.host_runtime_handler_observed = None;
    let json = serde_json::to_value(&a).unwrap();
    assert_eq!(json["requested_spec"]["runtime_handler"], "runsc");
    assert!(json["observed_runtime"]["host_runtime_handler_observed"].is_null());
    // Requesting gVisor must not be enough to verify it.
    assert_unverified(a, "Runtime observation unavailable");
}

#[test]
fn test_attestation_negative_cases_prevent_verified() {
    let mut a = fully_observed();
    a.observed_runtime.pod_uid = None;
    assert_unverified(a, "Missing observed pod_uid");
    let mut a = fully_observed();
    a.observed_runtime.container_id = None;
    assert_unverified(a, "Missing observed container_id");
    let mut a = fully_observed();
    a.observed_runtime.container_image_digest =
        Some("docker.io/library/rust@sha256:deadbeef".into());
    assert_unverified(a, "Unexpected image digest");
    let mut a = fully_observed();
    a.observed_runtime.observed_runtime_class_name = Some("runc".into());
    assert_unverified(a, "RuntimeClass mismatch");
    let mut a = fully_observed();
    a.observed_runtime.host_runtime_handler_observed = Some("runc".into());
    assert_unverified(a, "Runtime handler mismatch");
    let mut a = fully_observed();
    a.observed_runtime.exit_code = Some(1);
    a.observed_runtime.pod_phase = Some("Failed".into());
    assert_unverified(a, "Container termination failure");
    let mut a = fully_observed();
    a.observed_runtime.host_runtime_handler_observed = None;
    assert_unverified(a, "Runtime observation unavailable");
    let mut a = fully_observed();
    a.observed_at = Some(chrono::Utc::now() - chrono::Duration::hours(2));
    assert_unverified(a, "Stale attestation");
    let mut a = fully_observed();
    a.observed_runtime.job_name = Some("sandbox-job-other".into());
    a.job_name = None;
    assert_unverified(a, "Job correlation mismatch");
    let mut a = fully_observed();
    a.observed_runtime.observed_read_only_root_filesystem = Some(false);
    assert_unverified(a, "Read-only rootfs mismatch");
}

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

    // Must fail closed when untrusted workload attempts to bypass gVisor
    let non_gvisor_res =
        build_sandbox_job_spec(job_name, cmd, false, "factory-sandboxes", "python");
    assert!(
        non_gvisor_res.is_err(),
        "Untrusted code execution MUST reject disabling gVisor across all environments"
    );
    // Must fail closed if default namespace is used
    assert!(build_sandbox_job_spec(job_name, cmd, true, "default", "python").is_err());
    // Must fail closed on empty command
    assert!(build_sandbox_job_spec(job_name, "   ", true, "factory-sandboxes", "python").is_err());

    // Must use rust image for rust language
    let rust_spec =
        build_sandbox_job_spec(job_name, "cargo test", true, "factory-sandboxes", "rust").unwrap();
    let rust_image = rust_spec["spec"]["template"]["spec"]["containers"][0]["image"]
        .as_str()
        .unwrap();
    assert!(
        rust_image.contains("rust")
            && (rust_image.contains("1.85-slim") || rust_image.contains("sha256:")),
        "Expected rust immutable or versioned image, got: {}",
        rust_image
    );
}
