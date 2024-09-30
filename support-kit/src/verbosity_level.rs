#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    serde::Deserialize,
    strum::FromRepr,
    strum::EnumString,
    strum::Display,
)]
#[serde(rename_all = "kebab-case")]
pub enum VerbosityLevel {
    #[default]
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
