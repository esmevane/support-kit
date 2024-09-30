use super::{LogLevel, LogRotation, LogTarget, LoggerConfig, LoggerConfigOrPreset};

#[derive(Clone, Debug, serde::Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum LoggerPreset {
    Error,
    RollingInfo,
    RollingDebug,
    Stdout,
    Stderr,
}

impl From<LoggerPreset> for LoggerConfig {
    fn from(preset: LoggerPreset) -> Self {
        match preset {
            LoggerPreset::Error => error(),
            LoggerPreset::RollingInfo => rolling_info(),
            LoggerPreset::RollingDebug => rolling_debug(),
            LoggerPreset::Stdout => stdout(),
            LoggerPreset::Stderr => stderr(),
        }
    }
}

impl From<LoggerPreset> for LoggerConfigOrPreset {
    fn from(value: LoggerPreset) -> Self {
        Self::Preset(value)
    }
}

fn error() -> LoggerConfig {
    LoggerConfig::builder()
        .level(LogLevel::Error..LogLevel::Warn)
        .file(("logs", "app.error"))
        .build()
}

fn rolling_info() -> LoggerConfig {
    LoggerConfig::builder()
        .level(LogLevel::Info)
        .file(("logs", "app", LogRotation::Daily))
        .build()
}

fn rolling_debug() -> LoggerConfig {
    LoggerConfig::builder()
        .level(LogLevel::Error..LogLevel::Trace)
        .file(("logs", "app.debug", LogRotation::PerMinute))
        .build()
}

fn stdout() -> LoggerConfig {
    LoggerConfig::builder()
        .level(LogLevel::Info..LogLevel::Trace)
        .console(LogTarget::Stdout)
        .build()
}

fn stderr() -> LoggerConfig {
    LoggerConfig::builder()
        .level(LogLevel::Error..LogLevel::Warn)
        .console(LogTarget::Stderr)
        .build()
}

#[test]
fn logging_presets() -> Result<(), Box<dyn std::error::Error>> {
    use super::LoggingConfig;

    let config: LoggingConfig = serde_json::from_str(r#""stderr""#)?;

    assert_eq!(config, LoggingConfig::One(LoggerPreset::Stderr.into()));

    let config: LoggingConfig = serde_json::from_str(
        r#"
        ["stderr", "stdout", "error", "rolling-info", "rolling-debug"]
        "#,
    )?;

    assert_eq!(
        config,
        LoggingConfig::Many(vec![
            LoggerPreset::Stderr.into(),
            LoggerPreset::Stdout.into(),
            LoggerPreset::Error.into(),
            LoggerPreset::RollingInfo.into(),
            LoggerPreset::RollingDebug.into(),
        ]),
    );

    Ok(())
}

#[test]
fn stdout_preset() {
    let config: LoggerConfig = LoggerPreset::Stdout.into();

    let expectation = LoggerConfig::builder()
        .level(LogLevel::Info..LogLevel::Trace)
        .console(LogTarget::Stdout)
        .build();

    assert_eq!(config, expectation);
}

#[test]
fn stderr_preset() {
    let config: LoggerConfig = LoggerPreset::Stderr.into();

    let expectation = LoggerConfig::builder()
        .level(LogLevel::Error..LogLevel::Warn)
        .console(LogTarget::Stderr)
        .build();

    assert_eq!(config, expectation);
}

#[test]
fn error_preset() {
    let config: LoggerConfig = LoggerPreset::Error.into();
    let expectation = LoggerConfig::builder()
        .level(LogLevel::Error..LogLevel::Warn)
        .file(("logs", "app.error"))
        .build();

    assert_eq!(config, expectation);
}

#[test]
fn rolling_info_preset() {
    let config: LoggerConfig = LoggerPreset::RollingInfo.into();

    let expectation = LoggerConfig::builder()
        .level(LogLevel::Info)
        .file(("logs", "app", LogRotation::Daily))
        .build();

    assert_eq!(config, expectation);
}

#[test]
fn rolling_debug_preset() {
    let config: LoggerConfig = LoggerPreset::RollingDebug.into();

    let expectation = LoggerConfig::builder()
        .level(LogLevel::Error..LogLevel::Trace)
        .file(("logs", "app.debug", LogRotation::PerMinute))
        .build();

    assert_eq!(config, expectation);
}
