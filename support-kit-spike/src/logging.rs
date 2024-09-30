use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::{Level, LevelFilter};

#[derive(Debug, Clone, Default, Deserialize, Serialize, Parser, PartialEq)]
#[command(rename_all = "kebab-case", about = None, long_about = None)]
pub struct Logging {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub level: Option<Level>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub log_filter: Option<LevelFilter>,
}
