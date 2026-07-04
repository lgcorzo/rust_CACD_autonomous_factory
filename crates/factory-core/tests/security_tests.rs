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
            Ok(JitToken {
                token: String::new(),
            })
        }
    }

    let bounds = DummyBounds;
    let mut token = JitToken {
        token: String::from("some-secret"),
    };

    let token_ptr = token.token.as_ptr();
    let len = token.token.len();

    bounds.wipe_token_from_memory(&mut token);

    // Verify the String data is zeroed manually
    let wiped_data = unsafe { std::slice::from_raw_parts(token_ptr, len) };
    assert!(wiped_data.iter().all(|&byte| byte == 0));
}
