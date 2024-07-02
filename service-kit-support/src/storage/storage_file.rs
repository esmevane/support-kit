use super::storage_collection;

#[derive(Debug, sqlx::FromRow)]
pub struct StorageFile {
    pub name: String,
    pub path: String,
    pub size: i64,
    pub contents: Vec<u8>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl StorageFile {
    #[tracing::instrument(level = "debug", skip(storage), name = "Getting storage file for path")]
    pub async fn get(
        storage: &storage_collection::StorageCollection,
        path: &str,
    ) -> crate::Result<Option<Self>> {
        Ok(storage.get(path.parse()?).await.ok())
    }
}
