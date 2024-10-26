use std::path::PathBuf;

use crate::{shell, Configuration, ImageDefinition, Registry, ShellCommand};

#[derive(Debug, Clone, bon::Builder)]
pub struct ImageDeploymentContext {
    pub config: Configuration,
    pub image: ImageDefinition,
    pub registry: Registry,
}

impl ImageDeploymentContext {
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
    pub fn setup_config_volume(&self) -> crate::Result<ShellCommand> {
        shell(format!(
            "docker volume create {namespace}-{name}-config",
            name = self.image.name,
            namespace = self.image.namespace
        ))
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn kill_all(&self) -> crate::Result<ShellCommand> {
        shell(format!("docker kill {name}", name = self.name()))
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn push(&self) -> crate::Result<ShellCommand> {
        shell(format!(
            "docker push {descriptor}",
            descriptor = self.descriptor()
        ))
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn build(&self) -> crate::Result<ShellCommand> {
        let label = format!(
            "org.opencontainers.image.source={repo}",
            repo = self.image.repo
        );

        shell(format!(
            "docker build -f {definition} --label {label} -t {descriptor} .",
            definition = self.image.definition,
            descriptor = self.descriptor(),
        ))
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn pull(&self) -> crate::Result<ShellCommand> {
        shell(format!(
            "docker pull {descriptor}",
            descriptor = self.descriptor()
        ))
    }

    #[tracing::instrument(skip(self, path), level = "trace")]
    pub fn start(&self, path: impl Into<PathBuf>) -> crate::Result<ShellCommand> {
        let path = path.into();

        let operation = shell(format!(
            r#"
            docker run
              --rm
              -d 
              -p 443:{port}
              -e RUST_LOG=debug,support_kit=debug
              -v ./{path}:/{app_name}.json
              --mount source=certs,target=/certs
              --name {name}
              {descriptor}
              -vvvv
              --config-file {app_name}
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
