mod args;
mod color;
mod config;
mod containers;
mod deploy;
mod environment;
mod errors;
mod logs;
mod network;
mod service;
mod ssh;
mod structures;
mod support_control;
mod verbosity;

pub use args::*;
pub use color::Color;
pub use config::*;
pub use containers::*;
pub use deploy::*;
pub use environment::Environment;
pub use errors::*;
pub use logs::*;
pub use network::NetworkConfig;
pub use service::*;
pub use ssh::*;
pub use structures::*;
pub use support_control::SupportControl;
pub use verbosity::Verbosity;

type TracingTarget = Box<dyn tracing_subscriber::Layer<tracing_subscriber::Registry> + Send + Sync>;
type TracingTargets = Vec<TracingTarget>;

pub type Result<T> = std::result::Result<T, SupportKitError>;

#[cfg(test)]
mod tests {

    #[test]
    fn usage_as_a_library_consumer() -> Result<(), Box<dyn std::error::Error>> {
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
            (
                "app service install",
                Some(Commands::from(Install(Default::default()))),
            ),
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
}
