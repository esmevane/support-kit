use crate::{Color, LoggerConfig, NetworkConfig, ServiceConfig, ServiceName, VerbosityLevel};

use super::{Logging, LoggingConfig};

#[derive(Clone, Debug, Default, serde::Deserialize, PartialEq, bon::Builder)]
pub struct Config {
    #[serde(default)]
    #[builder(default, into)]
    pub logging: LoggingConfig,

    #[serde(default)]
    #[builder(default, into)]
    pub verbosity: VerbosityLevel,

    #[serde(default)]
    #[builder(default, into)]
    pub color: Color,

    #[serde(default)]
    #[builder(default, into)]
    pub server: NetworkConfig,

    #[serde(default)]
    #[builder(default, into)]
    pub service: ServiceConfig,
}

impl Config {
    pub fn init_logging(&self) -> Vec<tracing_appender::non_blocking::WorkerGuard> {
        Logging::initialize(self.clone())
    }

    pub fn loggers(&self) -> Vec<LoggerConfig> {
        self.logging.loggers()
    }

    pub fn name(&self) -> ServiceName {
        self.service.name()
    }

    pub fn env_filter(&self) -> tracing_subscriber::EnvFilter {
        let log_level = self.verbosity.to_string();
        let maybe_env_filter = tracing_subscriber::EnvFilter::try_from_default_env();

        if log_level.is_empty() {
            maybe_env_filter.unwrap_or_default()
        } else {
            maybe_env_filter.unwrap_or_else(|_| log_level.into())
        }
    }
}

mod sources {

    #[test]
    fn getting_config_from_json() -> Result<(), Box<dyn std::error::Error>> {
        use super::Config;

        use figment::{
            providers::{Format, Json},
            Figment,
        };

        figment::Jail::expect_with(|jail| {
            jail.create_file(
                "support-kit.json",
                r#"{
                    "server": { "host": "0.0.0.0" }
                }"#,
            )?;

            let config: Config = Figment::new()
                .merge(Json::file("support-kit.json"))
                .extract()?;

            assert_eq!(config, Config::builder().server("0.0.0.0").build());
            Ok(())
        });

        Ok(())
    }
}

#[test]
fn verbosity_and_env_filter() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = serde_json::from_str(
        r#"
        {
            "verbosity": "debug"
        }
        "#,
    )?;

    assert_eq!(
        config,
        Config::builder().verbosity(VerbosityLevel::Debug).build()
    );

    assert_eq!(config.env_filter().to_string(), "debug");

    let config: Config = serde_json::from_str(r#"{}"#)?;

    assert_eq!(config, Config::builder().build());
    assert_eq!(config.env_filter().to_string(), "");

    Ok(())
}

#[test]
fn root_config_notation() -> Result<(), Box<dyn std::error::Error>> {
    use super::{LogLevel, LogRotation, LoggerConfig};

    let config: Config = serde_json::from_str(
        r#"
        {
            "logging": [
                {
                    "directory": "logs",
                    "level": "warn",
                    "name": "app.error"
                },
                {
                    "directory": "logs",
                    "level": { "min": "info", "max": "warn" },
                    "name": "app",
                    "rotation": "daily"
                },
                {
                    "directory": "logs",
                    "level": { "min": "trace", "max": "info" },
                    "name": "app.debug",
                    "rotation": "per-minute"
                }
            ]
        }
        "#,
    )?;

    assert_eq!(
        config,
        Config::builder()
            .logging(bon::vec![
                LoggerConfig::builder()
                    .level(LogLevel::Warn)
                    .file(("logs", "app.error"))
                    .build(),
                LoggerConfig::builder()
                    .level(LogLevel::Info..LogLevel::Warn)
                    .file(("logs", "app", LogRotation::Daily))
                    .build(),
                LoggerConfig::builder()
                    .level(LogLevel::Trace..LogLevel::Info)
                    .file(("logs", "app.debug", LogRotation::PerMinute))
                    .build(),
            ])
            .build()
    );

    Ok(())
}

#[test]
fn server_config() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = serde_json::from_str(
        r#"
        {
            "server": {
                "host": "1.2.3.4",
                "port": 8080
            }
        }
        "#,
    )?;

    assert_eq!(config, Config::builder().server(("1.2.3.4", 8080)).build());

    let config: Config = serde_json::from_str(
        r#"
        {
            "server": {
                "host": "127.0.0.1"
            }
        }
        "#,
    )?;

    assert_eq!(config, Config::builder().server("127.0.0.1").build());

    let config: Config = serde_json::from_str(
        r#"
        {
            "server": {
                "port": 22
            }
        }
        "#,
    )?;

    assert_eq!(config, Config::builder().server(("0.0.0.0", 22)).build());

    let config: Config = serde_json::from_str(
        r#"
        {
        }
        "#,
    )?;

    assert_eq!(config, Config::builder().server(("0.0.0.0", 80)).build());

    Ok(())
}

#[test]
fn service_config() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = serde_json::from_str(
        r#"
        {
            "service": {
                "name": "consumer-package"
            }
        }
        "#,
    )?;

    assert_eq!(
        config,
        Config::builder().service("consumer-package").build()
    );

    let config: Config = serde_json::from_str(
        r#"
        {
            "service": {
                "name": "custom-name"
            }
        }
        "#,
    )?;

    assert_eq!(config, Config::builder().service("custom-name").build());

    let config: Config = serde_json::from_str(
        r#"
        {
            "service": {
                "system": true
            }
        }
        "#,
    )?;

    assert_eq!(
        config,
        Config::builder()
            .service(ServiceConfig::builder().system(true).build())
            .build()
    );

    let config: Config = serde_json::from_str(
        r#"
        {
            "service": {
                "system": false
            }
        }
        "#,
    )?;

    assert_eq!(
        config,
        Config::builder()
            .service(ServiceConfig::builder().system(false).build())
            .build()
    );

    Ok(())
}

#[test]
fn color_config() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = serde_json::from_str(
        r#"
        {
            "color": "auto"
        }
        "#,
    )?;

    assert_eq!(config, Config::builder().color(Color::Auto).build());

    let config: Config = serde_json::from_str(
        r#"
        {
            "color": "always"
        }
        "#,
    )?;

    assert_eq!(config, Config::builder().color(Color::Always).build());

    let config: Config = serde_json::from_str(
        r#"
        {
            "color": "never"
        }
        "#,
    )?;

    assert_eq!(config, Config::builder().color(Color::Never).build());

    Ok(())
}
