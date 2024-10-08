use crate::{Args, Config, SupportKitError};

#[derive(Default)]
pub struct SupportControl {
    pub config: Config,
    _guards: Vec<tracing_appender::non_blocking::WorkerGuard>,
}

impl SupportControl {
    pub fn from_args(args: &Args) -> Self {
        Self::from_config(args.config())
    }

    pub fn from_config(config: Config) -> Self {
        Self {
            config,
            ..Default::default()
        }
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

impl From<Config> for SupportControl {
    fn from(config: Config) -> Self {
        Self::from_config(config)
    }
}

impl From<Args> for SupportControl {
    fn from(args: Args) -> Self {
        Self::from_args(&args)
    }
}
