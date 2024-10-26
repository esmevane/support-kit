use std::path::PathBuf;

use clap::Subcommand;
use strum::VariantArray;

use super::BoilerplateTemplate;

#[derive(Clone, Debug, Subcommand, PartialEq, VariantArray)]
#[clap(rename_all = "kebab-case")]
pub enum BoilerplatePreset {
    Dockerfile,
    BuildAction,
    TestAction,
    CargoConfig,
}

impl BoilerplatePreset {
    pub fn all() -> Vec<Self> {
        Self::VARIANTS.to_vec()
    }

    pub fn init(&self, base_path: &PathBuf) -> BoilerplateTemplate {
        match self {
            Self::Dockerfile => BoilerplateTemplate::builder()
                .path(base_path.join("infrastructure/containers"))
                .file_name("Dockerfile")
                .source(include_str!(
                    "templates/infrastructure-containers-dockerfile"
                ))
                .build(),
            Self::BuildAction => BoilerplateTemplate::builder()
                .path(base_path.join(".github/workflows"))
                .file_name("build.yaml")
                .source(include_str!("templates/github-workflow-build.yaml"))
                .build(),
            Self::TestAction => BoilerplateTemplate::builder()
                .path(base_path.join(".github/workflows"))
                .file_name("test.yaml")
                .source(include_str!("templates/github-workflow-test.yaml"))
                .build(),
            Self::CargoConfig => BoilerplateTemplate::builder()
                .path(base_path.join(".cargo"))
                .file_name("config.toml")
                .source(include_str!("templates/cargo-config.toml"))
                .build(),
        }
    }
}
