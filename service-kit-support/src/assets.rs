pub struct FileResponse {
    name: String,
    contents: Vec<u8>,
}

impl FileResponse {
    pub fn new(name: impl Into<String>, contents: impl Into<Vec<u8>>) -> Self {
        Self {
            name: name.into(),
            contents: contents.into(),
        }
    }
}

impl axum::response::IntoResponse for FileResponse {
    fn into_response(self) -> axum::response::Response {
        let mime = mime_guess::from_path(&self.name).first_or_octet_stream();
        let headers = [(axum::http::header::CONTENT_TYPE, mime.as_ref())];

        (headers, self.contents).into_response()
    }
}

pub struct IndexAwareEmbeddedFile<E, T>(pub E, pub T);

impl<E, T> axum::response::IntoResponse for IndexAwareEmbeddedFile<E, T>
where
    E: Fn(&str) -> Option<rust_embed::EmbeddedFile>,
    T: AsRef<str>,
{
    #[tracing::instrument(level = "debug", skip(self), name = "static file response")]
    fn into_response(self) -> axum::response::Response {
        let get = self.0;
        let path = self.1.as_ref();
        let index_path = format!("{path}/index.html");

        tracing::debug!("looking for embedded static file: {path}");

        let index_asset = get(index_path.as_str());
        let asset = get(path);

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
