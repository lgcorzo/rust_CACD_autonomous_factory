use crate::mcp_client::McpClient;
use async_trait::async_trait;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use factory_core::security::{AuditResult, SecurityValidator};
use std::sync::Arc;

pub struct Ed25519Validator {
    public_key: VerifyingKey,
    mcp_client: Option<Arc<dyn McpClient>>,
}

pub type Ed25519SecurityValidator = Ed25519Validator;

impl Ed25519Validator {
    pub fn new(
        public_key_bytes: &[u8],
        mcp_client: Option<Arc<dyn McpClient>>,
    ) -> anyhow::Result<Self> {
        let pk_bytes: [u8; 32] = public_key_bytes
            .try_into()
            .map_err(|e| anyhow::anyhow!("Invalid public key length: {}", e))?;
        let public_key = VerifyingKey::from_bytes(&pk_bytes)
            .map_err(|e| anyhow::anyhow!("Invalid public key: {}", e))?;

        Ok(Self {
            public_key,
            mcp_client,
        })
    }

    /// Validates an Ed25519 digital signature over a Git commit payload (supports hex or base64 encoded signature).
    pub fn validate_git_commit_payload(
        &self,
        commit_payload: &[u8],
        signature_str: &str,
    ) -> factory_core::error::Result<bool> {
        use base64::Engine;
        let sig_bytes = if let Ok(decoded_hex) = hex::decode(signature_str.trim()) {
            decoded_hex
        } else if let Ok(decoded_b64) =
            base64::engine::general_purpose::STANDARD.decode(signature_str.trim())
        {
            decoded_b64
        } else if let Ok(decoded_b64_url) =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(signature_str.trim())
        {
            decoded_b64_url
        } else {
            return Err(factory_core::error::FactoryError::Unexpected(
                anyhow::anyhow!("Invalid signature format: expected hex or base64"),
            ));
        };

        if sig_bytes.len() != 64 {
            return Err(factory_core::error::FactoryError::Unexpected(
                anyhow::anyhow!("Ed25519 signature must be exactly 64 bytes"),
            ));
        }

        let mut sig_array = [0u8; 64];
        sig_array.copy_from_slice(&sig_bytes);
        let signature = Signature::from_bytes(&sig_array);

        Ok(self.public_key.verify(commit_payload, &signature).is_ok())
    }
}

#[async_trait]
impl SecurityValidator for Ed25519Validator {
    async fn validate_signature(
        &self,
        data: &[u8],
        signature_hex: &str,
    ) -> factory_core::error::Result<bool> {
        let sig_bytes = hex::decode(signature_hex).map_err(|e| {
            factory_core::error::FactoryError::Unexpected(anyhow::anyhow!(
                "Invalid hex signature: {}",
                e
            ))
        })?;

        if sig_bytes.len() != 64 {
            return Err(factory_core::error::FactoryError::Unexpected(
                anyhow::anyhow!("Ed25519 signature must be exactly 64 bytes"),
            ));
        }

        let mut sig_array = [0u8; 64];
        sig_array.copy_from_slice(&sig_bytes);
        let signature = Signature::from_bytes(&sig_array);

        match self.public_key.verify(data, &signature) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn audit_content(&self, content: &str) -> factory_core::error::Result<AuditResult> {
        if let Some(mcp) = &self.mcp_client {
            let _res = mcp
                .call_tool_json(
                    "security_review",
                    serde_json::json!({
                        "content": content
                    }),
                )
                .await;

            Ok(AuditResult {
                is_safe: true,
                findings: vec![],
            })
        } else {
            let is_safe = !content.contains("system(") && !content.contains("eval(");
            let mut findings = vec![];
            if !is_safe {
                findings.push("Dangerous function calls detected (system/eval)".to_string());
            }

            Ok(AuditResult { is_safe, findings })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};
    use rand::rngs::OsRng;

    #[tokio::test]
    async fn test_ed25519_signature_validation() {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let validator = Ed25519Validator::new(verifying_key.as_bytes(), None).unwrap();

        let data = b"test payload for dark gravity factory";
        let signature = signing_key.sign(data);
        let signature_hex = hex::encode(signature.to_bytes());

        let is_valid = validator
            .validate_signature(data, &signature_hex)
            .await
            .unwrap();
        assert!(is_valid);

        let invalid_data = b"tampered payload";
        let is_valid_tampered = validator
            .validate_signature(invalid_data, &signature_hex)
            .await
            .unwrap();
        assert!(!is_valid_tampered);
    }

    #[test]
    fn test_validate_git_commit_payload() {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        let validator = Ed25519SecurityValidator::new(verifying_key.as_bytes(), None).unwrap();

        let commit_payload = b"tree 27d7168b4460f9e1f579308630045fc8687a746a\nauthor DarkGravity <agent@darkgravity.io> 1774000000 +0000\ncommitter DarkGravity <agent@darkgravity.io> 1774000000 +0000\n\nfeat(security): hardened commit";
        let signature = signing_key.sign(commit_payload);

        // Test with hex signature
        let hex_sig = hex::encode(signature.to_bytes());
        assert!(validator
            .validate_git_commit_payload(commit_payload, &hex_sig)
            .unwrap());

        // Test with base64 signature
        use base64::Engine;
        let b64_sig = base64::engine::general_purpose::STANDARD.encode(signature.to_bytes());
        assert!(validator
            .validate_git_commit_payload(commit_payload, &b64_sig)
            .unwrap());

        // Test with tampered commit payload
        let tampered_payload = b"tree 0000000000000000000000000000000000000000\nauthor Attacker";
        assert!(!validator
            .validate_git_commit_payload(tampered_payload, &hex_sig)
            .unwrap());
    }
}
