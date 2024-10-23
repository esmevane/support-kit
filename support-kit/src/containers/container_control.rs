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

    pub fn setup_cert_volume(&self) -> crate::Result<OpsProcess> {
        Ok(format!("docker volume create certs").try_into()?)
    }

    /// Get the shell script from the [docker-install repo][docker-install].
    ///
    /// [docker-install]: https://github.com/docker/docker-install
    pub fn get_install_script(&self) -> crate::Result<OpsProcess> {
        Ok(format!("curl -fsSL https://get.docker.com -o get-docker.sh").try_into()?)
    }

    /// Install Docker on the host machine.
    pub fn install_docker(&self) -> crate::Result<OpsProcess> {
        Ok(format!("sh get-docker.sh").try_into()?)
    }

    pub fn login(&self) -> crate::Result<OpsProcess> {
        Ok(format!(
            "docker login {host} -u {account} -p {token}",
            host = self.registry.host,
            account = self.registry.account,
            token = self.registry.token
        )
        .try_into()?)
    }

    pub fn list_containers(&self) -> crate::Result<OpsProcess> {
        Ok(format!("docker ps").try_into()?)
    }
}

impl From<&SupportControl> for ContainerControl {
    fn from(controller: &SupportControl) -> Self {
        Self::from_controller(controller)
    }
}
