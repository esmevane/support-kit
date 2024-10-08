use bon::Builder;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::Config;

use super::{ServiceCommand, ServiceControl, ServiceControlError, ServiceLabel};

#[derive(Clone, Debug, Default, Deserialize, Parser, Serialize, PartialEq, Builder)]
#[clap(rename_all = "kebab-case")]
pub struct ServiceArgs {
    /// Control the service itself.
    #[clap(subcommand)]
    pub operation: Option<ServiceCommand>,
    /// The service label to use. Defaults to the binary name.
    #[clap(long = "name", short = 'n', default_value_t)]
    #[builder(default)]
    pub label: ServiceLabel,
    /// Install system-wide. If not set, attempts to install for the current user.
    #[clap(long)]
    #[builder(default)]
    pub system: bool,
}

impl ServiceArgs {
    pub fn execute(&self, config: Config) -> Result<(), ServiceControlError> {
        let control = ServiceControl::init(&config)?;

        if let Some(operation) = &self.operation {
            match operation {
                ServiceCommand::Install => control.install(PathBuf::new(), vec![]),
                ServiceCommand::Start => control.start(),
                ServiceCommand::Stop => control.stop(),
                ServiceCommand::Uninstall => control.uninstall(),
            }?;
        }

        Ok(())
    }
}

impl From<ServiceCommand> for ServiceArgs {
    fn from(command: ServiceCommand) -> Self {
        Self::builder().operation(command).build()
    }
}
