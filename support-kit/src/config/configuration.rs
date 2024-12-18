use figment::{providers::Serialized, Figment, Provider};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

use crate::{
    Args, Color, DeploymentConfig, DeploymentControl, Environment, LoggerConfig, Logging,
    LoggingConfig, NetworkConfig, ServiceConfig, ServiceName, Verbosity,
};

#[derive(Clone, Debug, Default, Serialize, Deserialize, bon::Builder)]
pub struct Configuration {
    #[serde(default)]
    #[builder(default, into)]
    pub logging: LoggingConfig,

    #[serde(default)]
    #[builder(default, into)]
    pub verbosity: Verbosity,

    #[serde(default)]
    #[builder(default, into)]
    pub color: Color,

    #[serde(default)]
    #[builder(default, into)]
    pub server: NetworkConfig,

    #[serde(default)]
    #[builder(default, into)]
    pub service: ServiceConfig,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub environment: Option<Environment>,

    #[serde(default)]
    #[builder(into)]
    pub deployment: Option<DeploymentConfig>,

    #[serde(default, skip_serializing)]
    #[builder(default)]
    pub secret: SecretString,
}

impl Configuration {
    pub fn init_color(&self) {
        self.color.init();
    }

    pub fn init_logging(&self) -> Vec<tracing_appender::non_blocking::WorkerGuard> {
        Logging::initialize(self.clone())
    }

    pub async fn init_tls(&self) -> Option<rustls_acme::axum::AxumAcceptor> {
        match &self.deployment {
            Some(deployment) => DeploymentControl::initialize(deployment).await,
            None => None,
        }
    }

    pub fn loggers(&self) -> Vec<LoggerConfig> {
        self.logging.loggers()
    }

    pub fn name(&self) -> ServiceName {
        self.service.name()
    }

    pub fn address(&self) -> crate::Result<std::net::SocketAddr> {
        self.server.address()
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

impl Provider for Configuration {
    fn metadata(&self) -> figment::Metadata {
        Default::default()
    }

    fn data(
        &self,
    ) -> Result<figment::value::Map<figment::Profile, figment::value::Dict>, figment::Error> {
        Figment::new()
            .merge(Serialized::from(self.clone(), "default"))
            .data()
    }
}

impl PartialEq for Configuration {
    fn eq(&self, other: &Self) -> bool {
        self.logging == other.logging
            && self.verbosity == other.verbosity
            && self.color == other.color
            && self.server == other.server
            && self.service == other.service
            && self.environment == other.environment
            && self.deployment == other.deployment
    }
}

impl From<Args> for Configuration {
    fn from(args: Args) -> Self {
        let Args {
            color,
            environment,
            host,
            name,
            service_manager,
            system,
            port,
            verbose,
            ..
        } = args.clone();

        let verbosity_level = Verbosity::from_repr(verbose as usize);
        let server = match (&host, port) {
            (None, None) => None,
            _ => Some(
                NetworkConfig::builder()
                    .maybe_host(host)
                    .maybe_port(port)
                    .build(),
            ),
        };

        let service = ServiceConfig::builder()
            .maybe_name(name)
            .maybe_service_manager(service_manager)
            .system(system)
            .build();

        Self::builder()
            .maybe_verbosity(verbosity_level)
            .maybe_server(server)
            .maybe_environment(environment)
            .color(color)
            .service(service)
            .build()
    }
}

impl From<&Args> for Configuration {
    fn from(args: &Args) -> Self {
        args.clone().into()
    }
}

mod sources {

    #[test]
    fn getting_config_from_json() -> Result<(), Box<dyn std::error::Error>> {
        use super::Configuration;

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

            let config: Configuration = Figment::new()
                .merge(Json::file("support-kit.json"))
                .extract()?;

            assert_eq!(config, Configuration::builder().server("0.0.0.0").build());
            Ok(())
        });

        Ok(())
    }
}

