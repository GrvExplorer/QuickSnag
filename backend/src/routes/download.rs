use axum::{
    response::{Response, IntoResponse},
    http::{StatusCode, header::HeaderMap},
    Json,
};
use reqwest::header;
use serde::Deserialize;

use crate::utils::youtube::download_youtube;

#[derive(Deserialize)]
pub struct VideoRequest {
    site: String,
    url: String,
}

pub async fn download_video(Json(payload): Json<VideoRequest>) -> impl IntoResponse {
    let response_body = match payload.site.as_str() {
        "youtube" => {
            format!("Processing youtube with URL: {}", payload.url)
        },
        "type2" => format!("Processing type2 with URL: {}", payload.url),
        _ => format!("Unknown type: {}", payload.site),
    };

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());

    Response::builder()
    .status(StatusCode::OK)
    .header(header::CONTENT_TYPE, "text/plain")
    .body(response_body)
    .unwrap()
}
