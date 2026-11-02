use factory_core::security::SastScanResult;

#[test]
fn test_adversarial_evasion_separated_loop_and_break() {
    // Adversarial sample where an infinite loop is introduced, but "break" appears
    // in an unrelated comment or function earlier in the diff.
    // Basic regex: `!combined_added.contains("break")` fails to catch this!
    let adversarial_diff = r#"
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -10,0 +11,10 @@
+// Let us take a tea break.
+pub fn dangerous_agent_code() {
+    loop {
+        do_something();
+    }
+}
+"#;

    let scan = SastScanResult::inspect_diff(adversarial_diff);
    // AST-aware analyzer MUST detect this unbounded loop even if "break" is in a comment!
    assert!(
        !scan.is_safe,
        "AST analysis must detect infinite loop despite 'break' in comment. Findings: {:?}",
        scan.findings
    );
}

#[test]
fn test_adversarial_evasion_multiline_obfuscated_command() {
    // Evasive mutation: Command execution split across lines or via core dynamic invocation
    let adversarial_diff = r#"
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1,0 +1,5 @@
+fn sneaky() {
+    let mut cmd = std::
+        process::
+        Command::new("curl");
+}
+"#;

    let scan = SastScanResult::inspect_diff(adversarial_diff);
    assert!(
        !scan.is_safe,
        "AST analysis must catch multiline process execution. Findings: {:?}",
        scan.findings
    );
}
