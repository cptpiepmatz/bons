use axum::{
    body::Body,
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "dist/"]
#[allow_missing = true]
struct FrontendAssets;

pub async fn serve(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    if path.starts_with("assets/") {
        let Some(file) = FrontendAssets::get(path) else {
            return StatusCode::NOT_FOUND.into_response();
        };
        return file_response(file, "public, max-age=31536000, immutable");
    }

    if let Some(file) = FrontendAssets::get(path) {
        return file_response(file, "no-cache");
    }

    if path.rsplit('/').next().unwrap_or("").contains('.') {
        return StatusCode::NOT_FOUND.into_response();
    }

    let Some(file) = FrontendAssets::get("index.html") else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            "Frontend assets are missing. Run deno task build:frontend before building the server.",
        )
            .into_response();
    };

    file_response(file, "no-cache")
}

fn file_response(file: rust_embed::EmbeddedFile, cache_control: &str) -> Response {
    let content_type = file.metadata.mimetype();

    (
        [
            (header::CONTENT_TYPE, content_type),
            (header::CACHE_CONTROL, cache_control),
        ],
        Body::from(file.data.into_owned()),
    )
        .into_response()
}
