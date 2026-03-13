use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{
    application::use_cases::GetUploadUrlUseCase,
    domain::value_objects::ApiResponse,
    infrastructure::{bootstrap::AppState, http::auth_header_extractor::AuthHeader},
};

#[derive(Deserialize)]
pub struct GetUploadUrlRequest {
    pub file_name: String,
}

pub async fn get_upload_url(
    State(state): State<Arc<AppState>>,
    _: AuthHeader,
    Path(project_name): Path<String>,
    Query(query): Query<GetUploadUrlRequest>,
) -> impl IntoResponse {
    let use_case = GetUploadUrlUseCase {
        file_storage: state.file_storage.clone(),
    };

    let result = use_case.execute(&project_name, &query.file_name).await;
    ApiResponse::default(result)
}

pub async fn get_download_url(
    State(state): State<Arc<AppState>>,
    _: AuthHeader,
    Path((project_name, file_name)): Path<(String, String)>,
) -> impl IntoResponse {
    let use_case = GetUploadUrlUseCase {
        file_storage: state.file_storage.clone(),
    };

    let result = use_case.execute(&project_name, &file_name).await;
    ApiResponse::default(result)
}
