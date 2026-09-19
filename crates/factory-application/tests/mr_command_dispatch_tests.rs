use factory_application::workflows::comment_control::CommentControlService;

#[test]
fn test_format_failure_report_structure() {
    let error_msg = "Compiling error: undefined symbol FooBar in src/lib.rs:42";
    let remediation = [
        "Check that FooBar struct is exported from factory_core",
        "Add FooBar to imports in src/lib.rs",
    ];

    let report = CommentControlService::format_failure_report(error_msg, &remediation);

    assert!(report.contains("❌ **Dark Gravity Action Failed**"));
    assert!(report.contains("undefined symbol FooBar"));
    assert!(report.contains("1. Check that FooBar struct is exported"));
    assert!(report.contains("2. Add FooBar to imports"));
}
