use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing_appender::rolling;

use crate::settings::Settings;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Logging {
    pub loggers: Vec<Log>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Log {
    pub directory: PathBuf,
    pub max: Option<LogLevel>,
    pub min: Option<LogLevel>,
    pub name: String,
    pub rotation: Rotation,
}

impl Log {
    pub fn error_logger(settings: &Settings) -> Self {
        let app_name = &settings.app_name;
        Self {
            min: LogLevel::Error.into(),
            max: LogLevel::Warn.into(),
            name: format!("{app_name}.error"),
            directory: PathBuf::from("logs"),
            rotation: Rotation::Never,
        }
    }

    pub fn rolling_info_logger(settings: &Settings) -> Self {
        let app_name = &settings.app_name;
        Self {
            max: LogLevel::Info.into(),
            min: LogLevel::Info.into(),
            name: format!("{app_name}"),
            directory: PathBuf::from("logs"),
            rotation: Rotation::Daily,
        }
    }

    pub fn rolling_debug_logger(settings: &Settings) -> Self {
        let app_name = &settings.app_name;
        Self {
            max: LogLevel::Trace.into(),
            min: LogLevel::Error.into(),
            name: format!("{app_name}.debug"),
            directory: PathBuf::from("logs"),
            rotation: Rotation::Minutely,
        }
    }

    pub fn appender(&self) -> rolling::RollingFileAppender {
        let directory = self.directory.clone();
        let file_name_prefix = format!("{}.log", &self.name);

        match self.rotation {
            Rotation::Daily => rolling::daily(directory, file_name_prefix),
            Rotation::Hourly => rolling::hourly(directory, file_name_prefix),
            Rotation::Minutely => rolling::minutely(directory, file_name_prefix),
            Rotation::Never => rolling::never(directory, file_name_prefix),
        }
    }

    pub fn min_level(&self) -> tracing::Level {
        self.min.clone().unwrap_or_default().as_level_filter()
    }

    pub fn max_level(&self) -> tracing::Level {
        self.max.clone().unwrap_or_default().as_level_filter()
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

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    #[default]
    Info = 2,
    Debug = 3,
    Trace = 4,
}

impl LogLevel {
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
