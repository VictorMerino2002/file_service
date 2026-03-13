use std::sync::Arc;

use axum::{routing::get, Router};
use lambda_http::{run, Error};

use crate::infrastructure::{bootstrap::setup, http::file_routes};

mod application;
mod domain;
mod infrastructure;

async fn get_router() -> Router {
    let state = setup().await;
    Router::new()
        .route("/upload/{project_name}", get(file_routes::get_upload_url))
        .route(
            "/download/{project_name}/{file_name}",
            get(file_routes::get_download_url),
        )
        .with_state(Arc::new(state))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_ansi(false)
        .without_time()
        .init();
    let router = get_router().await;
    run(router).await
}