#[test]
fn verbosity_and_env_filter() -> Result<(), Box<dyn std::error::Error>> {
    let config: Configuration = serde_json::from_str(
        r#"
        {
            "verbosity": "debug"
        }
        "#,
    )?;

    assert_eq!(
        config,
        Configuration::builder().verbosity(Verbosity::Debug).build()
    );

    assert_eq!(config.env_filter().to_string(), "debug");

    let config: Configuration = serde_json::from_str(r#"{}"#)?;

    assert_eq!(config, Configuration::builder().build());
    assert_eq!(config.env_filter().to_string(), "");

    Ok(())
}

#[test]
fn root_config_notation() -> Result<(), Box<dyn std::error::Error>> {
    use crate::{LogLevel, LogRotation, LoggerConfig};

    let config: Configuration = serde_json::from_str(
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
        Configuration::builder()
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
    let config: Configuration = serde_json::from_str(
        r#"
        {
            "server": {
                "host": "1.2.3.4",
                "port": 8080
            }
        }
        "#,
    )?;

    assert_eq!(
        config,
        Configuration::builder().server(("1.2.3.4", 8080)).build()
    );

    let config: Configuration = serde_json::from_str(
        r#"
        {
            "server": {
                "host": "127.0.0.1"
            }
        }
        "#,
    )?;

    assert_eq!(config, Configuration::builder().server("127.0.0.1").build());

    let config: Configuration = serde_json::from_str(
        r#"
        {
            "server": {
                "port": 22
            }
        }
        "#,
    )?;

    assert_eq!(
        config,
        Configuration::builder().server(("0.0.0.0", 22)).build()
    );

    let config: Configuration = serde_json::from_str(
        r#"
        {
        }
        "#,
    )?;

    assert_eq!(
        config,
        Configuration::builder().server(("0.0.0.0", 80)).build()
    );

    Ok(())
}

#[test]
fn service_config() -> Result<(), Box<dyn std::error::Error>> {
    let config: Configuration = serde_json::from_str(
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
        Configuration::builder().service("consumer-package").build()
    );

    let config: Configuration = serde_json::from_str(
        r#"
        {
            "service": {
                "name": "custom-name"
            }
        }
        "#,
    )?;

    assert_eq!(
        config,
        Configuration::builder().service("custom-name").build()
    );

    let config: Configuration = serde_json::from_str(
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
        Configuration::builder()
            .service(ServiceConfig::builder().system(true).build())
            .build()
    );

    let config: Configuration = serde_json::from_str(
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
        Configuration::builder()
            .service(ServiceConfig::builder().system(false).build())
            .build()
    );

    Ok(())
}

#[test]
fn color_config() -> Result<(), Box<dyn std::error::Error>> {
    let config: Configuration = serde_json::from_str(
        r#"
        {
            "color": "auto"
        }
        "#,
    )?;

    assert_eq!(config, Configuration::builder().color(Color::Auto).build());

    let config: Configuration = serde_json::from_str(
        r#"
        {
            "color": "always"
        }
        "#,
    )?;

    assert_eq!(
        config,
        Configuration::builder().color(Color::Always).build()
    );

    let config: Configuration = serde_json::from_str(
        r#"
        {
            "color": "never"
        }
        "#,
    )?;

    assert_eq!(config, Configuration::builder().color(Color::Never).build());

    Ok(())
}

#[test]
fn environment_config() -> Result<(), Box<dyn std::error::Error>> {
    let config: Configuration = serde_json::from_str(
        r#"
        {
            "environment": "development"
        }
        "#,
    )?;

    assert_eq!(
        config,
        Configuration::builder()
            .environment(Environment::Development)
            .build()
    );

    let config: Configuration = serde_json::from_str(
        r#"
        {
            "environment": "production"
        }
        "#,
    )?;

    assert_eq!(
        config,
        Configuration::builder()
            .environment(Environment::Production)
            .build()
    );

    let config: Configuration = serde_json::from_str(
        r#"
        {
            "environment": "test"
        }
        "#,
    )?;

    assert_eq!(
        config,
        Configuration::builder()
            .environment(Environment::Test)
            .build()
    );

    Ok(())
}
