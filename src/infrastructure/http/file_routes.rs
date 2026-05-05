use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

use crate::{
    application::use_cases::{DeleteFileUseCase, GetDownloadUrlUseCase, GetUploadUrlUseCase},
    domain::value_objects::ApiResponse,
    infrastructure::{bootstrap::AppState, http::auth_header_extractor::AuthHeader},
};

pub async fn get_upload_url(
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

pub async fn get_download_url(
    State(state): State<Arc<AppState>>,
    _: AuthHeader,
    Path((project_name, file_name)): Path<(String, String)>,
) -> impl IntoResponse {
    let use_case = GetDownloadUrlUseCase {
        file_storage: state.file_storage.clone(),
    };

    let result = use_case.execute(&project_name, &file_name).await;
    ApiResponse::default(result)
}

pub async fn delete_file(
    State(state): State<Arc<AppState>>,
    _: AuthHeader,
    Path((project_name, file_name)): Path<(String, String)>,
) -> impl IntoResponse {
    let use_case = DeleteFileUseCase {
        file_storage: state.file_storage.clone(),
    };

    let result = use_case.execute(&project_name, &file_name).await;
    ApiResponse::default(result)
}
