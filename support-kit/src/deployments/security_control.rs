use rustls_acme::{axum::AxumAcceptor, caches::DirCache, AcmeConfig};
use tokio_stream::StreamExt;

#[derive(Debug, Default, bon::Builder)]
pub struct SecurityControl {
    config: super::SecurityConfig,
}

impl SecurityControl {
    pub fn new(deployment: &super::DeploymentConfig) -> Self {
        Self::builder().config(deployment.security.clone()).build()
    }

    pub async fn init(&self) -> Option<AxumAcceptor> {
        match &self.config {
            super::SecurityConfig::Acme {
                domains,
                emails,
                cache,
                production,
                ..
            } => {
                let mut state = AcmeConfig::new(domains)
                    .contact(emails.iter().map(|email| format!("mailto:{email}")))
                    .cache_option(cache.clone().map(DirCache::new))
                    .directory_lets_encrypt(*production)
                    .state();

                let acceptor = state.axum_acceptor(state.default_rustls_config());

                tokio::spawn(async move {
                    loop {
                        match state.next().await.unwrap() {
                            Ok(ok) => tracing::info!("tls certification event: {:?}", ok),
                            Err(err) => {
                                tracing::error!("tls certification error: {:?}", err)
                            }
                        }
                    }
                });

                Some(acceptor)
            }
            _ => None,
        }
    }
}
