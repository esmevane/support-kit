use std::path::PathBuf;

use crate::{shell, Configuration, ImageDefinition, Registry, ShellCommand, ShellCommandError};

#[derive(Debug, Clone, bon::Builder)]
pub struct ImageDeploymentContext {
    pub config: Configuration,
    pub image: ImageDefinition,
    pub registry: Registry,
}

impl ImageDeploymentContext {
    #[tracing::instrument(skip(self), level = "trace")]
    fn volume_name(&self, volume: &str) -> String {
        format!(
            "{namespace}-{name}-{volume}",
            name = self.image.name,
            namespace = self.image.namespace,
            volume = volume
        )
    }

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
    pub fn setup_volume(&self, volume: &str) -> crate::Result<ShellCommand> {
        shell(format!(
            "docker volume create {volume_name}",
            volume_name = self.volume_name(volume)
        ))
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn setup_data_volume(&self) -> crate::Result<ShellCommand> {
        self.setup_volume("data")
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn setup_log_volume(&self) -> crate::Result<ShellCommand> {
        self.setup_volume("logs")
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn setup_config_volume(&self) -> crate::Result<ShellCommand> {
        self.setup_volume("config")
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

        let path = if path.is_absolute() {
            path
        } else {
            std::env::current_dir()
                .map_err(ShellCommandError::from)?
                .join(path)
        };

        let operation = shell(format!(
            r#"
            docker run
              --rm
              -d 
              -p 443:{port}
              -e RUST_LOG=debug,support_kit=debug
              -v {path}:/{app_name}.json
              --mount source={certs},target=/certs
              --mount source={logs},target=/logs
              --mount source={data},target=/data
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
            path = path.display(),
            certs = self.volume_name("certs"),
            logs = self.volume_name("logs"),
            data = self.volume_name("data"),
        ));

        println!("{:?}", operation);

        operation
    }
}
