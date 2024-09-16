use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Level {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl Level {
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

impl std::fmt::Display for Level {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level = match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        };

        write!(formatter, "{:?}", level)
    }
}
