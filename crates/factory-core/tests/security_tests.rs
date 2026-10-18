use factory_core::security::JitToken;

#[test]
fn test_manual_wipe_token() {
    use factory_core::error::Result;
    use factory_core::security::SecurityBounds;

    struct DummyBounds;
    #[async_trait::async_trait]
    impl SecurityBounds for DummyBounds {
        async fn validate_token(&self, _token: &JitToken) -> Result<bool> {
            Ok(true)
        }
        async fn issue_jit_token(&self, _aud: &str) -> Result<JitToken> {
            Ok(JitToken::new(""))
        }
    }

    let bounds = DummyBounds;
    let mut token = JitToken::new("some-secret");
    assert_eq!(token.expose_token(), "some-secret");

    bounds.wipe_token_from_memory(&mut token);
    assert!(token.expose_token().is_empty() || token.expose_token().chars().all(|c| c == '\0'));
}

#[tokio::test]
async fn test_default_security_validator_fails_closed() {
    use ed25519_dalek::VerifyingKey;
    use factory_core::security::{Ed25519SecurityValidator, SecurityValidator};

    let raw_key = [1u8; 32];
    let verifying_key = VerifyingKey::from_bytes(&raw_key).unwrap();
    let validator = Ed25519SecurityValidator {
        public_key: verifying_key,
    };

    let result = validator
        .audit_content("fn malicious() { eval(\"bad\"); }")
        .await;
    assert!(
        result.is_err(),
        "Default unconfigured auditor must fail closed with an error"
    );
}

#[test]
fn test_jit_token_debug_redacts_secret_and_no_unsafe_leak() {
    let secret = "vault-token-super-secret-12345";
    let token = factory_core::security::JitToken::new(secret);

    let debug_output = format!("{:?}", token);
    assert!(
        !debug_output.contains(secret),
        "Debug representation must redact secret"
    );
    assert!(debug_output.contains("[REDACTED]"));
    assert_eq!(token.expose_token(), secret);
}
