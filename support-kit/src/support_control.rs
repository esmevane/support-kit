use figment::Figment;

use crate::{Args, Config, Sources, SupportKitError};

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
        let base_config = args.build_config();

        let sources = Sources::builder().name(base_config.name().clone()).build();
        let figment = Figment::new().merge(base_config).merge(sources.clone());

        let env = figment.extract::<Config>()?.environment;

        Ok(Self::from_config(
            figment
                .merge(sources.with_env(env))
                .merge(sources.prefix())
                .merge(sources.with_env(env).prefix())
                .extract()?,
        ))
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
