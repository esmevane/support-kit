#[derive(Clone, Debug, Default, serde::Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum LogRotation {
    Daily,
    Hourly,
    PerMinute,
    #[default]
    Never,
}
