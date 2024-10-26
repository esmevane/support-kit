use std::path::PathBuf;

use crate::{shell, Configuration, Registry, ShellCommand, SupportControl};

use super::{HostDeploymentContext, ImageDeploymentContext};

#[derive(Debug, Clone, bon::Builder)]
pub struct DeploymentContext {
    pub config: Configuration,
    #[builder(default, into)]
    pub images: Vec<ImageDeploymentContext>,
    #[builder(default, into)]
    pub hosts: Vec<HostDeploymentContext>,
    pub registry: Registry,
}

impl DeploymentContext {
    pub fn from_controller(controller: &SupportControl) -> Self {
        let config = controller.config.clone();
        let host_defs = config
            .deployment
            .clone()
            .map(|deployment| deployment.hosts)
            .unwrap_or_default();

        let (image_defs, registry) = config
            .deployment
            .clone()
            .and_then(|deployment| deployment.artifacts)
            .and_then(|artifacts| artifacts.containers)
            .map(|containers| (containers.images, containers.registry.unwrap_or_default()))
            .unwrap_or_default();

        let mut images = vec![];
        let mut hosts = vec![];

        for host in host_defs {
            hosts.push(
                HostDeploymentContext::builder()
                    .config(config.clone())
                    .host(host)
                    .registry(registry.clone())
                    .build(),
            );
        }

        for image in image_defs {
            images.push(
                ImageDeploymentContext::builder()
                    .config(config.clone())
                    .image(image)
                    .registry(registry.clone())
                    .build(),
            );
        }

        Self::builder()
            .config(config)
            .images(images)
            .hosts(hosts)
            .registry(registry)
            .build()
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn emit_config(&self) -> crate::Result<PathBuf> {
        let path =
            std::env::temp_dir().join(format!("{name}.container.json", name = self.config.name()));

        let contents = serde_json::to_string(&self.config)?;

        tracing::debug!(path = ?path, contents = ?contents,"writing container config file");

        std::fs::write(&path, contents).expect("Unable to write file");

        Ok(path)
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn setup_cert_volume(&self) -> crate::Result<ShellCommand> {
        shell(format!("docker volume create certs"))
    }

    /// Get the shell script from the [docker-install repo][docker-install].
    ///
    /// [docker-install]: https://github.com/docker/docker-install
    #[tracing::instrument(skip(self), level = "trace")]
    pub fn get_install_script(&self) -> crate::Result<ShellCommand> {
        shell(format!(
            "curl -fsSL https://get.docker.com -o get-docker.sh"
        ))
    }

    /// Install Docker on the host machine.
    #[tracing::instrument(skip(self), level = "trace")]
    pub fn install_docker(&self) -> crate::Result<ShellCommand> {
        shell(format!("sh get-docker.sh"))
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn login(&self) -> crate::Result<ShellCommand> {
        shell(format!(
            "docker login {host} -u {account} -p {token}",
            host = self.registry.host,
            account = self.registry.account,
            token = self.registry.token
        ))
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn list_containers(&self) -> crate::Result<ShellCommand> {
        shell(format!("docker ps"))
    }
}

impl From<&SupportControl> for DeploymentContext {
    fn from(controller: &SupportControl) -> Self {
        Self::from_controller(controller)
    }
}
