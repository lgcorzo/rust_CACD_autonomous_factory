use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentSubject {
    pub id: String,
    pub roles: Vec<String>,
    pub allowed_namespaces: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CryptographicProof {
    pub proof_type: String,
    pub created: DateTime<Utc>,
    pub verification_method: String,
    pub proof_purpose: String,
    pub jws: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifiableCredential {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub credential_type: Vec<String>,
    pub issuer: String,
    pub issuance_date: DateTime<Utc>,
    pub credential_subject: AgentSubject,
    pub proof: Option<CryptographicProof>,
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
            credential_subject,
            proof: None,
        }
    }

    /// Generates a JWS for the credential and attaches it to the `proof` field.
    pub fn sign(
        &mut self,
        signing_key: &ed25519_dalek::SigningKey,
        key_id: &str,
    ) -> crate::error::Result<()> {
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
        use ed25519_dalek::Signer;

        // We serialize the credential without the proof to canonicalize it.
        let mut clone_no_proof = self.clone();
        clone_no_proof.proof = None;
        let payload_json = serde_json::to_string(&clone_no_proof).map_err(|e| {
            crate::error::FactoryError::Security(format!("Serialization error: {}", e))
        })?;

        // Simple JWS formatting: Header.Payload.Signature
        let header = serde_json::json!({
            "alg": "EdDSA",
            "b64": false,
            "crit": ["b64"]
        });

        let header_json = serde_json::to_string(&header).map_err(|e| {
            crate::error::FactoryError::Security(format!("Serialization error: {}", e))
        })?;

        let encoded_header = URL_SAFE_NO_PAD.encode(header_json.as_bytes());
        let signing_input = format!("{}.{}", encoded_header, payload_json);

        let signature = signing_key.sign(signing_input.as_bytes());
        let encoded_signature = URL_SAFE_NO_PAD.encode(signature.to_bytes());

        let jws = format!("{}..{}", encoded_header, encoded_signature);

        self.proof = Some(CryptographicProof {
            proof_type: "Ed25519Signature2020".to_string(),
            created: Utc::now(),
            verification_method: key_id.to_string(),
            proof_purpose: "assertionMethod".to_string(),
            jws,
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

    /// Asynchronously batch-signs multiple credentials concurrently.
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

        for (i, handle) in handles.into_iter().enumerate() {
            let signed_vc = handle.await.map_err(|e| {
                crate::error::FactoryError::Security(format!("Batch join error: {}", e))
            })??;
            credentials[i] = signed_vc;
        }

        Ok(())
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
    }
}
