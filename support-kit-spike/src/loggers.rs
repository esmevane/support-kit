use serde::Deserialize;
use std::path::PathBuf;
use tracing_appender::rolling;

use crate::{Config as Settings, Level as LogLevel, LogRotation as Rotation};

#[derive(Clone, Debug, Deserialize)]
pub struct Loggers {
    pub directory: PathBuf,
    pub max: LogLevel,
    pub min: LogLevel,
    pub name: String,
    pub rotation: Rotation,
}

impl Loggers {
    pub fn error(settings: &Settings) -> Self {
        let app_name = settings.name.name();
        Self {
            min: LogLevel::Error,
            max: LogLevel::Warn,
            name: format!("{app_name}.error"),
            directory: PathBuf::from("logs"),
            rotation: Rotation::Never,
        }
    }

    pub fn rolling_info(settings: &Settings) -> Self {
        let app_name = settings.name.name();
        Self {
            max: LogLevel::Info,
            min: LogLevel::Info,
            name: format!("{app_name}"),
            directory: PathBuf::from("logs"),
            rotation: Rotation::Daily,
        }
    }

    pub fn rolling_debug(settings: &Settings) -> Self {
        let app_name = settings.name.name();
        Self {
            max: LogLevel::Trace,
            min: LogLevel::Error,
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
        self.min.clone().as_level_filter()
    }

    pub fn max_level(&self) -> tracing::Level {
        self.max.clone().as_level_filter()
    }
}
