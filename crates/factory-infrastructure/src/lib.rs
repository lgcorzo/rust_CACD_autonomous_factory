#[cfg_attr(test, mockall::automock)]
pub trait ZitiIdentity: Send + Sync {
    fn get_token(&self) -> anyhow::Result<String>;
    fn service_name(&self) -> String;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait S3Storage {
    async fn put_object(&self, bucket: &str, key: &str, data: Vec<u8>) -> anyhow::Result<()>;
    async fn get_object(&self, bucket: &str, key: &str) -> anyhow::Result<Vec<u8>>;
}

#[cfg(test)]
use mockall::automock;

#[async_trait::async_trait]
#[cfg_attr(test, automock)]
pub trait JiraClient: Send + Sync {
    async fn search_issues(&self, query: &str) -> anyhow::Result<String>;
}

#[async_trait::async_trait]
#[cfg_attr(test, automock)]
pub trait R2rClient: Send + Sync {
    async fn search(&self, query: &str) -> anyhow::Result<String>;
}

pub mod jira;
pub mod kafka;
pub mod mcp_client;
pub mod r2r;
pub mod s3;
pub mod ziti;

pub use jira::HttpJiraClient;
pub use kafka::{KafkaClient, SimpleMockKafkaClient};
#[cfg(any(test, feature = "test-utils"))]
pub use mcp_client::MockMcpClient;
pub use mcp_client::{McpClient, McpHttpClient};
pub use r2r::HttpR2rClient;
pub use s3::AwsS3Storage;
pub use ziti::OpenZitiIdentity;
