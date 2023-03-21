use std::path::PathBuf;

use axum::{
    extract::State,
    response::{Html, IntoResponse},
    Json,
};
use utoipa::openapi::OpenApi;

use crate::io::error::AppError;

pub async fn rapidoc() -> Result<impl IntoResponse, AppError> {
    let path: PathBuf = "./src/static/rapidoc.html"
        .parse()
        .expect("static rapicdoc.html file not exist");

    let file = tokio::fs::read_to_string(path).await.unwrap();

    Ok(Html(file))
}

pub async fn openapi_json(State(apidoc): State<OpenApi>) -> Result<impl IntoResponse, AppError> {
    Ok((axum::http::StatusCode::OK, Json(apidoc)))
}
