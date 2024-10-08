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
