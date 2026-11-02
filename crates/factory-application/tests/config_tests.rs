use factory_application::workflows::autonomous_mission::resolve_r2r_credentials;

#[test]
fn test_production_startup_fails_without_r2r_credentials() {
    // In production environment (FACTORY_ENV=production or production profile),
    // missing credentials must return an error / abort rather than falling back to admin:admin
    let res = resolve_r2r_credentials("production", None, None);
    assert!(
        res.is_err(),
        "Production must fail closed without explicit credentials: {:?}",
        res
    );
}

#[test]
fn test_development_startup_allows_defaults_or_env() {
    let res_explicit = resolve_r2r_credentials(
        "production",
        Some("prod@corp.local".to_string()),
        Some("super-secret-vault-pwd".to_string()),
    );
    assert!(res_explicit.is_ok());
    let (user, pwd) = res_explicit.unwrap();
    assert_eq!(user, "prod@corp.local");
    assert_eq!(pwd, "super-secret-vault-pwd");
}
