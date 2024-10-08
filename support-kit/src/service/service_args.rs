use bon::Builder;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::Config;

use super::{ServiceCommand, ServiceConfig, ServiceControl, ServiceControlError, ServiceLabel};

#[derive(Clone, Debug, Default, Deserialize, Parser, Serialize, PartialEq, Builder)]
#[clap(rename_all = "kebab-case")]
pub struct ServiceArgs {
    /// Control the service itself.
    #[clap(subcommand)]
    pub operation: Option<ServiceCommand>,
    /// The service label to use. Defaults to the binary name.
    #[clap(long = "name", short = 'n')]
    pub label: Option<ServiceLabel>,
    /// Install system-wide. If not set, attempts to install for the current user.
    #[clap(long)]
    #[builder(default)]
    pub system: bool,
}

impl ServiceArgs {
    pub fn execute(&self, config: Config) -> Result<(), ServiceControlError> {
        let control = ServiceControl::init(&config)?;

        if let Some(operation) = &self.operation {
            match operation {
                ServiceCommand::Install => control.install(PathBuf::new(), vec![]),
                ServiceCommand::Start => control.start(),
                ServiceCommand::Stop => control.stop(),
                ServiceCommand::Uninstall => control.uninstall(),
            }?;
        }

        Ok(())
    }

    pub fn config(&self) -> ServiceConfig {
        ServiceConfig::builder()
            .maybe_label(self.label.clone())
            .system(self.system)
            .build()
    }
}

impl From<ServiceCommand> for ServiceArgs {
    fn from(command: ServiceCommand) -> Self {
        Self::builder().operation(command).build()
    }
}

#[test]
fn operations() -> Result<(), Box<dyn std::error::Error>> {
    use clap::Parser;
    use ServiceCommand::*;

    let expectations = [
        ("app", None),
        ("app install", Some(Install)),
        ("app start", Some(Start)),
        ("app stop", Some(Stop)),
        ("app uninstall", Some(Uninstall)),
    ];

    for (input, expected) in expectations {
        let cli = ServiceArgs::try_parse_from(input.split_whitespace())?;

        assert_eq!(cli.operation, expected);
    }

    Ok(())
}

#[test]
fn setting_labels() -> Result<(), Box<dyn std::error::Error>> {
    use clap::Parser;

    let expectations = [
        ("app", "support-kit"),
        ("app -n app-name", "app-name"),
        ("app --name app-name", "app-name"),
    ];

    for (input, expected) in expectations {
        let cli = ServiceArgs::try_parse_from(input.split_whitespace())?;

        assert_eq!(cli.label.unwrap_or_default(), expected.into());
    }

    Ok(())
}

#[test]
fn setting_system_flag() -> Result<(), Box<dyn std::error::Error>> {
    use clap::Parser;

    let expectations = [("app", false), ("app --system", true)];

    for (input, expected) in expectations {
        let cli = ServiceArgs::try_parse_from(input.split_whitespace())?;

        assert_eq!(cli.system, expected);
    }

    Ok(())
}

#[test]
fn reading_cargo_env_for_defaults() -> Result<(), Box<dyn std::error::Error>> {
    use clap::Parser;

    figment::Jail::expect_with(|jail| {
        jail.set_env("CARGO_PKG_NAME", "consumer-package");

        let expectations = [
            ("app", "consumer-package"),
            ("app -n app-name", "app-name"),
            ("app --name app-name", "app-name"),
        ];

        for (input, expected) in expectations {
            let cli =
                ServiceArgs::try_parse_from(input.split_whitespace()).expect("failed to parse");

            assert_eq!(cli.label.unwrap_or_default(), expected.into());
        }

        Ok(())
    });

    Ok(())
}
