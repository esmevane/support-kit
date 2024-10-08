use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Default, Deserialize, Clone, Copy, Debug, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[clap(rename_all = "kebab-case")]
pub enum Color {
    Always,
    #[default]
    Auto,
    Never,
}

impl Color {
    pub fn init(self) {
        // Set a supports-color override based on the variable passed in.
        match self {
            Color::Always => owo_colors::set_override(true),
            Color::Auto => {}
            Color::Never => owo_colors::set_override(false),
        }
    }
}
