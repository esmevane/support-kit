mod cli;
mod color;
mod config;
mod env_source_name;
mod environment;
mod level;
mod level_filter;
mod log_rotation;
mod loggers;
mod logging;
mod source;
mod source_name;
mod telemetry;

pub use cli::Cli;
pub use color::Color;
pub use config::Config;
pub use env_source_name::EnvSourceName;
pub use environment::Environment;
pub use environment::EnvironmentEnum;
pub use level::Level;
pub use level_filter::LevelFilter;
pub use log_rotation::LogRotation;
pub use loggers::Loggers;
pub use logging::Logging;
pub use source::Source;
pub use source_name::SourceName;

pub fn load_config(args: impl AsRef<str>) -> Result<Config, Box<dyn std::error::Error>> {
    use clap::Parser;

    let cli = Cli::try_parse_from(args.as_ref().split_whitespace())?;
    let config = Config::try_from_cli(cli)?;
    let _ = telemetry::init(&config);

    Ok(config)
}

#[test]
fn test_load_config() -> Result<(), Box<dyn std::error::Error>> {
    // defaults
    {
        let config = load_config("app-name")?;
        assert_eq!(config.name, SourceName::try_new("support-kit")?);
        assert_eq!(config.environment, "development".parse()?);
        assert_eq!(config.color, Color::Auto);
        assert_eq!(config.logging, Logging::default());
    }

    Ok(())
}

#[test]
fn test_load_config_with_cli() -> Result<(), Box<dyn std::error::Error>> {
    // defaults
    {
        let config = load_config("app-name --environment test")?;
        assert_eq!(config.name, SourceName::try_new("support-kit")?);
        assert_eq!(config.environment, "test".parse()?);
        assert_eq!(config.color, Color::Auto);
        assert_eq!(config.logging, Logging::default());
    }

    Ok(())
}
