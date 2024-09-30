use tracing_appender::non_blocking::WorkerGuard;

use crate::TracingTarget;

use super::{LogRotation, LoggerConfig};

#[derive(Clone, Debug, Default, serde::Deserialize, PartialEq, bon::Builder)]
pub struct LogFileConfig {
    #[builder(into)]
    pub directory: std::path::PathBuf,
    #[builder(into)]
    pub name: String,
    #[builder(into)]
    pub rotation: Option<LogRotation>,
}

impl LogFileConfig {
    pub fn init_log_appender(&self, logger_config: &LoggerConfig) -> (TracingTarget, WorkerGuard) {
        use tracing_appender::rolling::{daily, hourly, minutely, never};
        use tracing_subscriber::{
            fmt::{time::ChronoLocal, writer::MakeWriterExt},
            Layer,
        };

        let directory = self.directory.clone();
        let file_name_prefix = format!("{}.log", &self.name);

        let (non_blocking, guard) =
            tracing_appender::non_blocking(match self.rotation.clone().unwrap_or_default() {
                LogRotation::Daily => daily(directory, file_name_prefix),
                LogRotation::Hourly => hourly(directory, file_name_prefix),
                LogRotation::PerMinute => minutely(directory, file_name_prefix),
                LogRotation::Never => never(directory, file_name_prefix),
            });

        let logger = tracing_subscriber::fmt::layer()
            .json()
            .with_writer(
                non_blocking
                    .with_min_level(logger_config.min_tracing_level())
                    .with_max_level(logger_config.max_tracing_level()),
            )
            .with_timer(ChronoLocal::default())
            .boxed();

        (logger, guard)
    }
}

impl<GivenPath, GivenName> From<(GivenPath, GivenName)> for LogFileConfig
where
    GivenPath: Into<std::path::PathBuf>,
    GivenName: Into<String>,
{
    fn from((directory, name): (GivenPath, GivenName)) -> Self {
        Self::builder().directory(directory).name(name).build()
    }
}

impl<GivenPath, GivenName, GivenRotation> From<(GivenPath, GivenName, GivenRotation)>
    for LogFileConfig
where
    GivenPath: Into<std::path::PathBuf>,
    GivenName: Into<String>,
    GivenRotation: Into<LogRotation>,
{
    fn from((directory, name, rotation): (GivenPath, GivenName, GivenRotation)) -> Self {
        Self::builder()
            .directory(directory)
            .name(name)
            .rotation(rotation)
            .build()
    }
}
