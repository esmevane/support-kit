use crate::{Registry, SupportControl};

use super::{ImageControl, OpsProcess};

pub struct ContainerControl {
    pub images: Vec<ImageControl>,
    pub registry: Registry,
}

impl ContainerControl {
    pub fn from_controller(controller: &SupportControl) -> Self {
        let config = controller.config.clone();
        let image_defs = config
            .deployment
            .clone()
            .and_then(|deployment| deployment.artifacts)
            .and_then(|artifacts| artifacts.containers)
            .map(|containers| containers.images)
            .unwrap_or_default();

        let registry = config
            .deployment
            .clone()
            .and_then(|deployment| deployment.artifacts)
            .and_then(|artifacts| artifacts.containers)
            .and_then(|containers| containers.registry)
            .unwrap_or_default();

        let mut images = vec![];

        for image in image_defs {
            images.push(ImageControl {
                config: config.clone(),
                image,
                registry: registry.clone(),
            });
        }

        Self { images, registry }
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn setup_cert_volume(&self) -> crate::Result<OpsProcess> {
        to_container_op(format!("docker volume create certs"))
    }

    /// Get the shell script from the [docker-install repo][docker-install].
    ///
    /// [docker-install]: https://github.com/docker/docker-install
    #[tracing::instrument(skip(self), level = "trace")]
    pub fn get_install_script(&self) -> crate::Result<OpsProcess> {
        to_container_op(format!(
            "curl -fsSL https://get.docker.com -o get-docker.sh"
        ))
    }

    /// Install Docker on the host machine.
    #[tracing::instrument(skip(self), level = "trace")]
    pub fn install_docker(&self) -> crate::Result<OpsProcess> {
        to_container_op(format!("sh get-docker.sh"))
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn login(&self) -> crate::Result<OpsProcess> {
        to_container_op(format!(
            "docker login {host} -u {account} -p {token}",
            host = self.registry.host,
            account = self.registry.account,
            token = self.registry.token
        ))
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn list_containers(&self) -> crate::Result<OpsProcess> {
        to_container_op(format!("docker ps"))
    }
}

impl From<&SupportControl> for ContainerControl {
    fn from(controller: &SupportControl) -> Self {
        Self::from_controller(controller)
    }
}

#[tracing::instrument(skip(operation), level = "trace")]
fn to_container_op<T: Into<String>>(operation: T) -> crate::Result<OpsProcess> {
    let operation = operation.into();

    tracing::trace!(operation = ?operation, "converting to operation");

    Ok(operation.try_into()?)
}
