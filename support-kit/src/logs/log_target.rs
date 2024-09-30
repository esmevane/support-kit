use crate::TracingTarget;

use super::LoggerConfig;

#[derive(Clone, Debug, serde::Deserialize, PartialEq)]
pub enum LogTarget {
    Stdout,
    Stderr,
}

impl LogTarget {
    pub fn init_console_logger(&self, logger_config: &LoggerConfig) -> TracingTarget {
        use tracing_subscriber::fmt::writer::MakeWriterExt;
        use tracing_subscriber::EnvFilter;
        use tracing_subscriber::Layer;

        let env_filter = EnvFilter::try_from_default_env().unwrap_or_default();

        match self {
            LogTarget::Stderr => tracing_subscriber::fmt::layer()
                .with_writer(
                    std::io::stderr
                        .with_max_level(logger_config.max_tracing_level())
                        .with_min_level(logger_config.min_tracing_level()),
                )
                .with_filter(env_filter)
                .boxed(),
            LogTarget::Stdout => tracing_subscriber::fmt::layer()
                .with_writer(
                    std::io::stdout
                        .with_max_level(logger_config.max_tracing_level())
                        .with_min_level(logger_config.min_tracing_level()),
                )
                .with_filter(env_filter)
                .boxed(),
        }
    }
}
