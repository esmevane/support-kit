use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Default, Debug, Deserialize)]
#[serde(default)]
pub struct Configuration {
    pub db: Option<Database>,
    pub storage: Option<Storage>,
    pub logging: Logging,
}

// impl Default for Configuration {
//     fn default() -> Self {
//         Self {
//             db: None,
//             storage: None,
//             ..Default::default()
//         }
//     }
// }

#[derive(Clone, Debug, Deserialize)]
pub struct Storage {
    pub path: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Database {
    host: String,
    port: u16,
    user: String,
    password: String,
    database: Option<String>,
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
            self.database.as_deref().unwrap_or("acore_world"),
        )
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Logging {
    pub loggers: Option<Vec<Logger>>,
    pub console: Verbosity,
}

impl Default for Logging {
    fn default() -> Self {
        let console = Verbosity::Single(Level {
            verbosity: VerbosityLevel::Info,
        });

        let loggers = vec![
            Logger::File(FileLogger::error_logger()),
            Logger::Rolling(RollingFileLogger::daily_info_logger()),
        ];

        Self {
            loggers: Some(loggers),
            console,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum Logger {
    File(FileLogger),
    Rolling(RollingFileLogger),
}

#[derive(Clone, Debug, Deserialize)]
pub struct FileLogger {
    pub path: PathBuf,
    pub level: Verbosity,
    pub name: String,
}

impl FileLogger {
    pub fn error_logger() -> Self {
        Self {
            path: PathBuf::from("logs"),
            level: Verbosity::MinMax(MinMax {
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
    pub level: Verbosity,
    pub name: String,
    pub rotation: Rotation,
}

impl RollingFileLogger {
    pub fn daily_info_logger() -> Self {
        Self {
            name: "out".to_string(),
            path: PathBuf::from("logs"),
            rotation: Rotation::Daily,
            level: Verbosity::Single(Level {
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
pub enum Verbosity {
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
