mod service_args;
mod service_command;
mod service_config;
mod service_control;
mod service_control_error;

pub use service_args::ServiceArgs;
pub use service_command::ServiceCommand;
pub use service_config::ServiceConfig;
pub use service_control::ServiceControl;
pub use service_control_error::ServiceControlError;

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

    for (input, expected) in expectations.iter() {
        let cli = ServiceArgs::try_parse_from(input.split_whitespace())?;

        assert_eq!(cli.operation, *expected);
    }

    let expectations = [
        ("app", "support-kit"),
        ("app -n app-name", "app-name"),
        ("app --name app-name", "app-name"),
    ];

    for (input, expected) in expectations.iter() {
        let cli = ServiceArgs::try_parse_from(input.split_whitespace())?;

        assert_eq!(cli.label, (*expected).into());
    }

    Ok(())
}
