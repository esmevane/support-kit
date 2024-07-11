use clap::ValueEnum;
use config::ValueKind;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, Deserialize, strum::Display, strum::EnumIs, ValueEnum, Serialize,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Environment {
    #[default]
    Development,
    Production,
}

impl From<Environment> for ValueKind {
    fn from(val: Environment) -> Self {
        val.to_string().into()
    }
}