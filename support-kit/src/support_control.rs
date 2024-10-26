use bon::builder;
use figment::Figment;
use rustls_acme::axum::AxumAcceptor;

use crate::{
    Args, BoilerplateCommand, BoilerplatePreset, ConfigManifest, ConfigSources, Configuration,
    HostControl, ShellCommand, SupportKitError,
};

#[derive(Debug, Default, bon::Builder)]
pub struct SupportControl {
    pub args: Args,
    pub config: Configuration,
    #[builder(default, into)]
    _guards: Vec<tracing_appender::non_blocking::WorkerGuard>,
}

#[bon::bon]
impl SupportControl {
    #[tracing::instrument(skip(self), level = "trace")]
    pub fn manifest(&self) -> Result<ConfigManifest, SupportKitError> {
        Ok(self.source_collection().sources()?)
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn source_collection(&self) -> ConfigSources {
        ConfigSources::builder()
            .file(self.args.config())
            .maybe_env(self.config.environment)
            .build()
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn figment(&self) -> Result<Figment, SupportKitError> {
        Ok(Figment::new()
            .merge(&self.config)
            .merge(&self.source_collection()))
    }

    #[tracing::instrument(skip(args), level = "trace")]
    pub fn load_configuration(args: &Args) -> Result<Self, SupportKitError> {
        let initial_setup = Self::builder()
            .args(args.clone())
            .config(Configuration::from(args))
            .build();

        let controller = Self::builder()
            .args(args.clone())
            .config(initial_setup.figment()?.extract()?)
            .build();

        tracing::debug!(sources = ?controller.manifest()?.known(), "loaded configuration with sources");

        Ok(controller)
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn init(mut self) -> Self {
        self.config.init_color();
        self._guards = self.config.init_logging();
        self
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub async fn init_tls(&self) -> Option<AxumAcceptor> {
        self.config.init_tls().await
    }

    #[builder]
    #[tracing::instrument(skip(self), level = "trace")]
    pub async fn on_remotes(
        &self,
        #[builder(default, into)] commands: Vec<ShellCommand>,
    ) -> Result<(), SupportKitError> {
        let deployment = self.config.deployment.clone();

        if let Some(deployment) = deployment {
            HostControl::on_hosts(&deployment, commands).await?;
        }

        Ok(())
    }

    pub async fn execute(&self, args: Args) -> Result<(), SupportKitError> {
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
                    crate::Commands::Generate(boilerplate_args) => {
                        let control = crate::BoilerplateControl::from(self.config.clone());

                        match boilerplate_args.command {
                            Some(operation) => match operation {
                                BoilerplateCommand::Init => {
                                    for preset in BoilerplatePreset::all() {
                                        control.write(preset)?;
                                    }
                                }
                                BoilerplateCommand::Template { command: preset } => {
                                    control.write(preset)?;
                                }
                            },
                            None => {
                                tracing::info!(config = ?self.config, "no operation provided")
                            }
                        }
                    }
                    crate::Commands::Deploy(deployment_args) => match deployment_args.command {
                        Some(operation) => operation.exec_remote(&self).await?,
                        None => {}
                    },
                    crate::Commands::Container(deployment_args) => match deployment_args.command {
                        Some(operation) => operation.exec_local(&self).await?,
                        None => {}
                    },
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
        let control = SupportControl::load_configuration(&args).unwrap();

        assert_eq!(
            control.config,
            Configuration::builder()
                .color(crate::Color::Never)
                .environment(crate::Environment::Production)
                .service(
                    crate::ServiceConfig::builder()
                        .name("app")
                        .system(true)
                        .build()
                )
                .verbosity(crate::Verbosity::Trace)
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
        let control = SupportControl::load_configuration(&args).unwrap();

        assert_eq!(
            control.config,
            Configuration::builder()
                .color(crate::Color::Never)
                .environment(crate::Environment::Production)
                .service(
                    crate::ServiceConfig::builder()
                        .name("app")
                        .system(true)
                        .build()
                )
                .verbosity(crate::Verbosity::Trace)
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
        let control = SupportControl::load_configuration(&args).unwrap();

        assert_eq!(
            control.config,
            Configuration::builder()
                .color(crate::Color::Never)
                .environment(crate::Environment::Production)
                .service(
                    crate::ServiceConfig::builder()
                        .name("app")
                        .system(true)
                        .build()
                )
                .verbosity(crate::Verbosity::Trace)
                .build()
        );

        Ok(())
    });
}
