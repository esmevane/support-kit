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
    pub fn parse() -> crate::Result<(Self, WorkerGuard)> {
        let cli = Cli::parse();
        let sources = ConfigurationSources {
            home_config: cli.home_config(),
            root_config: cli.root_config(),
            env_config: cli.env_config(),
            env_prefix: APP_NAME.to_uppercase(),
            env_separator: "_".to_string(),
        };

        let config_builder = Config::builder()
            .set_default("logging.console.kind", "single")?
            .set_default("logging.console.verbosity", "info")?
            .set_default("logging.loggers[0].kind", "file")?
            .set_default("logging.loggers[0].path", "logs")?
            .set_default("logging.loggers[0].name", "error")?
            // .set_default("logging.loggers[0].level.kind", "single")?
            // .set_default("logging.loggers[0].level.verbosity", "error")?
            .set_default("logging.loggers[0].level.kind", "min-max")?
            .set_default("logging.loggers[0].level.min", "error")?
            .set_default("logging.loggers[0].level.max", "warn")?
            .set_default("logging.loggers[1].kind", "rolling")?
            .set_default("logging.loggers[1].path", "logs")?
            .set_default("logging.loggers[1].name", "out")?
            .set_default("logging.loggers[1].rotation", "daily")?
            .set_default("logging.loggers[1].level.kind", "single")?
            .set_default("logging.loggers[1].level.verbosity", "info")?
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
        let guard = telemetry::init(&settings);

        settings.cli.global.color.init();

        Ok((settings, guard))
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
    pub fn storage_path(&self) -> PathBuf {
        tracing::info!("Getting storage path");
        let path = match &self.config.storage {
            Some(storage) => storage.path.clone(),
            None => {
                // use directories to get a default data directory in user's config path
                match dirs::config_local_dir() {
                    Some(mut path) => {
                        path.push(APP_NAME.to_lowercase());
                        path.push("storage.db");
                        path
                    }
                    None => {
                        // otherwise we start in a temp directory
                        std::env::temp_dir().join("storage.db")
                    }
                }
            }
        };

        tracing::info!("Using storage path: {}", path.display());

        // ensure the file and path exist
        if let Some(parent) = path.parent() {
            tracing::info!("Ensuring storage path exists: {:?}", parent);
            std::fs::create_dir_all(parent).expect("Unable to create storage directory");
        }

        if std::fs::metadata(&path).is_err() {
            tracing::info!("Ensuring storage file exists: {:?}", path);
            std::fs::File::create(&path).expect("Unable to create storage file");
        }

        path
    }
}
