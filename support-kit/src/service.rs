mod service_args;
mod service_command;
mod service_config;
mod service_control;
mod service_control_error;
mod service_label;

pub use service_args::ServiceArgs;
pub use service_command::ServiceCommand;
pub use service_config::ServiceConfig;
pub use service_control::ServiceControl;
pub use service_control_error::ServiceControlError;
pub use service_label::ServiceLabel;

#[test]
fn service_args() -> Result<(), Box<dyn std::error::Error>> {
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

    let expectations = [
        ("app", "support-kit"),
        ("app -n app-name", "app-name"),
        ("app --name app-name", "app-name"),
    ];

    for (input, expected) in expectations {
        let cli = ServiceArgs::try_parse_from(input.split_whitespace())?;

        assert_eq!(cli.label.unwrap_or_default(), expected.into());
    }

    figment::Jail::expect_with(|jail| {
        jail.set_env("CARGO_PKG_NAME", "consumer-package");

        assert_eq!(service_label::get_runtime_name(), "consumer-package");

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

    let expectations = [("app", false), ("app --system", true)];

    for (input, expected) in expectations {
        let cli = ServiceArgs::try_parse_from(input.split_whitespace())?;

        assert_eq!(cli.system, expected);
    }

    Ok(())
}

#[test]
fn service_config() -> Result<(), Box<dyn std::error::Error>> {
    // let expectations = [
    //     ("app", "support-kit"),
    //     ("app -n app-name", "app-name"),
    //     ("app --name app-name", "app-name"),
    // ];

    // for (input, expected) in expectations {
    //     let cli = ServiceArgs::try_parse_from(input.split_whitespace())?;

    //     assert_eq!(cli.label, expected.into());
    // }

    Ok(())
}
