#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait KafkaClient {
    async fn publish(&self, topic: &str, key: &str, payload: &[u8]) -> anyhow::Result<()>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait S3Storage {
    async fn put_object(&self, bucket: &str, key: &str, data: Vec<u8>) -> anyhow::Result<()>;
    async fn get_object(&self, bucket: &str, key: &str) -> anyhow::Result<Vec<u8>>;
}

pub mod kafka;
pub mod mcp_client;
pub mod s3;

pub use kafka::SimpleMockKafkaClient;
#[cfg(any(test, feature = "test-utils"))]
pub use mcp_client::MockMcpClient;
pub use mcp_client::{McpClient, McpHttpClient};
pub use s3::AwsS3Storage;
