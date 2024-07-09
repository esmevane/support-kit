use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing_appender::rolling;
use tracing_subscriber::{
    fmt::{writer::MakeWriterExt, MakeWriter},
    Layer,
};

#[derive(Clone, Debug, Deserialize)]
pub struct Logging {
    pub loggers: Vec<Logger>,
    pub console: VerbosityDefinition,
}

impl Logging {
    pub fn build<L, S>(&self) -> LoggingDefinition<S>
    where
        L: tracing_subscriber::Layer<S>,
        S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    {
        let mut min_max_layers = Vec::new();
        let mut simple_layers = Vec::new();
        let mut guards = Vec::new();

        for logger in self.loggers.clone() {
            match logger.level() {
                VerbosityDefinition::MinMax(MinMax { min, max }) => {
                    let appender = logger.appender();
                    let (non_blocking, guard) = tracing_appender::non_blocking(appender);
                    let layer = tracing_subscriber::fmt::layer()
                        .json()
                        .with_writer(
                            non_blocking
                                .with_max_level(max.as_level_filter())
                                .with_min_level(min.as_level_filter()),
                        )
                        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::default());

                    min_max_layers.push(layer.boxed());
                    guards.push(guard);
                }
                VerbosityDefinition::Single(level) => {
                    let appender = logger.appender();
                    let (non_blocking, guard) = tracing_appender::non_blocking(appender);
                    let layer = tracing_subscriber::fmt::layer()
                        .json()
                        .with_writer(non_blocking)
                        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::default())
                        .with_filter(
                            tracing_subscriber::EnvFilter::from_default_env()
                                .add_directive(level.as_level_filter().into()),
                        );

                    simple_layers.push(layer.boxed());
                    guards.push(guard);
                }
            }
        }

        LoggingDefinition {
            min_max_layers,
            simple_layers,
            guards,
        }
    }
}

pub struct LoggingDefinition<S> {
    pub min_max_layers: Vec<Box<dyn Layer<S> + Send + Sync>>,
    pub simple_layers: Vec<Box<dyn Layer<S> + Send + Sync>>,
    pub guards: Vec<tracing_appender::non_blocking::WorkerGuard>,
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

    pub fn as_make_writer<Writer>(&self) -> Writer
    where
        Writer: for<'writer> MakeWriter<'writer>,
    {
        match self.level() {
            VerbosityDefinition::MinMax(MinMax { min, max }) => {
                let appender = self.appender();
                let (non_blocking, _) = tracing_appender::non_blocking(appender);

                non_blocking
                    .with_max_level(max.as_level_filter())
                    .with_min_level(min.as_level_filter())
            }
            VerbosityDefinition::Single(level) => {
                let appender = self.appender();
                let (non_blocking, guard) = tracing_appender::non_blocking(appender);

                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(level.as_level_filter().into())
            }
        }
        // W2: for<'writer> MakeWriter<'writer> + 'static,
    }

    pub fn setup_logging<
        T: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    >(
        &self,
    ) -> (
        Box<dyn Layer<T> + Send + 'static>,
        tracing_appender::non_blocking::WorkerGuard,
    ) {
        let appender = self.appender();

        match self.level() {
            VerbosityDefinition::MinMax(MinMax { min, max }) => {
                let (non_blocking, guard) = tracing_appender::non_blocking(appender);
                let layer = tracing_subscriber::fmt::layer()
                    .json()
                    .with_writer(
                        non_blocking
                            .with_max_level(max.as_level_filter())
                            .with_min_level(min.as_level_filter()),
                    )
                    .with_timer(tracing_subscriber::fmt::time::ChronoLocal::default());

                (layer.boxed(), guard)
            }
            VerbosityDefinition::Single(level) => {
                let (non_blocking, guard) = tracing_appender::non_blocking(appender);
                let layer = tracing_subscriber::fmt::layer()
                    .json()
                    .with_writer(non_blocking)
                    .with_timer(tracing_subscriber::fmt::time::ChronoLocal::default())
                    .with_filter(
                        tracing_subscriber::EnvFilter::from_default_env()
                            .add_directive(level.as_level_filter().into()),
                    );

                (layer.boxed(), guard)
            }
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
