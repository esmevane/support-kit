use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::Config;

use super::{ServiceCommand, ServiceControl, ServiceControlError};

#[derive(Clone, Debug, Default, Deserialize, Parser, Serialize, PartialEq)]
#[clap(rename_all = "kebab-case")]
pub struct ServiceArgs {
    /// Control the service itself.
    #[clap(subcommand)]
    pub operation: Option<ServiceCommand>,
    /// Install system-wide. If not set, attempts to install for the current user.
    #[clap(long)]
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
