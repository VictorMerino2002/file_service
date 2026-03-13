use crate::{application::ports::FileStoragePort, domain::value_objects::Error};
use std::sync::Arc;

pub struct GetDownloadUrlUseCase {
    pub file_storage: Arc<dyn FileStoragePort>,
}

impl GetDownloadUrlUseCase {
    pub async fn execute(&self, project_name: &str, file_name: &str) -> Result<String, Error> {
        self.file_storage
            .get_download_url(project_name, file_name)
            .await
    }
}
