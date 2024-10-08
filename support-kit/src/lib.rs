mod args;
mod config;
mod logs;
mod network;
mod service;
mod structures;
mod support_control;
mod support_kit_error;
mod verbosity_level;

pub use args::*;
pub use config::Config;
pub use logs::*;
pub use network::NetworkConfig;
pub use service::*;
pub use structures::*;
pub use support_control::SupportControl;
pub use support_kit_error::SupportKitError;
pub use verbosity_level::VerbosityLevel;

type TracingTarget = Box<dyn tracing_subscriber::Layer<tracing_subscriber::Registry> + Send + Sync>;
type TracingTargets = Vec<TracingTarget>;

#[test]
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
