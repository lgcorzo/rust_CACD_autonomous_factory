use crate::S3Storage;
use async_trait::async_trait;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;

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
        let response = self
            .client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;

        let data = response.body.collect().await?.into_bytes();
        Ok(data.to_vec())
    }

    async fn list_buckets(&self) -> anyhow::Result<Vec<String>> {
        let res = self.client.list_buckets().send().await?;
        let names = res
            .buckets()
            .iter()
            .filter_map(|b| b.name().map(|n| n.to_string()))
            .collect();
        Ok(names)
    }

    async fn list_objects(
        &self,
        bucket: &str,
        prefix: Option<&str>,
    ) -> anyhow::Result<Vec<String>> {
        let mut req = self.client.list_objects_v2().bucket(bucket);
        if let Some(p) = prefix {
            req = req.prefix(p);
        }
        let res = req.send().await?;
        let keys = res
            .contents()
            .iter()
            .filter_map(|obj| obj.key().map(|k| k.to_string()))
            .collect();
        Ok(keys)
    }
}
