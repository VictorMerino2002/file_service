use async_trait::async_trait;

use crate::domain::value_objects::Error;

#[async_trait]
pub trait FileStoragePort: Send + Sync {
    async fn get_upload_url(&self, project_name: &str, file_name: &str) -> Result<String, Error>;
    async fn get_download_url(&self, project_name: &str, file_name: &str) -> Result<String, Error>;
}
