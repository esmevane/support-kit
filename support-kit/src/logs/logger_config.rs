use crate::Config;

use super::{LogFileConfig, LogLevel, LogLevelConfig, LogTarget, LoggerConfigOrPreset, Logging};

#[derive(Clone, Debug, Default, serde::Deserialize, PartialEq, bon::Builder)]
#[serde(rename_all = "kebab-case")]
pub struct LoggerConfig {
    console: Option<LogTarget>,
    #[serde(flatten)]
    #[builder(into)]
    file: Option<LogFileConfig>,
    #[builder(into)]
    level: LogLevelConfig,
}

impl LoggerConfig {
    pub fn level_range(&self) -> std::ops::Range<LogLevel> {
        self.level.min_level()..self.level.max_level()
    }

    pub fn min_tracing_level(&self) -> tracing::Level {
        self.level.min_level().tracing_level()
    }

    pub fn max_tracing_level(&self) -> tracing::Level {
        self.level.max_level().tracing_level()
    }

    pub fn with_console_target(mut self, console: LogTarget) -> Self {
        self.console = Some(console);
        self
    }

    pub fn initialize(&self, config: &Config, logging: &mut Logging) {
        match &self.file {
            Some(file_config) => {
                let (logger, guard) = file_config.init_log_appender(&self);

                logging.loggers.push(logger);
                logging.guards.push(guard);
            }
            _ => {}
        }

        match &self.console {
            Some(console_target) => {
                let logger = console_target.init_console_logger(config, &self);

                logging.loggers.push(logger);
            }
            _ => {}
        }
    }
}

impl From<LoggerConfig> for LoggerConfigOrPreset {
    fn from(logger_config: LoggerConfig) -> Self {
        Self::Config(logger_config)
    }
}

impl From<LoggerConfigOrPreset> for LoggerConfig {
    fn from(config_or_preset: LoggerConfigOrPreset) -> Self {
        match config_or_preset {
            LoggerConfigOrPreset::Config(logger_config) => logger_config.into(),
            LoggerConfigOrPreset::Preset(logging_preset) => logging_preset.into(),
        }
    }
}

#[test]
fn single_logger_level_notation() -> Result<(), Box<dyn std::error::Error>> {
    use super::LogLevel;

    let config: LoggerConfig = serde_json::from_str(
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
        LoggerConfig::builder()
            .level(LogLevel::Warn)
            .file(("logs", "app.error"))
            .build()
    );

    Ok(())
}

#[test]
fn min_max_level_notation() -> Result<(), Box<dyn std::error::Error>> {
    use super::{LogLevel, LogRotation};

    let config: LoggerConfig = serde_json::from_str(
        r#"
        {
            "directory": "logs",
            "level": { "min": "info", "max": "warn" },
            "name": "app",
            "rotation": "daily"
        }
        "#,
    )?;

    assert_eq!(
        config,
        LoggerConfig::builder()
            .level(LogLevel::Info..LogLevel::Warn)
            .file(("logs", "app", LogRotation::Daily))
            .build()
    );

    Ok(())
}

#[test]
fn per_minute_rotation() -> Result<(), Box<dyn std::error::Error>> {
    use super::{LogLevel, LogRotation};

    let config: LoggerConfig = serde_json::from_str(
        r#"
        {
            "directory": "logs",
            "level": { "min": "trace", "max": "info" },
            "name": "app.debug",
            "rotation": "per-minute"
        }
        "#,
    )?;

    assert_eq!(
        config,
        LoggerConfig::builder()
            .level(LogLevel::Trace..LogLevel::Info)
            .file(("logs", "app.debug", LogRotation::PerMinute))
            .build()
    );

    Ok(())
}

#[test]
fn determining_log_levels() -> Result<(), Box<dyn std::error::Error>> {
    use super::{LogLevel, LoggerConfig};

    let error_logger: LoggerConfig = serde_json::from_str(
        r#"
        {
            "directory": "logs",
            "level": { "min": "info", "max": "warn" },
            "name": "app",
            "rotation": "daily"
        }
        "#,
    )?;

    assert_eq!(error_logger.level_range(), LogLevel::Info..LogLevel::Warn);

    let debug_logger: LoggerConfig = serde_json::from_str(
        r#"
        {
            "directory": "logs",
            "level": { "min": "trace", "max": "info" },
            "name": "app.debug",
            "rotation": "per-minute"
        }
        "#,
    )?;

    assert_eq!(debug_logger.level_range(), LogLevel::Trace..LogLevel::Info);

    Ok(())
}

#[test]
fn constructing_loggers() -> Result<(), Box<dyn std::error::Error>> {
    use super::{LogLevel, LogRotation, LoggerConfig, LoggingConfig};

    let config: LoggingConfig = serde_json::from_str(
        r#"
        {
            "directory": "logs",
            "level": "warn",
            "name": "app.debug",
            "rotation": "per-minute"
        }
        "#,
    )?;

    let loggers = config.loggers();

    assert_eq!(loggers.len(), 1);
    assert_eq!(
        loggers,
        vec![LoggerConfig::builder()
            .level(LogLevel::Warn)
            .file(("logs", "app.debug", LogRotation::PerMinute))
            .build()]
    );

    Ok(())
}

#[test]
fn constructing_loggers_from_logger_config() -> Result<(), Box<dyn std::error::Error>> {
    use super::{LogLevel, LoggerConfig};

    let config: LoggerConfig = serde_json::from_str(
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
        LoggerConfig::builder()
            .level(LogLevel::Warn)
            .file(("logs", "app.error"))
            .build()
    );

    Ok(())
}
