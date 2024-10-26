use rustls_acme::axum::AxumAcceptor;

use super::{DeploymentConfig, SecurityControl};

pub struct DeploymentControl;

impl DeploymentControl {
    pub async fn initialize(deployment_config: &DeploymentConfig) -> Option<AxumAcceptor> {
        SecurityControl::new(deployment_config).init().await
    }
}
