use factory_mcp_server::sandbox::{SandboxDriver, SubprocessDriver};
use std::time::Duration;

#[tokio::test]
async fn test_subprocess_driver_fails_closed_in_production() {
    let driver = SubprocessDriver::new().with_force_production(true);

    let result = driver.execute("print('attack host')", "python").await;

    assert!(
        result.is_err(),
        "SubprocessDriver must fail closed in production"
    );
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("Host subprocess execution is forbidden in production"),
        "Expected security error message, got: {}",
        err_msg
    );
}

#[tokio::test]
async fn test_subprocess_driver_rejects_unsupported_runtime_and_stubbed_languages() {
    let driver = SubprocessDriver::new().with_force_production(false);

    for unsupported in ["cobol", "bash", "sh", "go", "typescript", "ts"] {
        let res = driver.execute("echo test", unsupported).await;
        assert!(
            res.is_err(),
            "Unsupported or unverified runtime '{}' must be rejected rather than stubbed/simulated",
            unsupported
        );
    }
}

#[tokio::test]
async fn test_subprocess_driver_enforces_timeout() {
    let driver = SubprocessDriver::new()
        .with_force_production(false)
        .with_timeout(Duration::from_millis(150));

    let res = driver.execute("import time; time.sleep(5)", "python").await;

    assert!(res.is_err(), "Timed-out execution must return an error");
    let msg = res.unwrap_err().to_string();
    assert!(
        msg.to_lowercase().contains("timed out"),
        "Expected timeout message, got: {}",
        msg
    );
}

#[tokio::test]
async fn test_subprocess_driver_reports_actual_nonzero_exit_result() {
    let driver = SubprocessDriver::new().with_force_production(false);

    let res = driver
        .execute(
            "import sys; sys.stderr.write('assertion failed\\n'); sys.exit(3)",
            "python",
        )
        .await
        .expect("Process execution should return ExecutionResult on non-zero exit");

    assert!(
        !res.is_success,
        "Non-zero exit code must set is_success=false"
    );
    assert_eq!(res.exit_code, Some(3));
    assert!(res.stderr.contains("assertion failed"));
}

#[tokio::test]
async fn test_subprocess_driver_allowed_in_development() {
    let driver = SubprocessDriver::new().with_force_production(false);

    let result = driver.execute("print('dev test')", "python").await;

    assert!(result.is_ok(), "SubprocessDriver allowed in development");
    let out = result.unwrap();
    assert!(out.is_success);
    assert_eq!(out.exit_code, Some(0));
    assert!(out.stdout.contains("dev test"));
}
