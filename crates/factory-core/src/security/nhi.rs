use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationOutcome {
    Valid,
    Expired,
    BadSignature,
    Malformed,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AgentSubject {
    pub id: String,
    pub roles: Vec<String>,
    pub allowed_namespaces: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CryptographicProof {
    #[serde(rename = "type")]
    pub proof_type: String,
    pub created: DateTime<Utc>,
    pub verification_method: String,
    pub proof_purpose: String,
    pub jws: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ProofOptions<'a> {
    #[serde(rename = "type")]
    proof_type: &'a str,
    created: DateTime<Utc>,
    verification_method: &'a str,
    proof_purpose: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    nonce: Option<&'a str>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VerifiableCredential {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub credential_type: Vec<String>,
    pub issuer: String,
    pub issuance_date: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<DateTime<Utc>>,
    pub credential_subject: AgentSubject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<CryptographicProof>,
}

fn compute_signing_input(
    encoded_header: &str,
    proof_options: &ProofOptions,
    doc_without_proof: &VerifiableCredential,
) -> crate::error::Result<String> {
    let proof_opts_canonical = serde_jcs::to_string(proof_options).map_err(|e| {
        crate::error::FactoryError::Security(format!(
            "RFC 8785 proof options canonicalization error: {}",
            e
        ))
    })?;
    let payload_canonical = serde_jcs::to_string(doc_without_proof).map_err(|e| {
        crate::error::FactoryError::Security(format!(
            "RFC 8785 payload canonicalization error: {}",
            e
        ))
    })?;
    Ok(format!(
        "{}.{}.{}",
        encoded_header, proof_opts_canonical, payload_canonical
    ))
}

impl VerifiableCredential {
    pub fn new(id: String, issuer: String, credential_subject: AgentSubject) -> Self {
        Self {
            context: vec![
                "https://www.w3.org/2018/credentials/v1".to_string(),
                "https://w3id.org/security/suites/ed25519-2020/v1".to_string(),
            ],
            id,
            credential_type: vec!["VerifiableCredential".to_string()],
            issuer,
            issuance_date: Utc::now(),
            expiration_date: None,
            credential_subject,
            proof: None,
        }
    }

    pub fn with_expiration(mut self, expiration_date: DateTime<Utc>) -> Self {
        self.expiration_date = Some(expiration_date);
        self
    }

    /// Generates a JWS for the credential and attaches it to the `proof` field.
    pub fn sign(
        &mut self,
        signing_key: &ed25519_dalek::SigningKey,
        key_id: &str,
    ) -> crate::error::Result<()> {
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
        use ed25519_dalek::Signer;

        let mut clone_no_proof = self.clone();
        clone_no_proof.proof = None;

        let created = Utc::now();
        let proof_type = "Ed25519Signature2020";
        let proof_purpose = "assertionMethod";

        let proof_opts = ProofOptions {
            proof_type,
            created,
            verification_method: key_id,
            proof_purpose,
            nonce: None,
        };

        // Standard JWS header for detached Ed25519 payloads
        let header = serde_json::json!({
            "alg": "EdDSA",
            "b64": false,
            "crit": ["b64"]
        });

        let header_json = serde_json::to_string(&header).map_err(|e| {
            crate::error::FactoryError::Security(format!("Serialization error: {}", e))
        })?;

        let encoded_header = URL_SAFE_NO_PAD.encode(header_json.as_bytes());
        let signing_input = compute_signing_input(&encoded_header, &proof_opts, &clone_no_proof)?;

        let signature = signing_key.sign(signing_input.as_bytes());
        let encoded_signature = URL_SAFE_NO_PAD.encode(signature.to_bytes());

        let jws = format!("{}..{}", encoded_header, encoded_signature);

        self.proof = Some(CryptographicProof {
            proof_type: proof_type.to_string(),
            created,
            verification_method: key_id.to_string(),
            proof_purpose: proof_purpose.to_string(),
            jws,
            nonce: None,
        });

        Ok(())
    }

    /// Asynchronously signs credentials without blocking the Tokio task executor.
    pub async fn sign_async(
        &mut self,
        signing_key: ed25519_dalek::SigningKey,
        key_id: String,
    ) -> crate::error::Result<()> {
        let mut vc = self.clone();
        let signed_vc =
            tokio::task::spawn_blocking(move || -> crate::error::Result<VerifiableCredential> {
                vc.sign(&signing_key, &key_id)?;
                Ok(vc)
            })
            .await
            .map_err(|e| {
                crate::error::FactoryError::Security(format!("Task join error: {}", e))
            })??;

        *self = signed_vc;
        Ok(())
    }

    /// Asynchronously batch-signs multiple credentials concurrently and atomically.
    pub async fn sign_batch_async(
        credentials: &mut [VerifiableCredential],
        signing_key: &ed25519_dalek::SigningKey,
        key_id: &str,
    ) -> crate::error::Result<()> {
        let key_id_owned = key_id.to_string();
        let mut handles = Vec::with_capacity(credentials.len());

        for vc in credentials.iter() {
            let mut vc_clone = vc.clone();
            let key_clone = signing_key.clone();
            let key_id_clone = key_id_owned.clone();
            handles.push(tokio::task::spawn_blocking(
                move || -> crate::error::Result<VerifiableCredential> {
                    vc_clone.sign(&key_clone, &key_id_clone)?;
                    Ok(vc_clone)
                },
            ));
        }

        let mut signed_results = Vec::with_capacity(credentials.len());
        for handle in handles {
            let signed_vc = handle.await.map_err(|e| {
                crate::error::FactoryError::Security(format!("Batch join error: {}", e))
            })??;
            signed_results.push(signed_vc);
        }

        // Atomic assignment only after all futures resolve successfully
        for (dest, src) in credentials.iter_mut().zip(signed_results) {
            *dest = src;
        }

        Ok(())
    }

    /// Verifies the cryptographic proof and temporal validity against the given timestamp and key.
    pub fn verify_at(
        &self,
        now: DateTime<Utc>,
        verifying_key: &ed25519_dalek::VerifyingKey,
    ) -> crate::error::Result<VerificationOutcome> {
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
        use ed25519_dalek::{Signature, Verifier};

        let proof = match &self.proof {
            Some(p) => p,
            None => return Ok(VerificationOutcome::Malformed),
        };

        if let Some(exp) = self.expiration_date {
            if now > exp {
                return Ok(VerificationOutcome::Expired);
            }
        }

        let parts: Vec<&str> = proof.jws.split("..").collect();
        if parts.len() != 2 {
            return Ok(VerificationOutcome::Malformed);
        }
        let encoded_header = parts[0];
        let encoded_signature = parts[1];

        let sig_bytes = match URL_SAFE_NO_PAD.decode(encoded_signature) {
            Ok(b) => b,
            Err(_) => return Ok(VerificationOutcome::Malformed),
        };

        if sig_bytes.len() != 64 {
            return Ok(VerificationOutcome::Malformed);
        }
        let mut sig_array = [0u8; 64];
        sig_array.copy_from_slice(&sig_bytes);
        let signature = Signature::from_bytes(&sig_array);

        let mut clone_no_proof = self.clone();
        clone_no_proof.proof = None;

        let proof_opts = ProofOptions {
            proof_type: &proof.proof_type,
            created: proof.created,
            verification_method: &proof.verification_method,
            proof_purpose: &proof.proof_purpose,
            nonce: proof.nonce.as_deref(),
        };

        let signing_input =
            match compute_signing_input(encoded_header, &proof_opts, &clone_no_proof) {
                Ok(inp) => inp,
                Err(_) => return Ok(VerificationOutcome::Malformed),
            };

        if verifying_key
            .verify(signing_input.as_bytes(), &signature)
            .is_ok()
        {
            Ok(VerificationOutcome::Valid)
        } else {
            Ok(VerificationOutcome::BadSignature)
        }
    }

    /// Verifies the cryptographic proof of a Verifiable Credential against an Ed25519 verifying key at the current time.
    pub fn verify(
        &self,
        verifying_key: &ed25519_dalek::VerifyingKey,
    ) -> crate::error::Result<bool> {
        Ok(self.verify_at(Utc::now(), verifying_key)? == VerificationOutcome::Valid)
    }

    /// Asynchronously verifies a batch of verifiable credentials concurrently.
    pub async fn verify_batch_async(
        credentials: &[VerifiableCredential],
        verifying_key: &ed25519_dalek::VerifyingKey,
    ) -> crate::error::Result<bool> {
        let mut handles = Vec::with_capacity(credentials.len());

        for vc in credentials.iter() {
            let vc_clone = vc.clone();
            let key_clone = *verifying_key;
            handles.push(tokio::task::spawn_blocking(
                move || -> crate::error::Result<bool> { vc_clone.verify(&key_clone) },
            ));
        }

        for handle in handles {
            let is_valid = handle.await.map_err(|e| {
                crate::error::FactoryError::Security(format!("Batch verify join error: {}", e))
            })??;
            if !is_valid {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    #[tokio::test]
    async fn test_vc_async_signing_and_batch() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);

        let subject = AgentSubject {
            id: "agent-1".to_string(),
            roles: vec!["worker".to_string()],
            allowed_namespaces: vec!["default".to_string()],
        };

        let mut vc =
            VerifiableCredential::new("vc-1".to_string(), "issuer-1".to_string(), subject.clone());

        vc.sign_async(signing_key.clone(), "key-1".to_string())
            .await
            .unwrap();
        assert!(vc.proof.is_some());
        assert_eq!(vc.proof.as_ref().unwrap().verification_method, "key-1");

        let mut batch = vec![
            VerifiableCredential::new("vc-2".to_string(), "issuer-1".to_string(), subject.clone()),
            VerifiableCredential::new("vc-3".to_string(), "issuer-1".to_string(), subject),
        ];

        VerifiableCredential::sign_batch_async(&mut batch, &signing_key, "key-1")
            .await
            .unwrap();

        assert!(batch[0].proof.is_some());
        assert!(batch[1].proof.is_some());

        let verifying_key = signing_key.verifying_key();
        assert!(vc.verify(&verifying_key).unwrap());
        assert!(
            VerifiableCredential::verify_batch_async(&batch, &verifying_key)
                .await
                .unwrap()
        );
    }
}
