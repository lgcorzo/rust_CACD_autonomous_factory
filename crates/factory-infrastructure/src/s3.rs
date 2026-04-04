use async_trait::async_trait;
use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;
use crate::S3Storage;

pub struct AwsS3Storage {
    client: Client,
}

impl AwsS3Storage {
    pub async fn new() -> Self {
        let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let client = Client::new(&config);
        Self { client }
    }
}

#[async_trait]
impl S3Storage for AwsS3Storage {
    async fn put_object(&self, bucket: &str, key: &str, data: Vec<u8>) -> anyhow::Result<()> {
        self.client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(ByteStream::from(data))
            .send()
            .await?;
        Ok(())
    }

    async fn get_object(&self, bucket: &str, key: &str) -> anyhow::Result<Vec<u8>> {
        let response = self.client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;
            
        let data = response.body.collect().await?.into_bytes();
        Ok(data.to_vec())
    }
}
