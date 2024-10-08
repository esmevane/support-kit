mod args;
mod config;
mod logs;
mod network;
mod service;
mod structures;
mod verbosity_level;

pub use args::*;
pub use config::Config;
pub use logs::*;
pub use network::NetworkConfig;
pub use service::*;
pub use structures::*;
pub use support_control::SupportControl;
pub use verbosity_level::VerbosityLevel;

type TracingTarget = Box<dyn tracing_subscriber::Layer<tracing_subscriber::Registry> + Send + Sync>;
type TracingTargets = Vec<TracingTarget>;

mod support_control {
    use crate::{Args, Config};

    #[derive(Default)]
    pub struct SupportControl {
        config: Config,
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
            self._guards = self.config.init_logging();
            self
        }

        pub fn execute(&self, args: Args) {
            match args.command {
                Some(command) => {
                    tracing::info!(
                        "Executing command: {command:#?}\nConfig: {config:#?}",
                        config = self.config
                    );

                    match command {
                        crate::Commands::Service(service_args) => {
                            let control = crate::ServiceControl::init(&self.config)
                                .expect("Failed to initialize service control");

                            match service_args.operation {
                                Some(operation) => {
                                    control
                                        .execute(operation)
                                        .expect("Failed to execute operation");
                                }
                                None => {
                                    tracing::info!(
                                        "No operation provided.\nConfig: {config:#?}",
                                        config = self.config
                                    );
                                }
                            }
                        }
                    }
                }
                None => {
                    tracing::info!(
                        "No command provided.\nConfig: {config:#?}",
                        config = self.config
                    );
                }
            }
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
}

// #[test]
fn todos() {
    let todos = include_str!("../todo.md");

    assert!(false, "{todos}");
}

#[test]
fn subcommand_dispatch() -> Result<(), Box<dyn std::error::Error>> {
    use clap::{Parser, Subcommand};

    #[derive(Parser)]
    struct LocalCli {
        #[clap(subcommand)]
        command: Option<LocalCommand>,

        #[clap(flatten)]
        support: crate::Args,
    }

    #[derive(Clone, Copy, Debug, Subcommand, PartialEq)]
    enum LocalCommand {
        Local,
    }

    let expectations = [
        ("app", None),
        ("app local", Some(LocalCommand::Local)),
        ("app service install", None),
        ("app service start", None),
        ("app service stop", None),
        ("app service uninstall", None),
    ];

    for (input, expected) in expectations {
        let cli = LocalCli::try_parse_from(input.split_whitespace())?;

        assert_eq!(cli.command, expected);
    }

    use crate::{Commands, ServiceCommand::*};
    let expectations = [
        ("app", None),
        ("app local", None),
        ("app service install", Some(Commands::from(Install))),
        ("app service start", Some(Commands::from(Start))),
        ("app service stop", Some(Commands::from(Stop))),
        ("app service uninstall", Some(Commands::from(Uninstall))),
    ];

    for (input, expected) in expectations {
        let cli = LocalCli::try_parse_from(input.split_whitespace())?;

        assert_eq!(cli.support.command, expected);
    }

    Ok(())
}
