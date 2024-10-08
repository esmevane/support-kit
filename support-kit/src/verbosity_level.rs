use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Serialize,
    Deserialize,
    strum::FromRepr,
    strum::EnumString,
    strum::Display,
)]
#[serde(rename_all = "kebab-case")]
pub enum VerbosityLevel {
    #[default]
    #[strum(serialize = "")]
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
