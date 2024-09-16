use axum::{extract::State, http::Uri, response::IntoResponse, routing::get, Router};
use rust_embed::RustEmbed;
use service_kit_support::{
    assets::{FileResponse, IndexAwareEmbeddedFile},
    storage::StorageFile,
};

#[derive(RustEmbed)]
#[folder = "dist/"]
struct Asset;

pub async fn router(_context: crate::WebContext) -> Router<crate::WebContext> {
    Router::new()
        .route("/*file", get(embed_handler))
        .route("/", get(index))
        .fallback(get(index))
}

#[tracing::instrument(level = "debug", skip(app_context), name = "index")]
async fn index(State(app_context): State<crate::WebContext>) -> impl IntoResponse {
    embed_handler(State(app_context), "/index.html".parse::<Uri>().unwrap()).await
}

#[tracing::instrument(
    level = "debug",
    skip(app_context),
    name = "looking for embedded assets"
)]
async fn embed_handler(
    State(app_context): State<crate::WebContext>,
    uri: Uri,
) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.starts_with("dist/") {
        tracing::debug!("removing dist prefix from path: {}", path);
        path = path.replace("dist/", "");
    }

    match StorageFile::get(&app_context.storage, &uri.to_string()).await {
        Ok(Some(file)) => {
            tracing::debug!("found storage file: {uri}");

            FileResponse::new(uri.to_string(), file.contents).into_response()
        }
        _ => {
            tracing::debug!("no storage file found: {path}");
            IndexAwareEmbeddedFile(Asset::get, path).into_response()
        }
    }
}
