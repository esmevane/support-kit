use std::{fmt::Display, path::PathBuf, str::FromStr};

use bon::Builder;
use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::Config;

use super::{ServiceCommand, ServiceControl, ServiceControlError};

#[derive(Clone, Debug, Default, Deserialize, Parser, Serialize, PartialEq, Builder)]
#[clap(rename_all = "kebab-case")]
pub struct ServiceArgs {
    /// Control the service itself.
    #[clap(subcommand)]
    pub operation: Option<ServiceCommand>,
    /// The service label to use. Defaults to the binary name.
    #[clap(long = "name", short = 'n', default_value_t)]
    #[builder(default)]
    pub label: Label,
    /// Install system-wide. If not set, attempts to install for the current user.
    #[clap(long)]
    #[builder(default)]
    pub system: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Label(String);

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Label {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl From<&str> for Label {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Default for Label {
    fn default() -> Self {
        Self(get_runtime_name())
    }
}

fn get_runtime_name() -> String {
    std::env::var("CARGO_PKG_NAME").unwrap_or_default()
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
