pub use service_manager::ServiceManagerKind;

mod service_command;
mod service_config;
mod service_control;
mod service_name;

pub use service_command::ServiceCommand;
pub use service_config::ServiceConfig;
pub use service_control::ServiceControl;
pub use service_name::ServiceName;

#[test]
fn building_service_config_from_cli_args() -> Result<(), Box<dyn std::error::Error>> {
    use crate::Args;
    use crate::Configuration;
    use clap::Parser;

    let expectations = [
        ("app", ServiceConfig::default()),
        (
            "app --system",
            ServiceConfig::builder().system(true).build(),
        ),
        (
            "app --name app-name",
            ServiceConfig::builder().name("app-name").build(),
        ),
        (
            "app --name app-name --system",
            ServiceConfig::builder()
                .name("app-name")
                .system(true)
                .build(),
        ),
        (
            "app --system --name app-name --service-manager systemd",
            ServiceConfig::builder()
                .name("app-name")
                .system(true)
                .service_manager(service_manager::ServiceManagerKind::Systemd)
                .build(),
        ),
    ];

    for (input, expected) in expectations {
        let cli = Args::try_parse_from(input.split_whitespace())?;

        assert_eq!(Configuration::from(cli).service, expected.into());
    }

    Ok(())
}
