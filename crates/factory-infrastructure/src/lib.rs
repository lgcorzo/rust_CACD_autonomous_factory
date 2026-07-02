#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait S3Storage {
    async fn put_object(&self, bucket: &str, key: &str, data: Vec<u8>) -> anyhow::Result<()>;
    async fn get_object(&self, bucket: &str, key: &str) -> anyhow::Result<Vec<u8>>;
}

pub mod aethalgard;
pub mod gitlab;
pub mod jira;
pub mod kafka;
pub mod mcp_client;
pub mod r2r;
pub mod s3;
pub mod sentry;
pub mod ziti;

#[cfg(any(test, feature = "test-utils"))]
pub use aethalgard::MockAethalgardClient;
pub use aethalgard::{AethalgardClient, HttpAethalgardClient};

#[cfg(any(test, feature = "test-utils"))]
pub use jira::MockJiraClient;
pub use jira::{HttpJiraClient, JiraClient};

pub use kafka::{KafkaClient, RdKafkaClient, SimpleMockKafkaClient};

#[cfg(any(test, feature = "test-utils"))]
pub use mcp_client::MockMcpClient;
pub use mcp_client::{McpClient, McpHttpClient, McpSseClient};

#[cfg(any(test, feature = "test-utils"))]
pub use r2r::MockR2rClient;
pub use r2r::{HttpR2rClient, R2rClient};

pub use s3::AwsS3Storage;
#[cfg(any(test, feature = "test-utils"))]
pub use ziti::MockZitiIdentity;
pub use ziti::{OpenZitiIdentity, ZitiIdentity};

#[cfg(any(test, feature = "test-utils"))]
pub use sentry::MockSentryClient;
pub use sentry::{CrashEvent, HttpSentryClient, SentryClient};

#[cfg(any(test, feature = "test-utils"))]
pub use gitlab::MockGitlabClient;
pub use gitlab::{GitlabClient, GitlabIssue, HttpGitlabClient};
