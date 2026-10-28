use chrono::{Duration, Utc};
use ed25519_dalek::SigningKey;
use factory_core::security::nhi::{AgentSubject, VerifiableCredential, VerificationOutcome};
use rand_core::OsRng;

#[test]
fn test_nhi_w3c_camel_case_serialization() {
    let subject = AgentSubject {
        id: "did:key:agent-007".to_string(),
        roles: vec!["autonomous-worker".to_string()],
        allowed_namespaces: vec!["default".to_string(), "remediation".to_string()],
    };

    let mut vc = VerifiableCredential::new(
        "urn:uuid:test-credential-001".to_string(),
        "did:key:issuer-darkgravity".to_string(),
        subject,
    );
    vc.expiration_date = Some(Utc::now() + Duration::hours(1));

    let json = serde_json::to_string(&vc).expect("Serialization failed");

    // Must be W3C camelCase
    assert!(
        json.contains("\"issuanceDate\":"),
        "Must contain issuanceDate in camelCase: {}",
        json
    );
    assert!(
        json.contains("\"expirationDate\":"),
        "Must contain expirationDate in camelCase: {}",
        json
    );
    assert!(
        json.contains("\"credentialSubject\":"),
        "Must contain credentialSubject in camelCase: {}",
        json
    );
    assert!(
        json.contains("\"allowedNamespaces\":"),
        "Must contain allowedNamespaces in camelCase: {}",
        json
    );

    // Must NOT contain snake_case
    assert!(
        !json.contains("\"issuance_date\":"),
        "Must not contain snake_case issuance_date: {}",
        json
    );
    assert!(
        !json.contains("\"expiration_date\":"),
        "Must not contain snake_case expiration_date: {}",
        json
    );
    assert!(
        !json.contains("\"credential_subject\":"),
        "Must not contain snake_case credential_subject: {}",
        json
    );
    assert!(
        !json.contains("\"allowed_namespaces\":"),
        "Must not contain snake_case allowed_namespaces: {}",
        json
    );
}

#[tokio::test]
async fn test_nhi_cryptographic_verification_outcomes() {
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();

    let subject = AgentSubject {
        id: "did:key:agent-1".to_string(),
        roles: vec!["worker".to_string()],
        allowed_namespaces: vec!["prod".to_string()],
    };

    let now = Utc::now();
    let mut vc = VerifiableCredential::new(
        "urn:uuid:vc-100".to_string(),
        "did:key:issuer".to_string(),
        subject,
    );
    vc.issuance_date = now - Duration::minutes(10);
    vc.expiration_date = Some(now + Duration::minutes(30));

    // Sign the credential
    vc.sign(&signing_key, "did:key:issuer#key-1")
        .expect("Signing failed");

    // 1. Valid at current time
    let outcome = vc
        .verify_at(now, &verifying_key)
        .expect("Verification failed");
    assert_eq!(outcome, VerificationOutcome::Valid);
    assert!(vc.verify(&verifying_key).expect("Default verify failed"));

    // 2. Expired verification
    let future_time = now + Duration::hours(2);
    let expired_outcome = vc
        .verify_at(future_time, &verifying_key)
        .expect("Verification failed");
    assert_eq!(expired_outcome, VerificationOutcome::Expired);

    // 3. Bad signature when tampering with payload
    let mut tampered_vc = vc.clone();
    tampered_vc
        .credential_subject
        .roles
        .push("admin".to_string());
    let bad_sig_outcome = tampered_vc
        .verify_at(now, &verifying_key)
        .expect("Verification failed");
    assert_eq!(bad_sig_outcome, VerificationOutcome::BadSignature);

    // 4. Bad signature when tampering with proof options (e.g., verificationMethod)
    let mut tampered_proof_vc = vc.clone();
    if let Some(ref mut proof) = tampered_proof_vc.proof {
        proof.verification_method = "did:key:attacker#key-2".to_string();
    }
    let bad_proof_outcome = tampered_proof_vc
        .verify_at(now, &verifying_key)
        .expect("Verification failed");
    assert_eq!(bad_proof_outcome, VerificationOutcome::BadSignature);

    // 5. Malformed without proof
    let mut no_proof_vc = vc.clone();
    no_proof_vc.proof = None;
    let malformed_outcome = no_proof_vc
        .verify_at(now, &verifying_key)
        .expect("Verification failed");
    assert_eq!(malformed_outcome, VerificationOutcome::Malformed);
}

#[tokio::test]
async fn test_nhi_batch_signing_atomic_behavior() {
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();

    let subject = AgentSubject {
        id: "did:key:batch-agent".to_string(),
        roles: vec!["worker".to_string()],
        allowed_namespaces: vec!["default".to_string()],
    };

    let mut batch = vec![
        VerifiableCredential::new("vc-b1".to_string(), "issuer".to_string(), subject.clone()),
        VerifiableCredential::new("vc-b2".to_string(), "issuer".to_string(), subject),
    ];

    VerifiableCredential::sign_batch_async(&mut batch, &signing_key, "key-1")
        .await
        .expect("Batch sign failed");

    assert!(batch[0].proof.is_some());
    assert!(batch[1].proof.is_some());
    assert_eq!(
        batch[0].verify_at(Utc::now(), &verifying_key).unwrap(),
        VerificationOutcome::Valid
    );
    assert_eq!(
        batch[1].verify_at(Utc::now(), &verifying_key).unwrap(),
        VerificationOutcome::Valid
    );
}
