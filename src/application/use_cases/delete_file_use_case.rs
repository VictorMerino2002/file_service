use std::sync::Arc;

use crate::{application::ports::FileStoragePort, domain::value_objects::Error};

pub struct DeleteFileUseCase {
    pub file_storage: Arc<dyn FileStoragePort>,
}

impl DeleteFileUseCase {
    pub async fn execute(&self, project_name: &str, file_name: &str) -> Result<(), Error> {
        self.file_storage.delete_file(project_name, file_name).await
    }
}
