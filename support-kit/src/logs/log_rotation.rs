use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum LogRotation {
    Daily,
    Hourly,
    PerMinute,
    #[default]
    Never,
}
