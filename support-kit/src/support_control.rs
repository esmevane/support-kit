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
        let name = base_config.name();
        let paths = [home_dir.clone(), config_dir.clone(), PathBuf::new()];

        let mut figment = Figment::new().merge(Serialized::from(base_config, "default"));
        for path in &paths {
            let file = path.with_file_name(&base_file_name);

            figment = figment
                .merge(Yaml::file(file.with_extension("yaml")))
                .merge(Json::file(file.with_extension("json")))
                .merge(Toml::file(file.with_extension("toml")));
        }

        let interim_config = figment.extract::<Config>()?;
        let config_env = interim_config.environment.to_string();
        let mut figment = Figment::new().merge(Serialized::from(interim_config, "default"));

        for path in paths {
            let file = path.with_file_name(&base_file_name);

            figment = figment
                .merge(Yaml::file(
                    file.with_extension(format!("{config_env}.yaml")),
                ))
                .merge(Json::file(
                    file.with_extension(format!("{config_env}.json")),
                ))
                .merge(Toml::file(
                    file.with_extension(format!("{config_env}.toml")),
                ));
        }

        let prefix = format!(
            "{name}__",
            name = name.to_string().to_case(Case::UpperSnake)
        );
        let env_prefix = format!(
            "{name}__{config_env}__",
            name = name.to_string().to_case(Case::UpperSnake),
            config_env = config_env.to_case(Case::UpperSnake)
        );

        figment = figment
            .merge(dbg!(Env::prefixed(&prefix).split("__")))
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
fn yaml_config_precedence_flow() {
    use clap::Parser;

    figment::Jail::expect_with(|jail| {
        jail.create_file(
            "support-kit.yaml",
            r#"
            environment: production
        "#,
        )?;

        jail.create_file(
            "support-kit.production.yaml",
            r#"
            environment: production
            service:
                name: app
                system: true
            verbosity: warn
        "#,
        )?;

        jail.set_env("SUPPORT_KIT__COLOR", "never");
        jail.set_env("SUPPORT_KIT__PRODUCTION__VERBOSITY", "trace");

        let args = Args::try_parse_from("app".split_whitespace()).unwrap();
        let control = SupportControl::load_configuartion(&args).unwrap();

        assert_eq!(
            control.config,
            Config::builder()
                .color(crate::Color::Never)
                .environment(crate::Environment::Production)
                .service(
                    crate::ServiceConfig::builder()
                        .name("app")
                        .system(true)
                        .build()
                )
                .verbosity(crate::VerbosityLevel::Trace)
                .build()
        );

        Ok(())
    });
}

#[test]
fn json_config_precedence_flow() {
    use clap::Parser;

    figment::Jail::expect_with(|jail| {
        jail.create_file(
            "support-kit.json",
            r#"
            {
                "environment": "production"
            }
        "#,
        )?;

        jail.create_file(
            "support-kit.production.json",
            r#"
            {
                "environment": "production",
                "service": {
                    "name": "app",
                    "system": true
                },
                "verbosity": "warn"
            }
        "#,
        )?;

        jail.set_env("SUPPORT_KIT__COLOR", "never");
        jail.set_env("SUPPORT_KIT__PRODUCTION__VERBOSITY", "trace");

        let args = Args::try_parse_from("app".split_whitespace()).unwrap();
        let control = SupportControl::load_configuartion(&args).unwrap();

        assert_eq!(
            control.config,
            Config::builder()
                .color(crate::Color::Never)
                .environment(crate::Environment::Production)
                .service(
                    crate::ServiceConfig::builder()
                        .name("app")
                        .system(true)
                        .build()
                )
                .verbosity(crate::VerbosityLevel::Trace)
                .build()
        );

        Ok(())
    });
}

#[test]
fn toml_config_precedence_flow() {
    use clap::Parser;

    figment::Jail::expect_with(|jail| {
        jail.create_file(
            "support-kit.toml",
            r#"
            environment = "production"
        "#,
        )?;

        jail.create_file(
            "support-kit.production.toml",
            r#"
            environment = "production"
            verbosity = "warn"

            [service]
            name = "app"
            system = true
        "#,
        )?;

        jail.set_env("SUPPORT_KIT__COLOR", "never");
        jail.set_env("SUPPORT_KIT__PRODUCTION__VERBOSITY", "trace");

        let args = Args::try_parse_from("app".split_whitespace()).unwrap();
        let control = SupportControl::load_configuartion(&args).unwrap();

        assert_eq!(
            control.config,
            Config::builder()
                .color(crate::Color::Never)
                .environment(crate::Environment::Production)
                .service(
                    crate::ServiceConfig::builder()
                        .name("app")
                        .system(true)
                        .build()
                )
                .verbosity(crate::VerbosityLevel::Trace)
                .build()
        );

        Ok(())
    });
}
