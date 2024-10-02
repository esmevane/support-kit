use clap::Parser;
use serde::{Deserialize, Serialize};
use strum::{EnumString, VariantNames};

#[derive(
    Copy, Clone, Debug, Deserialize, Parser, EnumString, VariantNames, Serialize, PartialEq,
)]
#[clap(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ServiceCommand {
    Install,
    Uninstall,
    Start,
    Stop,
}

impl ServiceCommand {
    pub fn options() -> &'static [&'static str] {
        Self::VARIANTS
    }
}
