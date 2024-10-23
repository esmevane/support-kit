use std::path::PathBuf;

use crate::{Configuration, Image, Registry};

use super::OpsProcess;

pub struct ImageControl {
    pub config: Configuration,
    pub image: Image,
    pub registry: Registry,
}

impl ImageControl {
    #[tracing::instrument(skip(self), level = "trace")]
    fn descriptor(&self) -> String {
        format!(
            "{repo}/{namespace}/{name}:{label}",
            name = self.image.name,
            namespace = self.image.namespace,
            repo = self.registry.host,
            label = self.image.label
        )
    }

    #[tracing::instrument(skip(self), level = "trace")]
    fn name(&self) -> String {
        format!(
            "{namespace}-{name}-deployment",
            name = self.image.name,
            namespace = self.image.namespace,
        )
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn setup_config_volume(&self) -> crate::Result<OpsProcess> {
        to_image_op(format!(
            "docker volume create {namespace}-{name}-config",
            name = self.image.name,
            namespace = self.image.namespace
        ))
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn kill_all(&self) -> crate::Result<OpsProcess> {
        let container_ids = format!("docker ps -qf name={name}", name = self.name());

        to_image_op(format!("docker kill $({container_ids})"))
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn push(&self) -> crate::Result<OpsProcess> {
        to_image_op(format!(
            "docker push {descriptor}",
            descriptor = self.descriptor()
        ))
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn build(&self) -> crate::Result<OpsProcess> {
        let label = format!(
            "org.opencontainers.image.source={repo}",
            repo = self.image.repo
        );

        to_image_op(format!(
            "docker build \
            -f {definition} \
            --label {label} \
            -t {descriptor} .",
            definition = self.image.definition,
            descriptor = self.descriptor(),
        ))
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn pull(&self) -> crate::Result<OpsProcess> {
        to_image_op(format!(
            "docker pull {descriptor}",
            descriptor = self.descriptor()
        ))
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn emit_config(&self) -> crate::Result<PathBuf> {
        let path =
            std::env::temp_dir().join(format!("{name}.container.json", name = self.config.name()));

        let contents = serde_json::to_string(&self.config)?;

        tracing::trace!(path = ?path, contents = ?contents,"writing container config file");

        std::fs::write(&path, contents).expect("Unable to write file");

        Ok(path)
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn start(&self) -> crate::Result<OpsProcess> {
        let path = self.emit_config()?;

        let operation = to_image_op(format!(
            r#"
            docker run
              --rm
              -p 443:{port}
              -e RUST_LOG="debug,support_kit=debug"
              -v {path}:/{app_name}.json
              --mount source=certs,target=/certs
              --name {name}
              {descriptor}
              --config-file /{app_name}.json
              --port {port}
            "#,
            descriptor = self.descriptor(),
            app_name = self.config.name(),
            name = self.name(),
            port = self.config.server.port,
            path = path.display()
        ));

        println!("{:?}", operation);

        operation
    }
}

#[tracing::instrument(skip(operation), level = "trace")]
fn to_image_op<T: Into<String>>(operation: T) -> crate::Result<OpsProcess> {
    let operation = operation.into();

    tracing::trace!(operation = ?operation, "converting to operation");

    Ok(operation.try_into()?)
}
