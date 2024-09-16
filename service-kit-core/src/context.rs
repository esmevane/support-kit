#[derive(Clone, Debug)]
pub struct WebContext {
    pub settings: service_kit_support::settings::Settings,
    pub storage: service_kit_support::storage::StorageCollection,
}

impl WebContext {
    pub async fn new(settings: service_kit_support::settings::Settings) -> crate::Result<Self> {
        Ok(Self {
            storage: service_kit_support::storage::StorageCollection::file_index(
                settings.storage_path(),
            )
            .await?,
            settings,
        })
    }

    pub async fn listener(&self) -> crate::Result<tokio::net::TcpListener> {
        Ok(self.settings.listener().await?)
    }

    pub fn settings(&self) -> &service_kit_support::settings::Settings {
        &self.settings
    }
}
