use clap::{Parser, Subcommand};

use crate::{
    Color, Config, NetworkConfig, ServiceCommand, ServiceConfig, ServiceManagerKind, ServiceName,
    VerbosityLevel,
};

mod service_args;

pub use service_args::ServiceArgs;

#[derive(Clone, Debug, Parser)]
pub struct Args {
    /// The verbosity level to use. Defaults to off.
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    /// The host to bind to.
    #[arg(short = 'H', long, global = true)]
    host: Option<String>,

    /// The port to bind to.
    #[arg(short = 'P', long, global = true)]
    port: Option<i32>,

    /// The service label to use. Defaults to the binary name.
    #[clap(long, short, global = true)]
    name: Option<ServiceName>,

    /// The kind of service manager to use. Defaults to system native.
    #[clap(long, value_enum, global = true)]
    service_manager: Option<ServiceManagerKind>,

    /// Install system-wide. If not set, attempts to install for the current user.
    #[clap(long, global = true)]
    system: bool,

    /// Color output.
    #[clap(long, global = true, default_value = "auto")]
    color: Color,

    /// The path to the configuration file.
    #[clap(long, short)]
    config_file: Option<String>,

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
    pub fn config(&self) -> String {
        let service_config = self.service();
        let name = service_config.name();
        let file_name = format!("{name}.config");

        self.config_file.clone().unwrap_or_else(|| file_name)
    }

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

    pub fn service(&self) -> ServiceConfig {
        ServiceConfig::builder()
            .maybe_name(self.name.clone())
            .maybe_service_manager(self.service_manager)
            .system(self.system)
            .build()
    }

    pub fn build_config(&self) -> Config {
        Config::builder()
            .maybe_verbosity(self.verbosity_level())
            .maybe_server(self.server())
            .color(self.color)
            .service(self.service())
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
            args.build_config(),
            Config::builder().verbosity(*expected).build()
        );
    }
    Ok(())
}

#[test]
fn default_config_file() -> Result<(), Box<dyn std::error::Error>> {
    let expectations = [
        ("app", "support-kit.config"),
        ("app --config-file custom.config", "custom.config"),
    ];

    for (input, expected) in expectations {
        let args = Args::try_parse_from(input.split_whitespace())?;

        assert_eq!(args.config(), expected.to_string());
    }

    Ok(())
}

#[test]
fn setting_color_with_args() -> Result<(), Box<dyn std::error::Error>> {
    let expectations = [
        ("app", Color::Auto),
        ("app --color always", Color::Always),
        ("app --color never", Color::Never),
        ("app --color auto", Color::Auto),
    ];

    for (input, expected) in expectations {
        let args = Args::try_parse_from(input.split_whitespace())?;

        assert_eq!(
            args.build_config(),
            Config::builder().color(expected).build()
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

    for (input, expected) in expectations {
        let args = Args::try_parse_from(input.split_whitespace())?;

        assert_eq!(
            args.build_config(),
            Config::builder().server(expected.clone()).build()
        );
    }
    Ok(())
}
