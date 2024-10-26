use clap::Parser;

use crate::DeploymentCommand;

#[derive(Clone, Debug, Parser, PartialEq)]
#[clap(rename_all = "kebab-case")]
pub struct DeploymentArgs {
    #[clap(subcommand)]
    pub command: Option<DeploymentCommand>,
}

// AppCommand::Debug => {}
// AppCommand::Remote { command } => command.exec_remote(&controller).await?,
// AppCommand::Local { command } => command.exec_local(&controller).await?,
