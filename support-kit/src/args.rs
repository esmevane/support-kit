use clap::{Parser, Subcommand};

use crate::{
    Color, ConfigFile, Environment, ServiceCommand, ServiceConfig, ServiceManagerKind, ServiceName,
};

mod boilerplate_args;
mod deployment_args;
mod service_args;

pub use boilerplate_args::*;
pub use deployment_args::DeploymentArgs;
pub use service_args::ServiceArgs;

#[derive(Clone, Debug, Default, Parser)]
pub struct Args {
    /// The verbosity level to use. Defaults to off.
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    /// The host to bind to.
    #[arg(short = 'H', long, global = true)]
    pub host: Option<String>,

    /// The port to bind to.
    #[arg(short = 'P', long, global = true)]
    pub port: Option<i32>,

    /// The environment to use.
    #[arg(short, long, global = true)]
    pub environment: Option<Environment>,

    /// The service label to use. Defaults to the binary name.
    #[clap(long, short, global = true)]
    pub name: Option<ServiceName>,

    /// The kind of service manager to use. Defaults to system native.
    #[clap(long, value_enum, global = true)]
    pub service_manager: Option<ServiceManagerKind>,

    /// Install system-wide. If not set, attempts to install for the current user.
    #[clap(long, global = true)]
    pub system: bool,

    /// Color output.
    #[clap(long, global = true, default_value = "auto")]
    pub color: Color,

    /// The path to the configuration file.
    #[clap(long, short)]
    pub config_file: Option<ConfigFile>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Clone, Debug, Subcommand, PartialEq)]
#[clap(rename_all = "kebab-case")]
pub enum Commands {
    Service(ServiceArgs),
    Deploy(DeploymentArgs),
    Generate(BoilerplateArgs),
    Container(DeploymentArgs),
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
    pub fn config(&self) -> ConfigFile {
        let service_config = self.service();

        self.config_file
            .clone()
            .unwrap_or_else(|| service_config.name().into())
    }

    pub fn service(&self) -> ServiceConfig {
        ServiceConfig::builder()
            .maybe_name(self.name.clone())
            .maybe_service_manager(self.service_manager)
            .system(self.system)
            .build()
    }
}

#[test]
fn setting_verbosity_with_args() -> Result<(), Box<dyn std::error::Error>> {
    use crate::{Configuration, Verbosity};

    let expectations = [
        ("app", Verbosity::Off),
        ("app -v", Verbosity::Error),
        ("app -vv", Verbosity::Warn),
        ("app -vvv", Verbosity::Info),
        ("app -vvvv", Verbosity::Debug),
        ("app -vvvvv", Verbosity::Trace),
    ];

    for (input, expected) in expectations.iter() {
        let args = Args::try_parse_from(input.split_whitespace())?;

        assert_eq!(
            Configuration::from(args),
            Configuration::builder().verbosity(*expected).build()
        );
    }
    Ok(())
}

#[test]
fn config_file() -> Result<(), Box<dyn std::error::Error>> {
    let expectations = [
        ("app", "support-kit"),
        ("app --config-file custom.config", "custom.config"),
    ];

    for (input, expected) in expectations {
        let args = Args::try_parse_from(input.split_whitespace())?;

        assert_eq!(args.config(), expected.into());
    }

    figment::Jail::expect_with(|jail| {
        jail.set_env("CARGO_PKG_NAME", "custom-package");

        let args = Args::try_parse_from("app".split_whitespace()).unwrap();

        assert_eq!(args.config(), "custom-package".into());

        Ok(())
    });

    Ok(())
}

#[test]
fn setting_environment_with_args() -> Result<(), Box<dyn std::error::Error>> {
    use crate::Configuration;

    let expectations = [
        ("app", None),
        (
            "app --environment development",
            Some(Environment::Development),
        ),
        (
            "app --environment production",
            Some(Environment::Production),
        ),
        ("app --environment test", Some(Environment::Test)),
    ];

    for (input, expected) in expectations {
        let args = Args::try_parse_from(input.split_whitespace())?;

        assert_eq!(
            Configuration::from(args),
            Configuration::builder().maybe_environment(expected).build()
        );
    }
    Ok(())
}

#[test]
fn setting_color_with_args() -> Result<(), Box<dyn std::error::Error>> {
    use crate::Configuration;

    let expectations = [
        ("app", Color::Auto),
        ("app --color always", Color::Always),
        ("app --color never", Color::Never),
        ("app --color auto", Color::Auto),
    ];

    for (input, expected) in expectations {
        let args = Args::try_parse_from(input.split_whitespace())?;

        assert_eq!(
            Configuration::from(args),
            Configuration::builder().color(expected).build()
        );
    }
    Ok(())
}

#[test]
fn setting_server_with_args() -> Result<(), Box<dyn std::error::Error>> {
    use crate::{Configuration, NetworkConfig};
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
            Configuration::from(args),
            Configuration::builder().server(expected.clone()).build()
        );
    }
    Ok(())
}
