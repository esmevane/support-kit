use clap::Parser;

use crate::{Config, NetworkConfig, VerbosityLevel};

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[arg(short = 'H', long)]
    host: Option<String>,

    #[arg(short = 'P', long)]
    port: Option<i32>,
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

    pub fn as_config(self) -> Config {
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
            args.as_config(),
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
            args.as_config(),
            Config::builder().server(expected.clone()).build()
        );
    }
    Ok(())
}
