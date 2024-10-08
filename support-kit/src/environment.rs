use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIs};

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Display, EnumIs, ValueEnum, Serialize, PartialEq,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Environment {
    Test,
    #[default]
    Development,
    Production,
}
