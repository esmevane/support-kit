use clap::Parser;
use serde::{Deserialize, Serialize};
use service_manager::ServiceManagerKind;
use strum::{EnumString, VariantNames};

#[derive(Clone, Debug, Default, Deserialize, Parser, Serialize)]
pub struct Service {
    /// Control the service itself.
    #[clap(subcommand)]
    pub operation: Option<ServiceOperation>,
    /// The service label to use. Defaults to the app name.
    #[clap(long)]
    pub service_label: Option<String>,
    /// The kind of service manager to use. Defaults to system native.
    #[clap(long, value_enum)]
    pub service_manager: Option<ServiceManagerKind>,
    /// Install system-wide. If not set, attempts to install for the current user.
    #[clap(long)]
    pub system: bool,
}

#[derive(Clone, Debug, Deserialize, Parser, EnumString, VariantNames, Serialize)]
#[clap(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ServiceOperation {
    Install,
    Uninstall,
    Start,
    Stop,
}

impl ServiceOperation {
    pub fn options() -> &'static [&'static str] {
        Self::VARIANTS
    }
}
