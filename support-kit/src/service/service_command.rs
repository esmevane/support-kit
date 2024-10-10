use std::ffi::OsString;

use clap::Parser;
use serde::{Deserialize, Serialize};
use strum::{EnumString, VariantNames};

#[derive(Clone, Debug, Deserialize, Parser, EnumString, VariantNames, Serialize, PartialEq)]
#[clap(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ServiceCommand {
    Install(InstallArgs),
    Uninstall,
    Start,
    Stop,
}

#[derive(Clone, Debug, Default, Deserialize, Parser, Serialize, PartialEq)]
#[clap(rename_all = "kebab-case")]
pub struct InstallArgs {
    #[clap(raw = true, required = false)]
    pub args: Vec<OsString>,
}

impl ServiceCommand {
    pub fn options() -> &'static [&'static str] {
        Self::VARIANTS
    }
}
