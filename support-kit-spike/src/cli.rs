use clap::Parser;
use figment::{providers::Serialized, Figment, Provider};
use serde::{Deserialize, Serialize};

use crate::{Config, EnvSourceName, Environment, Logging, SourceName};

#[derive(Parser, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Cli {
    #[clap(short, long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,
    #[clap(short, long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<SourceName>,
    // #[clap(short, long)]
    // database: String,
    // #[clap(short, long)]
    // storage: String,
    // #[clap(short, long)]
    // network: String,
    // #[clap(short, long)]
    // service: String,
    #[command(flatten)]
    pub logging: Logging,
}

impl PartialEq for Cli {
    fn eq(&self, other: &Self) -> bool {
        self.environment == other.environment && self.name == other.name
    }
}

impl Provider for Cli {
    fn metadata(&self) -> figment::Metadata {
        figment::Metadata::named("Cli args")
    }

    fn data(&self) -> figment::Result<figment::value::Map<figment::Profile, figment::value::Dict>> {
        let name = self.name.clone().unwrap_or_default();
        let partial_config: Config = Figment::new()
            .merge(name.source())
            .merge(Serialized::defaults(self.clone()))
            .extract()?;

        let env = partial_config.environment.clone();
        let env_name = EnvSourceName::new(name, env);

        Figment::new()
            .merge(Serialized::defaults(partial_config))
            .merge(env_name.source())
            .merge(Serialized::defaults(self))
            .data()
    }
}

#[test]
fn consuming_with_an_app_cli() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Debug, Parser)]
    struct ConsumerCli {
        #[clap(flatten)]
        support: Cli,
    }

    let command = "app_name --level trace";
    let args = command.split_whitespace();
    let consumer_cli = ConsumerCli::try_parse_from(args.clone()).unwrap();
    let cli = Cli::try_parse_from(args).unwrap();

    assert_eq!(consumer_cli.support, cli);

    Ok(())
}

#[test]
fn setting_named_source_by_cli() -> Result<(), Box<dyn std::error::Error>> {
    // defaults
    {
        let command = "app_name";
        let args = command.split_whitespace();
        let config = Cli::try_parse_from(args)?;

        assert_eq!(config.name, None)
    }

    // custom
    {
        let command = "app_name -n my_cool_app";
        let args = command.split_whitespace();
        let config = Cli::try_parse_from(args)?;

        assert_eq!(config.name, Some(SourceName::try_new("my_cool_app")?))
    }

    Ok(())
}

#[test]
fn setting_environment_by_cli() -> Result<(), Box<dyn std::error::Error>> {
    use std::error::Error;

    {
        let command = format!("app_name");
        let args = command.split_whitespace();
        let config = Cli::try_parse_from(args)?;

        assert_eq!(config.environment, None)
    }

    {
        for environment in ["test", "development", "production"] {
            let command = format!("app_name -e {environment}");
            let args = command.split_whitespace();
            let config = Cli::try_parse_from(args)?;

            assert_eq!(config.environment, Some(Environment::try_new(environment)?))
        }
    }

    {
        let command = "app_name -e invalid";
        let args = command.split_whitespace();
        let error = Cli::try_parse_from(args).unwrap_err();

        assert_eq!(
            error.source().unwrap().to_string(),
            "The environment is not valid: invalid, \
                expected one of: test, development, production."
        )
    }

    Ok(())
}
