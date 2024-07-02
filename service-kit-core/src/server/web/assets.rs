use axum::{extract::State, http::Uri, response::IntoResponse, routing::get, Router};
use rust_embed::RustEmbed;

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

    match crate::storage::StorageFile::get(&app_context, &uri.to_string()).await {
        Ok(Some(file)) => {
            tracing::debug!("found storage file: {uri}");

            FileResponse::new(uri.to_string(), file.contents).into_response()
        }
        _ => {
            tracing::debug!("no storage file found: {path}");
            EmbeddedStaticFile(path).into_response()
        }
    }
}

struct FileResponse {
    name: String,
    contents: Vec<u8>,
}

impl FileResponse {
    fn new(name: impl Into<String>, contents: impl Into<Vec<u8>>) -> Self {
        Self {
            name: name.into(),
            contents: contents.into(),
        }
    }
}

impl IntoResponse for FileResponse {
    fn into_response(self) -> axum::response::Response {
        let mime = mime_guess::from_path(&self.name).first_or_octet_stream();
        let headers = [(axum::http::header::CONTENT_TYPE, mime.as_ref())];

        (headers, self.contents).into_response()
    }
}

#[derive(RustEmbed)]
#[folder = "dist/"]
struct Asset;

pub struct EmbeddedStaticFile<T>(pub T);

impl<T> IntoResponse for EmbeddedStaticFile<T>
where
    T: Into<String>,
{
    #[tracing::instrument(level = "debug", skip(self), name = "static file response")]
    fn into_response(self) -> axum::response::Response {
        let path = self.0.into();
        let index_path = format!("{path}/index.html");

        tracing::debug!("looking for embedded static file: {path}");

        let index_asset = Asset::get(index_path.as_str());
        let asset = Asset::get(path.as_str());

        match (index_asset, asset) {
            (Some(index), _) => {
                tracing::debug!("found index file: {index_path}");

                FileResponse::new(index_path, index.data).into_response()
            }
            (None, Some(content)) => {
                tracing::debug!("found static file: {}", path);

                FileResponse::new(path, content.data).into_response()
            }
            (None, None) => {
                tracing::debug!("no static file found for: {}", path);

                (axum::http::StatusCode::NOT_FOUND, "404 Not Found").into_response()
            }
        }
    }
}
