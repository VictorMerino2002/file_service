use std::time::Duration;

use async_trait::async_trait;
use aws_sdk_s3::{presigning::PresigningConfig, Client};

use crate::{application::ports::FileStoragePort, domain::value_objects::Error};

pub struct S3FileStorage {
    client: Client,
}

impl S3FileStorage {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl FileStoragePort for S3FileStorage {
    async fn get_upload_url(&self, project_name: &str, file_name: &str) -> Result<String, Error> {
        let config = PresigningConfig::expires_in(Duration::from_secs(300))
            .map_err(|_| Error::internal_server("Failed to create presigning config"))?;

        let presigned = self
            .client
            .put_object()
            .bucket(project_name)
            .key(file_name)
            .content_type("application/octet-stream")
            .presigned(config)
            .await
            .map_err(|_| Error::internal_server("Failed to generate presigned URL"))?;

        Ok(presigned.uri().to_string())
    }

    async fn get_download_url(&self, project_name: &str, file_name: &str) -> Result<String, Error> {
        let config = PresigningConfig::expires_in(Duration::from_secs(3600)).map_err(|e| {
            tracing::error!("Failed to create presigning config: {e}");
            Error::internal_server("Failed to create presigning config")
        })?;

        let presigned = self
            .client
            .get_object()
            .bucket(project_name)
            .key(file_name)
            .presigned(config)
            .await
            .map_err(|e| {
                tracing::error!("Failed to generate download URL: {e}");
                Error::internal_server("Failed to generate download URL")
            })?;

        Ok(presigned.uri().to_string())
    }
}
