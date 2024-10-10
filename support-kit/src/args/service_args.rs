use clap::Parser;

use super::ServiceCommand;

#[derive(Clone, Debug, Default, Parser, PartialEq)]
#[clap(rename_all = "kebab-case")]
pub struct ServiceArgs {
    /// Control the service itself.
    #[clap(subcommand)]
    pub operation: Option<ServiceCommand>,
}

impl ServiceArgs {
    pub fn new(operation: Option<ServiceCommand>) -> Self {
        Self { operation }
    }
}

impl From<ServiceCommand> for ServiceArgs {
    fn from(command: ServiceCommand) -> Self {
        Self::new(Some(command))
    }
}

#[test]
fn operations() -> Result<(), Box<dyn std::error::Error>> {
    use crate::Args;

    use clap::Parser;
    use ServiceCommand::*;

    let expectations = [
        ("app service", None),
        ("app service install", Some(Install(Default::default()))),
        ("app service start", Some(Start)),
        ("app service stop", Some(Stop)),
        ("app service uninstall", Some(Uninstall)),
    ];

    for (input, expected) in expectations {
        let cli = Args::try_parse_from(input.split_whitespace())?;

        assert_eq!(cli.command, Some(ServiceArgs::new(expected).into()));
    }

    Ok(())
}

#[test]
fn setting_labels() -> Result<(), Box<dyn std::error::Error>> {
    use crate::Args;

    use clap::Parser;

    let expectations = [
        ("app", "support-kit"),
        ("app -n app-name", "app-name"),
        ("app --name app-name", "app-name"),
    ];

    for (input, expected) in expectations {
        let cli = Args::try_parse_from(input.split_whitespace())?;

        assert_eq!(cli.name.unwrap_or_default(), expected.into());
    }

    Ok(())
}

#[test]
fn setting_service_manager() -> Result<(), Box<dyn std::error::Error>> {
    use crate::Args;

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
        let cli = Args::try_parse_from(input.split_whitespace())?;

        assert_eq!(cli.service_manager, expected);
    }

    Ok(())
}

#[test]
fn setting_system_flag() -> Result<(), Box<dyn std::error::Error>> {
    use crate::Args;

    use clap::Parser;

    let expectations = [("app", false), ("app --system", true)];

    for (input, expected) in expectations {
        let cli = Args::try_parse_from(input.split_whitespace())?;

        assert_eq!(cli.system, expected);
    }

    Ok(())
}

#[test]
fn reading_cargo_env_for_defaults() -> Result<(), Box<dyn std::error::Error>> {
    use crate::Args;

    use clap::Parser;

    figment::Jail::expect_with(|jail| {
        jail.set_env("CARGO_PKG_NAME", "consumer-package");

        let expectations = [
            ("app", "consumer-package"),
            ("app -n app-name", "app-name"),
            ("app --name app-name", "app-name"),
        ];

        for (input, expected) in expectations {
            let cli = Args::try_parse_from(input.split_whitespace()).expect("failed to parse");

            assert_eq!(cli.name.unwrap_or_default(), expected.into());
        }

        Ok(())
    });

    Ok(())
}
