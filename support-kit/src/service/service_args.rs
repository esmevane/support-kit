use bon::Builder;
use clap::Parser;
use serde::{Deserialize, Serialize};
use service_manager::ServiceManagerKind;

use super::{ServiceCommand, ServiceConfig, ServiceName};

#[derive(Clone, Debug, Default, Deserialize, Parser, Serialize, PartialEq, Builder)]
#[clap(rename_all = "kebab-case")]
pub struct ServiceArgs {
    /// Control the service itself.
    #[clap(subcommand)]
    pub operation: Option<ServiceCommand>,
    /// The service label to use. Defaults to the binary name.
    #[clap(long, short, global = true)]
    pub name: Option<ServiceName>,
    /// The kind of service manager to use. Defaults to system native.
    #[clap(long, value_enum, global = true)]
    pub service_manager: Option<ServiceManagerKind>,
    /// Install system-wide. If not set, attempts to install for the current user.
    #[clap(long, global = true)]
    #[builder(default)]
    pub system: bool,
}

impl ServiceArgs {
    pub fn config(&self) -> ServiceConfig {
        ServiceConfig::builder()
            .maybe_name(self.name.clone())
            .maybe_service_manager(self.service_manager)
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

        assert_eq!(cli.name.unwrap_or_default(), expected.into());
    }

    Ok(())
}

#[test]
fn setting_service_manager() -> Result<(), Box<dyn std::error::Error>> {
    use clap::Parser;
    use service_manager::ServiceManagerKind::*;

    let expectations = [
        ("app", None),
        ("app --service-manager systemd", Some(Systemd)),
        ("app --service-manager winsw", Some(WinSw)),
        ("app --service-manager launchd", Some(Launchd)),
        ("app --service-manager openrc", Some(OpenRc)),
        ("app --service-manager rcd", Some(Rcd)),
        ("app --service-manager sc", Some(Sc)),
    ];

    for (input, expected) in expectations {
        let cli = ServiceArgs::try_parse_from(input.split_whitespace())?;

        assert_eq!(cli.service_manager, expected);
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

            assert_eq!(cli.name.unwrap_or_default(), expected.into());
        }

        Ok(())
    });

    Ok(())
}
