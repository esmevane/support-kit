use clap::{Parser, Subcommand};

use crate::{Config, NetworkConfig, ServiceArgs, ServiceCommand, VerbosityLevel};

#[derive(Clone, Debug, Parser)]
pub struct Args {
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    #[arg(short = 'H', long, global = true)]
    host: Option<String>,

    #[arg(short = 'P', long, global = true)]
    port: Option<i32>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Clone, Debug, Subcommand, PartialEq)]
#[clap(rename_all = "kebab-case")]
pub enum Commands {
    Service(ServiceArgs),
}

impl From<ServiceArgs> for Commands {
    fn from(args: ServiceArgs) -> Self {
        Commands::Service(args)
    }
}

impl From<ServiceCommand> for Commands {
    fn from(command: ServiceCommand) -> Self {
        ServiceArgs::from(command).into()
    }
}

impl Args {
    pub fn verbosity_level(&self) -> Option<VerbosityLevel> {
        let verbosity = self.verbose;
        VerbosityLevel::from_repr(verbosity as usize)
    }

    pub fn server(&self) -> Option<NetworkConfig> {
        match (self.host.clone(), self.port) {
            (None, None) => None,
            _ => Some(
                NetworkConfig::builder()
                    .maybe_host(self.host.clone())
                    .maybe_port(self.port)
                    .build()
                    .into(),
            ),
        }
    }

    pub fn config(&self) -> Config {
        Config::builder()
            .maybe_verbosity(self.verbosity_level())
            .maybe_server(self.server())
            .build()
    }
}

#[test]
fn setting_verbosity_with_args() -> Result<(), Box<dyn std::error::Error>> {
    let expectations = [
        ("app", VerbosityLevel::Off),
        ("app -v", VerbosityLevel::Error),
        ("app -vv", VerbosityLevel::Warn),
        ("app -vvv", VerbosityLevel::Info),
        ("app -vvvv", VerbosityLevel::Debug),
        ("app -vvvvv", VerbosityLevel::Trace),
    ];

    for (input, expected) in expectations.iter() {
        let args = Args::try_parse_from(input.split_whitespace())?;

        assert_eq!(
            args.config(),
            Config::builder().verbosity(*expected).build()
        );
    }
    Ok(())
}

#[test]
fn setting_server_with_args() -> Result<(), Box<dyn std::error::Error>> {
    let expectations = [
        ("app", NetworkConfig::default()),
        ("app -H localhost", NetworkConfig::from("localhost")),
        ("app -P 8080", NetworkConfig::builder().port(8080).build()),
        (
            "app -H localhost -P 8080",
            NetworkConfig::from(("localhost", 8080)),
        ),
    ];

    for (input, expected) in expectations.iter() {
        let args = Args::try_parse_from(input.split_whitespace())?;

        assert_eq!(
            args.config(),
            Config::builder().server(expected.clone()).build()
        );
    }
    Ok(())
}
