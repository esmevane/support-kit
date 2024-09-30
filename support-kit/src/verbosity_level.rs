#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    serde::Deserialize,
    strum::FromRepr,
    strum::EnumString,
    strum::Display,
)]
#[serde(rename_all = "kebab-case")]
pub enum VerbosityLevel {
    #[strum(serialize = "")]
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
