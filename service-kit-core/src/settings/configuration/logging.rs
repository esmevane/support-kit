use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing_appender::rolling;

#[derive(Clone, Debug, Deserialize)]
pub struct Logging {
    pub loggers: Vec<Logger>,
    pub console: VerbosityDefinition,
}

impl Default for Logging {
    fn default() -> Self {
        let console = VerbosityDefinition::Single(Level {
            verbosity: VerbosityLevel::Info,
        });

        let loggers = vec![
            Logger::File(FileLogger::error_logger()),
            Logger::Rolling(RollingFileLogger::daily_info_logger()),
        ];

        Self { loggers, console }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum Logger {
    File(FileLogger),
    Rolling(RollingFileLogger),
}

impl Logger {
    pub fn appender(&self) -> rolling::RollingFileAppender {
        match self {
            Self::File(_) => rolling::never(self.directory(), self.file_name_prefix()),
            Self::Rolling(logger) => match logger.rotation {
                Rotation::Daily => rolling::daily(self.directory(), self.file_name_prefix()),
                Rotation::Hourly => rolling::hourly(self.directory(), self.file_name_prefix()),
                Rotation::Minutely => rolling::minutely(self.directory(), self.file_name_prefix()),
                Rotation::Never => rolling::never(self.directory(), self.file_name_prefix()),
            },
        }
    }

    pub fn directory(&self) -> PathBuf {
        match self {
            Self::File(logger) => logger.path.clone(),
            Self::Rolling(logger) => logger.path.clone(),
        }
    }

    pub fn file_name_prefix(&self) -> String {
        format!(
            "{}.log",
            match self {
                Self::File(logger) => &logger.name,
                Self::Rolling(logger) => &logger.name,
            }
        )
    }

    pub fn level(&self) -> VerbosityDefinition {
        match self {
            Self::File(logger) => logger.level.clone(),
            Self::Rolling(logger) => logger.level.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct FileLogger {
    pub path: PathBuf,
    pub level: VerbosityDefinition,
    pub name: String,
}

impl FileLogger {
    pub fn error_logger() -> Self {
        Self {
            path: PathBuf::from("logs"),
            level: VerbosityDefinition::MinMax(MinMax {
                min: VerbosityLevel::Error,
                max: VerbosityLevel::Warn,
            }),
            name: "error".to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct RollingFileLogger {
    pub path: PathBuf,
    pub level: VerbosityDefinition,
    pub name: String,
    pub rotation: Rotation,
}

impl RollingFileLogger {
    pub fn daily_info_logger() -> Self {
        Self {
            name: "out".to_string(),
            path: PathBuf::from("logs"),
            rotation: Rotation::Daily,
            level: VerbosityDefinition::Single(Level {
                verbosity: VerbosityLevel::Info,
            }),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Rotation {
    Daily,
    Hourly,
    Minutely,
    Never,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum VerbosityDefinition {
    MinMax(MinMax),
    Single(Level),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MinMax {
    pub min: VerbosityLevel,
    pub max: VerbosityLevel,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Level {
    pub verbosity: VerbosityLevel,
}

impl Level {
    pub fn as_level_filter(&self) -> tracing::Level {
        self.verbosity.as_level_filter()
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VerbosityLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

impl VerbosityLevel {
    pub fn as_level_filter(&self) -> tracing::Level {
        match self {
            Self::Error => tracing::Level::ERROR,
            Self::Warn => tracing::Level::WARN,
            Self::Info => tracing::Level::INFO,
            Self::Debug => tracing::Level::DEBUG,
            Self::Trace => tracing::Level::TRACE,
        }
    }
}
