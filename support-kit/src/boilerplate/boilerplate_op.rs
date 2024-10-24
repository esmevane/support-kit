use clap::Parser;

use crate::Configuration;

use super::BoilerplateTemplate;

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
pub enum BoilerplateOp {
    CreateDockerfile,
    CreateGithubActions,
    CreateCargoConfig,
}

impl BoilerplateOp {
    pub fn run(&self, config: &Configuration) -> crate::Result<()> {
        match self {
            Self::CreateDockerfile => BoilerplateTemplate::new(
                "infrastructure/containers",
                "Dockerfile",
                include_str!("templates/infrastructure-containers-dockerfile"),
            )
            .write(&config)?,
            Self::CreateGithubActions => {
                BoilerplateTemplate::new(
                    ".github/workflows",
                    "build.yaml",
                    include_str!("templates/github-workflow-build.yaml"),
                )
                .write(&config)?;

                BoilerplateTemplate::new(
                    ".github/workflows",
                    "test.yaml",
                    include_str!("templates/github-workflow-test.yaml"),
                )
                .write(&config)?;
            }
            Self::CreateCargoConfig => BoilerplateTemplate::new(
                ".cargo",
                "config.toml",
                include_str!("templates/cargo-config.toml"),
            )
            .write(&config)?,
        };

        Ok(())
    }
}
