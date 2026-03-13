use std::sync::Arc;

use aws_sdk_s3::Client;

use crate::{application::ports::FileStoragePort, infrastructure::adapters::S3FileStorage};

pub struct AppState {
    pub file_storage: Arc<dyn FileStoragePort>,
}

pub async fn setup() -> AppState {
    let aws_config = aws_config::load_from_env().await;
    let s3_client = Client::new(&aws_config);

    let file_storage = Arc::new(S3FileStorage::new(s3_client));

    AppState { file_storage }
}
