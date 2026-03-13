use std::sync::Arc;

use crate::{application::ports::FileStoragePort, domain::value_objects::Error};

pub struct GetUploadUrlUseCase {
    pub file_storage: Arc<dyn FileStoragePort>,
}

impl GetUploadUrlUseCase {
    pub async fn execute(&self, project_name: &str, file_name: &str) -> Result<String, Error> {
        self.file_storage
            .get_upload_url(project_name, file_name)
            .await
    }
}
