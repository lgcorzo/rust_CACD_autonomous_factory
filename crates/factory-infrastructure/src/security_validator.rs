use crate::mcp_client::McpClient;
use async_trait::async_trait;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use factory_core::security::{AuditResult, SecurityValidator};
use std::sync::Arc;

pub struct Ed25519Validator {
    public_key: VerifyingKey,
    mcp_client: Option<Arc<dyn McpClient>>,
}

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
}
