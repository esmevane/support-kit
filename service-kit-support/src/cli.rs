use clap::Parser;
use clap_verbosity_flag::Verbosity;
use serde::Serialize;

use crate::settings::{Color, Environment, Service};

/// A CLI application that helps do non-standard AzerothCore db tasks
#[derive(Clone, Debug, Parser, Serialize)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,

    #[clap(flatten)]
    pub global: GlobalOpts,
}

#[derive(Clone, Debug, Parser, Serialize)]
#[serde(rename_all = "kebab-case")]
#[clap(rename_all = "kebab-case")]
pub enum Command {
    /// Bootstrap all systems and configuration and print a debug report.
    Debug,
    /// Start the TUI interface.
    Tui,
    /// Start the server
    Server,
    /// Manage services
    Service(Service),
    /// Print the version of the application.
    Version,
}

#[derive(Clone, Debug, Parser, Serialize)]
pub struct GlobalOpts {
    /// Enable or disable colored output.
    #[clap(long, value_enum, global = true, default_value = "auto")]
    pub color: Color,
    /// The path to the configuration root.
    #[clap(short, long)]
    pub config: Option<String>,
    /// What environment to run the program in.
    #[clap(short, long, default_value = "development")]
    pub environment: Environment,
    /// Enable verbose output.
    #[command(flatten)]
    #[serde(skip_serializing)]
    pub verbosity: Verbosity,
}
