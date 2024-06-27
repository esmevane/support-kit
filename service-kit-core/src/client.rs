use crate::settings::NetworkSettings;
use service_kit_proto::prelude::*;

/// Make a network request with a `NetworkSettings` configuration against the /health endpoint.
///
pub async fn health(config: NetworkSettings) -> crate::Result<HealthResponse> {
    let uri = format!("http://{}/health", config.address());
    let response = reqwest::get(&uri).await?;

    Ok(response.json::<HealthResponse>().await?)
}

pub struct WebClient;

impl WebClient {
    pub fn new() -> Self {
        Self
    }

    pub async fn health(&self) -> crate::Result<HealthResponse> {
        health(NetworkSettings {
            host: "localhost".to_string(),
            port: 8080,
        })
        .await
    }
}
