mod configuration;
mod environment;
mod logging;
mod network;
mod service;

use config::Config;
use serde::Serialize;
use std::path::PathBuf;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;

use configuration::Configuration;
use logging::Log;

pub use environment::Environment;
pub use logging::Logging;
pub use network::Network;
pub use service::Service;

use crate::telemetry;

pub trait SourceProvider {
    const APP_NAME: &'static str;

    fn base_config(&self) -> BaseConfig;
    fn home_config_path(&self) -> String;
    fn environment_scoped_config_path(&self) -> String;
    fn root_config_path(&self) -> String;
    fn env_var_prefix(&self) -> String;
    fn env_var_separator(&self) -> String;

    fn sources(&self) -> ConfigurationSources {
        ConfigurationSources {
            app_name: Self::APP_NAME.to_string(),
            home_config: self.home_config_path(),
            root_config: self.root_config_path(),
            env_config: self.environment_scoped_config_path(),
            env_prefix: self.env_var_prefix(),
            env_separator: self.env_var_separator(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BaseConfig {
    pub verbosity: Level,
}

#[derive(Debug, Clone)]
pub struct ConfigurationSources {
    pub app_name: String,
    pub home_config: String,
    pub root_config: String,
    pub env_config: String,
    pub env_prefix: String,
    pub env_separator: String,
}

#[derive(Clone, Debug)]
pub struct Settings {
    pub app_name: String,
    pub verbosity: Level,
    pub config: Configuration,
    pub sources: ConfigurationSources,
}

impl Settings {
    pub fn parse<T: SourceProvider + Serialize>(cli: T) -> crate::Result<(Self, Vec<WorkerGuard>)> {
        let sources = cli.sources();

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
        let mut settings = Settings {
            app_name: T::APP_NAME.into(),
            verbosity: cli.base_config().verbosity,
            config,
            sources,
        };

        let guards = telemetry::init(&settings);
        let storage = settings.config.storage.initialize(&settings)?;

        settings.config.storage = storage;

        // settings.cli.global.color.init();

        Ok((settings, guards))
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub fn loggers(&self) -> Vec<Log> {
        let mut loggers = self.config.logging.loggers.clone();

        if loggers.is_empty() {
            loggers = vec![Log::error_logger(self), Log::rolling_info_logger(self)];

            if self.config.environment.is_development() {
                loggers.push(Log::rolling_debug_logger(self))
            }
        }

        loggers
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub fn storage_path(&self) -> PathBuf {
        self.config.storage.path.clone()
    }
}
