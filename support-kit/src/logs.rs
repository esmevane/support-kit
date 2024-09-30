mod log_file_config;
mod log_level;
mod log_level_config;
mod log_rotation;
mod log_target;
mod logger_config;
mod logger_preset;
mod logging;

pub use log_file_config::LogFileConfig;
pub use log_level::LogLevel;
pub use log_level_config::LogLevelConfig;
pub use log_rotation::LogRotation;
pub use log_target::LogTarget;
pub use logger_config::LoggerConfig;
pub use logger_preset::LoggerPreset;
pub use logging::Logging;

use crate::{ConfigOrPreset, OneOrMany};

pub type LoggerConfigOrPreset = ConfigOrPreset<LoggerConfig, LoggerPreset>;
pub type LoggingConfig = OneOrMany<LoggerConfigOrPreset>;

impl LoggingConfig {
    pub fn loggers(&self) -> Vec<LoggerConfig> {
        match self {
            Self::Many(config_or_preset) => config_or_preset
                .into_iter()
                .cloned()
                .map(Into::into)
                .collect(),
            Self::One(config_or_preset) => vec![config_or_preset.clone().into()],
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self::Many(vec![
            ConfigOrPreset::Preset(LoggerPreset::Stdout),
            ConfigOrPreset::Preset(LoggerPreset::Stderr),
        ])
    }
}

#[test]
fn single_logger_notation() -> Result<(), Box<dyn std::error::Error>> {
    use super::{LogLevel, LoggerConfig};

    let config: LoggingConfig = serde_json::from_str(
        r#"
        {
            "directory": "logs",
            "level": "warn",
            "name": "app.error"
        }
        "#,
    )?;

    assert_eq!(
        config,
        OneOrMany::One(
            LoggerConfig::builder()
                .level(LogLevel::Warn)
                .file(("logs", "app.error"))
                .build()
                .into()
        ),
    );

    Ok(())
}

#[test]
fn multiple_loggers_notation() -> Result<(), Box<dyn std::error::Error>> {
    use super::{LogLevel, LogRotation, LoggerConfig};

    let config: LoggingConfig = serde_json::from_str(
        r#"
        [{
            "directory": "logs",
            "level": { "min": "info", "max": "warn" },
            "name": "app",
            "rotation": "daily"
        }]
        "#,
    )?;

    assert_eq!(
        config,
        vec![LoggerConfig::builder()
            .level(LogLevel::Info..LogLevel::Warn)
            .file(("logs", "app", LogRotation::Daily))
            .build()
            .into()]
        .into(),
    );

    Ok(())
}
