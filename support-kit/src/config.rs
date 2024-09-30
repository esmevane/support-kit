use super::{Logging, LoggingConfig};

#[derive(Clone, Debug, Default, serde::Deserialize, PartialEq, bon::Builder)]
pub struct Config {
    #[builder(into)]
    logging: LoggingConfig,
}

impl Config {
    pub fn init_logging(&self) -> Vec<tracing_appender::non_blocking::WorkerGuard> {
        Logging::initialize(self.logging.clone())
    }
}

#[test]
fn root_config_notation() -> Result<(), Box<dyn std::error::Error>> {
    use super::{LogLevel, LogRotation, LoggerConfig};

    let config: Config = serde_json::from_str(
        r#"
        {
            "logging": [
                {
                    "directory": "logs",
                    "level": "warn",
                    "name": "app.error"
                },
                {
                    "directory": "logs",
                    "level": { "min": "info", "max": "warn" },
                    "name": "app",
                    "rotation": "daily"
                },
                {
                    "directory": "logs",
                    "level": { "min": "trace", "max": "info" },
                    "name": "app.debug",
                    "rotation": "per-minute"
                }
            ]
        }
        "#,
    )?;

    assert_eq!(
        config,
        Config::builder()
            .logging(bon::vec![
                LoggerConfig::builder()
                    .level(LogLevel::Warn)
                    .file(("logs", "app.error"))
                    .build(),
                LoggerConfig::builder()
                    .level(LogLevel::Info..LogLevel::Warn)
                    .file(("logs", "app", LogRotation::Daily))
                    .build(),
                LoggerConfig::builder()
                    .level(LogLevel::Trace..LogLevel::Info)
                    .file(("logs", "app.debug", LogRotation::PerMinute))
                    .build(),
            ])
            .build()
    );

    Ok(())
}
