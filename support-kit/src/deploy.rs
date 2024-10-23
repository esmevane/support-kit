use rustls_acme::axum::AxumAcceptor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Deployment {
    pub artifacts: Option<Artifacts>,
    pub hosts: Vec<Host>,
    #[serde(default)]
    pub security: Tls,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Artifacts {
    pub containers: Option<Containers>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Containers {
    pub registry: Option<Registry>,
    pub images: Vec<Image>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Registry {
    pub account: String,
    pub host: String,
    pub token: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Image {
    pub definition: String,
    pub name: String,
    pub label: String,
    pub namespace: String,
    pub repo: String,
}

impl Deployment {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn init_tls(&self) -> Option<AxumAcceptor> {
        tls::TlsControl::new(self).init().await
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Host {
    pub address: String,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub auth: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
struct Security {
    certificates: Option<Tls>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Tls {
    Acme {
        domains: Vec<String>,
        emails: Vec<String>,
        cache: Option<String>,
        production: bool,
    },
    #[serde(untagged)]
    #[default]
    Off,
    #[serde(untagged)]
    Unknown(serde_json::Value),
}

mod tls {
    use rustls_acme::{axum::AxumAcceptor, caches::DirCache, AcmeConfig};
    use tokio_stream::StreamExt;

    #[derive(Debug, Default, bon::Builder)]
    pub struct TlsControl {
        config: super::Tls,
    }

    impl TlsControl {
        pub fn new(deployment: &super::Deployment) -> Self {
            Self::builder().config(deployment.security.clone()).build()
        }

        pub async fn init(&self) -> Option<AxumAcceptor> {
            match &self.config {
                super::Tls::Acme {
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
}
