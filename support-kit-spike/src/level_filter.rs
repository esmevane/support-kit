use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use crate::Level;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum LevelFilter {
    #[default]
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<Option<Level>> for LevelFilter {
    fn from(value: Option<Level>) -> Self {
        match value {
            None => LevelFilter::Off,
            Some(Level::Error) => LevelFilter::Error,
            Some(Level::Warn) => LevelFilter::Warn,
            Some(Level::Info) => LevelFilter::Info,
            Some(Level::Debug) => LevelFilter::Debug,
            Some(Level::Trace) => LevelFilter::Trace,
        }
    }
}
