use factory_mcp_server::sandbox::{SandboxDriver, SubprocessDriver};

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
async fn test_subprocess_driver_allowed_in_development() {
    let driver = SubprocessDriver::new().with_force_production(false);

    let result = driver.execute("print('dev test')", "python").await;

    assert!(result.is_ok(), "SubprocessDriver allowed in development");
    assert!(result.unwrap().stdout.contains("dev test"));
}
