use crate::{Configuration, Image, Registry};

use super::OpsProcess;

pub struct ImageControl {
    pub config: Configuration,
    pub image: Image,
    pub registry: Registry,
}

impl ImageControl {
    fn descriptor(&self) -> String {
        format!(
            "{repo}/{namespace}/{name}:{label}",
            name = self.image.name,
            namespace = self.image.namespace,
            repo = self.registry.host,
            label = self.image.label
        )
    }

    fn name(&self) -> String {
        format!(
            "{namespace}-{name}-deployment",
            name = self.image.name,
            namespace = self.image.namespace,
        )
    }

    pub fn setup_config_volume(&self) -> crate::Result<OpsProcess> {
        Ok(format!(
            "docker volume create {namespace}-{name}-config",
            name = self.image.name,
            namespace = self.image.namespace
        )
        .try_into()?)
    }

    pub fn kill_all(&self) -> crate::Result<OpsProcess> {
        let container_ids = format!("docker ps -qf name={name}", name = self.name());

        Ok(format!("docker kill $({container_ids})").try_into()?)
    }

    pub fn push(&self) -> crate::Result<OpsProcess> {
        Ok(format!("docker push {descriptor}", descriptor = self.descriptor()).try_into()?)
    }

    pub fn build(&self) -> crate::Result<OpsProcess> {
        let label = format!(
            "org.opencontainers.image.source={repo}",
            repo = self.image.repo
        );

        Ok(format!(
            "docker build \
            -f {definition} \
            --label {label} \
            -t {descriptor} ..",
            definition = self.image.definition,
            descriptor = self.descriptor(),
        )
        .try_into()?)
    }

    pub fn pull(&self) -> crate::Result<OpsProcess> {
        Ok(format!("docker pull {descriptor}", descriptor = self.descriptor()).try_into()?)
    }

    pub fn start(&self) -> crate::Result<OpsProcess> {
        Ok(format!(
            "docker run --rm \
            -p 443:{port} \
            --mount source=certs,target=/certs \
            -e RUST_LOG=debug \
            {descriptor}",
            descriptor = self.descriptor(),
            port = self.config.server.port,
        )
        .try_into()?)
    }
}
