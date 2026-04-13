use async_trait::async_trait;

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait ZitiIdentity: Send + Sync {
    async fn get_token(&self) -> anyhow::Result<String>;
    fn service_name(&self) -> String;
}

pub struct OpenZitiIdentity {
    service: String,
    identity_file: String,
}

impl OpenZitiIdentity {
    pub fn new(service: &str, identity_file: &str) -> Self {
        Self {
            service: service.to_string(),
            identity_file: identity_file.to_string(),
        }
    }
}

#[async_trait]
impl ZitiIdentity for OpenZitiIdentity {
    async fn get_token(&self) -> anyhow::Result<String> {
        tracing::info!(
            "Retrieving mTLS token from OpenZiti for service {} using identity {}",
            self.service,
            self.identity_file
        );
        // In a real implementation, this would use the OpenZiti Rust SDK
        Ok("ziti-v1-token-placeholder".to_string())
    }

    fn service_name(&self) -> String {
        self.service.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_open_ziti_identity_new() {
        let service = "test-service";
        let identity_file = "test-identity.json";
        let identity = OpenZitiIdentity::new(service, identity_file);

        assert_eq!(identity.service, service);
        assert_eq!(identity.identity_file, identity_file);
    }

    #[tokio::test]
    async fn test_open_ziti_identity_trait_methods() {
        let service = "test-service";
        let identity_file = "test-identity.json";
        let identity = OpenZitiIdentity::new(service, identity_file);

        assert_eq!(identity.service_name(), service);

        let token = identity.get_token().await.unwrap();
        assert_eq!(token, "ziti-v1-token-placeholder");
    }
}
