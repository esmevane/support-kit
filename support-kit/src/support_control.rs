use convert_case::{Case, Casing};
use figment::{
    providers::{Env, Format, Json, Serialized, Toml, Yaml},
    Figment,
};
use std::path::PathBuf;

use crate::{Args, Config, MissingDirError, SupportKitError};

#[derive(Default)]
pub struct SupportControl {
    pub config: Config,
    _guards: Vec<tracing_appender::non_blocking::WorkerGuard>,
}

impl SupportControl {
    pub fn from_config(config: Config) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    pub fn load_configuartion(args: &Args) -> Result<Self, SupportKitError> {
        let home_dir = dirs::home_dir().ok_or(MissingDirError::HomeDir)?;
        let config_dir = dirs::config_dir().ok_or(MissingDirError::ConfigDir)?;
        let base_file_name = args.config();
        let base_config = args.build_config();
        let config_env = base_config.environment.to_string();
        let name = base_config.name();

        let files = [
            home_dir.join(&base_file_name),
            config_dir.join(&base_file_name),
            PathBuf::from(&base_file_name),
        ];

        let mut figment = Figment::new().merge(Serialized::from(base_config, "default"));
        for file in files {
            let file = String::from(file.to_string_lossy());
            let env_file = format!("{file}.{config_env}");

            figment = figment
                .merge(Yaml::file(format!("{file}.yaml")))
                .merge(Json::file(format!("{file}.json")))
                .merge(Toml::file(format!("{file}.toml")))
                .merge(Yaml::file(format!("{env_file}.yaml")))
                .merge(Json::file(format!("{env_file}.json")))
                .merge(Toml::file(format!("{env_file}.toml")));
        }

        let prefix = format!("{name}_").to_case(Case::UpperSnake);
        let env_prefix = format!("{name}_{config_env}_").to_case(Case::UpperSnake);

        figment = figment
            .merge(Env::prefixed(&prefix).split("__"))
            .merge(Env::prefixed(&env_prefix).split("__"));

        Ok(Self::from_config(figment.extract()?))
    }

    pub fn init(mut self) -> Self {
        self.config.init_color();
        self._guards = self.config.init_logging();
        self
    }

    pub fn execute(&self, args: Args) -> Result<(), SupportKitError> {
        match args.command {
            Some(command) => {
                tracing::info!(
                    command = ?command,
                    config = ?self.config,
                    "executing command"
                );

                match command {
                    crate::Commands::Service(service_args) => {
                        let control = crate::ServiceControl::init(&self.config)?;

                        match service_args.operation {
                            Some(operation) => control.execute(operation)?,
                            None => {
                                tracing::info!(config = ?self.config, "no operation provided")
                            }
                        }
                    }
                }
            }
            None => tracing::trace!(config = ?&self.config, "no command provided."),
        }

        Ok(())
    }
}

#[test]
fn yaml_config() {
    use crate::Environment;
    use clap::Parser;

    figment::Jail::expect_with(|jail| {
        dbg!(jail.create_file(
            "support-kit.yaml",
            r#"
            environment: production
            name: app
            service:
                service_manager: systemd
                system: true
            verbose: 1
        "#,
        ))?;

        let config: Config = Figment::new()
            .merge(Yaml::file("support-kit.yaml"))
            .extract()?;

        let args = Args::try_parse_from("app".split_whitespace()).unwrap();
        let control = SupportControl::load_configuartion(&args).unwrap();

        // dbg!(config);
        assert_eq!(control.config.environment, Environment::Production);

        Ok(())
    });
}
