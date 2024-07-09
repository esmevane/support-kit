use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing_appender::rolling;

use crate::APP_NAME;

#[derive(Clone, Default, Debug, Deserialize)]
#[serde(default)]
pub struct Configuration {
    pub db: Database,
    pub storage: Storage,
    pub logging: Logging,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Storage {
    pub path: PathBuf,
}

impl Default for Storage {
    fn default() -> Self {
        // use directories to get a default data directory in user's config path
        let path = match dirs::config_local_dir() {
            Some(mut path) => {
                path.push(APP_NAME.to_lowercase());
                path.push("storage.db");
                path
            }
            None => {
                // otherwise we start in a temp directory
                std::env::temp_dir()
                    .join(APP_NAME.to_lowercase())
                    .join("storage.db")
            }
        };

        tracing::debug!("Using storage path: {}", path.display());

        // ensure the file and path exist
        if let Some(parent) = path.parent() {
            tracing::debug!("Ensuring storage path exists: {:?}", parent);
            std::fs::create_dir_all(parent).expect("Unable to create storage directory");
        }

        if std::fs::metadata(&path).is_err() {
            tracing::debug!("Ensuring storage file exists: {:?}", path);
            std::fs::File::create(&path).expect("Unable to create storage file");
        }

        Self { path }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Database {
    host: String,
    port: u16,
    user: String,
    password: String,
    database: Option<String>,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 3306,
            user: "root".to_string(),
            password: "password".to_string(),
            database: None,
        }
    }
}

impl std::fmt::Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "mysql://{}:{}@{}:{}/{}",
            self.user,
            self.password,
            self.host,
            self.port,
            self.database.as_deref().unwrap_or(&APP_NAME),
        )
    }
}

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
