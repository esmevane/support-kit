use serde::Deserialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub db: Option<Database>,
    pub storage: Option<Storage>,
    pub logging: Option<Logging>,
}

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

#[derive(Clone, Debug, Deserialize)]
pub struct RollingFileLogger {
    pub path: PathBuf,
    pub level: Verbosity,
    pub name: String,
    pub rotation: Rotation,
}

#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Rotation {
    #[default]
    Daily,
    Hourly,
    Minutely,
    Never,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum Verbosity {
    MinMax(MinMax),
    Single(Level),
}

#[derive(Clone, Debug, Deserialize)]
pub struct MinMax {
    pub min: VerbosityLevel,
    pub max: VerbosityLevel,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Level {
    pub verbosity: VerbosityLevel,
}

#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VerbosityLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}
