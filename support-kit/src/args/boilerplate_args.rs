use clap::{Parser, Subcommand};

use crate::BoilerplatePreset;

#[derive(Clone, Debug, Parser, PartialEq)]
#[clap(rename_all = "kebab-case")]
pub struct BoilerplateArgs {
    #[clap(subcommand)]
    pub command: Option<BoilerplateCommand>,
}

#[derive(Clone, Debug, Subcommand, PartialEq)]
#[clap(rename_all = "kebab-case")]
pub enum BoilerplateCommand {
    Init,
    Template {
        #[clap(subcommand)]
        command: BoilerplatePreset,
    },
}
