use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LogRotation {
    Daily,
    Hourly,
    Minutely,
    Never,
}
