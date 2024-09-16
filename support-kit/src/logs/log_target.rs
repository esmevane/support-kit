use serde::{Deserialize, Serialize};

use crate::{Config, TracingTarget};

use super::LoggerConfig;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum LogTarget {
    Stdout,
    Stderr,
}

impl LogTarget {
    pub fn init_console_logger(
        &self,
        config: &Config,
        logger_config: &LoggerConfig,
    ) -> TracingTarget {
        use tracing_subscriber::fmt::writer::MakeWriterExt;
        use tracing_subscriber::Layer;

        match self {
            LogTarget::Stderr => tracing_subscriber::fmt::layer()
                .with_writer(
                    std::io::stderr
                        .with_max_level(logger_config.max_tracing_level())
                        .with_min_level(logger_config.min_tracing_level()),
                )
                .with_filter(config.env_filter())
                .boxed(),
            LogTarget::Stdout => tracing_subscriber::fmt::layer()
                .with_writer(
                    std::io::stdout
                        .with_max_level(logger_config.max_tracing_level())
                        .with_min_level(logger_config.min_tracing_level()),
                )
                .with_filter(config.env_filter())
                .boxed(),
        }
    }
}
