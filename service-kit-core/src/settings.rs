mod client;
mod configuration;
mod environment;
mod network_settings;
mod server;
mod service;
mod service_settings;

use clap::Parser;
use config::Config;
use std::path::PathBuf;
use tracing_appender::non_blocking::WorkerGuard;

use crate::{
    cli::{Cli, Command},
    telemetry, APP_NAME,
};

use client::ClientResource;
use configuration::Configuration;
use server::ServerMode;
use service::ServiceOperation;

pub use client::Client;
pub use environment::Environment;
pub use network_settings::NetworkSettings;
pub use server::Server;
pub use service::Service;
pub use service_settings::ServiceSettings;

#[derive(Clone, Debug)]
pub struct Settings {
    pub cli: Cli,
    pub config: Configuration,
    pub sources: ConfigurationSources,
}

#[derive(Debug, Clone)]
pub struct ConfigurationSources {
    pub home_config: String,
    pub root_config: String,
    pub env_config: String,
    pub env_prefix: String,
    pub env_separator: String,
}

impl Settings {
    pub fn parse() -> crate::Result<(Self, Vec<WorkerGuard>)> {
        let cli = Cli::parse();
        let sources = ConfigurationSources {
            home_config: cli.home_config(),
            root_config: cli.root_config(),
            env_config: cli.env_config(),
            env_prefix: APP_NAME.to_uppercase(),
            env_separator: "_".to_string(),
        };

        let config_builder = Config::builder()
            .add_source(config::File::with_name(&sources.home_config).required(false))
            .add_source(config::File::with_name(&sources.root_config).required(false))
            .add_source(config::File::with_name(&sources.env_config).required(false))
            .add_source(
                config::Environment::with_prefix(&sources.env_prefix)
                    .separator(&sources.env_separator),
            )
            .build()?;

        let config: Configuration = config_builder.try_deserialize()?;
        let settings = Settings {
            cli,
            config,
            sources,
        };
        let guards = telemetry::init(&settings);

        settings.cli.global.color.init();

        Ok((settings, guards))
    }

    pub async fn exec(&self) -> crate::Result<()> {
        let cli = self.cli.clone();

        match cli.command {
            Command::Version => {
                tracing::info!("Version");

                println!("{}", crate::build::PKG_VERSION);
            }
            Command::Debug => {
                tracing::info!("Debugging");

                println!("{:#?}", self);
            }
            Command::Tui => {
                tracing::info!("Starting TUI");

                crate::tui::init().await?;
            }
            Command::Server(server_details) => {
                tracing::info!("Server command: {:?}", server_details);

                let context =
                    crate::context::WebContext::new(server_details.settings, self.clone()).await?;

                match server_details.mode {
                    Some(mode) => mode.exec(context).await?,
                    None => {
                        tracing::info!("No server mode specified, prompting");

                        ServerMode::select()?.exec(context).await?;
                    }
                }
            }
            Command::Client(client_details) => {
                tracing::info!("Client command");

                let response = match client_details.resource {
                    Some(resource) => resource.exec(client_details.settings).await?,
                    None => {
                        tracing::info!("No client resource specified, prompting");

                        ClientResource::select()?
                            .exec(client_details.settings)
                            .await?
                    }
                };

                tracing::info!("{}", response);
            }
            Command::Service(service_details) => {
                tracing::info!("Service command: {:?}", service_details);

                match service_details.operation {
                    Some(operation) => operation.exec(service_details.settings).await?,
                    None => {
                        tracing::info!("No service operation specified, prompting");

                        ServiceOperation::select()?
                            .exec(service_details.settings)
                            .await?;
                    }
                }
            }
        }

        Ok(())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub fn loggers(&self) -> Vec<configuration::logging::Log> {
        let mut loggers = self.config.logging.loggers.clone();

        if self.cli.global.environment.is_development() {
            loggers.push(configuration::logging::Log::rolling_debug_logger())
        }

        loggers
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub fn storage_path(&self) -> PathBuf {
        self.config.storage.path.clone()
    }
}
