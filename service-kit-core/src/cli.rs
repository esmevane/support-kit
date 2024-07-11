use std::path::PathBuf;

use serde::Serialize;
use service_kit_support::settings::SourceProvider;

use crate::APP_NAME;

/// A CLI application that helps do non-standard AzerothCore db tasks
#[derive(Clone, Debug, clap::Parser, Serialize)]
pub struct Cli {
    #[clap(flatten)]
    pub support_cli: service_kit_support::cli::Cli,
}

impl Cli {
    fn base_config_path(&self) -> String {
        match self.support_cli.global.config {
            Some(ref config) => config.clone(),
            None => String::new(),
        }
    }
}

impl SourceProvider for Cli {
    const APP_NAME: &'static str = APP_NAME;

    fn base_config(&self) -> service_kit_support::settings::BaseConfig {
        todo!()
    }

    fn home_config_path(&self) -> String {
        let mut path = PathBuf::new();
        path.push(dirs::home_dir().unwrap_or_default());
        path.push(".config");
        path.push(APP_NAME.to_lowercase());
        path.push("config");

        path.to_string_lossy().into()
    }

    fn environment_scoped_config_path(&self) -> String {
        let mut path = PathBuf::new();
        path.push(self.base_config_path());
        path.push(format!(
            "{}.{}",
            APP_NAME.to_lowercase(),
            self.support_cli.global.environment.clone()
        ));

        path.to_string_lossy().into()
    }

    fn root_config_path(&self) -> String {
        let mut path = PathBuf::new();
        path.push(self.base_config_path());
        path.push(APP_NAME.to_lowercase());

        path.to_string_lossy().into()
    }

    fn env_var_prefix(&self) -> String {
        todo!()
    }

    fn env_var_separator(&self) -> String {
        todo!()
    }
}
