use figment::Figment;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{Cli, Color, Environment, Logging, SourceName};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "Environment::default")]
    pub environment: Environment,
    #[serde(default = "SourceName::default")]
    pub name: SourceName,
    #[serde(default = "Logging::default")]
    pub logging: Logging,
    #[serde(default = "Color::default")]
    pub color: Color,
}

#[test]
fn deserializing_cli_into_config() -> Result<(), Box<dyn std::error::Error>> {
    let cli = serde_json::to_string(&Cli::default())?;
    let config: Config = serde_json::from_str(&cli)?;

    assert_eq!(config.environment, Environment::default());

    Ok(())
}

impl Config {
    pub fn try_from_cli(cli: Cli) -> Result<Self, ConfigError> {
        Ok(Figment::new().merge(cli).extract()?)
    }
}

#[derive(Clone, Debug, Error)]
pub enum ConfigError {
    #[error("Unable to initialize configuration: {0}")]
    InitializationError(#[from] figment::Error),
}

#[test]
fn loading_from_consumer_cli() -> Result<(), Box<dyn std::error::Error>> {
    use clap::Parser;

    #[derive(Debug, clap::Parser)]
    struct ConsumerCli {
        #[clap(flatten)]
        support: crate::Cli,
    }

    figment::Jail::expect_with(|jail| {
        jail.create_file(
            "support-kit.toml",
            r#"
                environment = "development"
            "#,
        )?;

        jail.create_file(
            "support-kit.json",
            r#"
                {
                    "environment": "production"
                }
            "#,
        )?;

        jail.create_file(
            "support-kit.test.toml",
            r#"
                logging.level = "trace"
            "#,
        )?;

        jail.set_env("SUPPORT_KIT_ENVIRONMENT", "test");

        let command = "app_name --log-filter off";
        let args = command.split_whitespace();
        let cli = ConsumerCli::try_parse_from(args).unwrap();
        let config = Config::try_from_cli(cli.support.clone()).expect("Unable to load config");

        assert_eq!(config.environment, Environment::try_new("test").unwrap());
        assert_eq!(config.name, SourceName::try_new("support-kit").unwrap());
        assert_eq!(config.logging.level, Some(crate::Level::Trace));
        assert_eq!(config.logging.log_filter, cli.support.logging.log_filter,);

        Ok(())
    });

    Ok(())
}
