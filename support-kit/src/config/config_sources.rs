use bon::builder;
use figment::{Figment, Provider};
use std::fmt::Debug;
use std::path::PathBuf;

use crate::{Configuration, Environment};

use super::{ConfigDefinition, ConfigFile, ConfigFormat, ConfigManifest};

#[derive(Clone, Debug, bon::Builder)]
#[builder(derive(Clone))]
pub struct ConfigSources {
    #[builder(default, into)]
    file: ConfigFile,
    env: Option<Environment>,
}

impl ConfigSources {
    pub fn manifest(&self) -> ConfigManifest {
        let mut definitions = Vec::new();
        let definition = ConfigDefinition::builder()
            .maybe_env(self.env)
            .file(self.file.clone());

        for path in canonical_paths() {
            let file_definition = definition.clone().path(path);

            definitions.push(file_definition.clone().format(ConfigFormat::Yaml).build());
            definitions.push(file_definition.clone().format(ConfigFormat::Json).build());
            definitions.push(file_definition.clone().format(ConfigFormat::Toml).build());
        }

        definitions.push(definition.env_var((self.file.clone(), self.env)).build());

        ConfigManifest::builder().definitions(definitions).build()
    }

    pub fn sources(&self) -> figment::Result<ConfigManifest> {
        let manifest_builder = Self::builder().file(self.file.clone());
        let mut root_manifest = manifest_builder.clone().build().manifest();

        let next_env = match self.env {
            Some(env) => Some(env),
            None => {
                let figment = Figment::new().merge(&root_manifest);

                figment.extract::<Configuration>()?.environment
            }
        };

        let env_manifest = manifest_builder.env(next_env.unwrap_or_default());

        root_manifest.merge(env_manifest.build().manifest());

        Ok(root_manifest)
    }
}

impl Provider for ConfigSources {
    fn metadata(&self) -> figment::Metadata {
        Default::default()
    }

    fn data(
        &self,
    ) -> Result<figment::value::Map<figment::Profile, figment::value::Dict>, figment::Error> {
        let sources = self.sources()?;

        Ok(Figment::new().merge(sources).data()?)
    }
}

pub fn canonical_paths() -> Vec<PathBuf> {
    let mut paths = vec![PathBuf::new()];

    paths.extend(dirs::config_dir());
    paths.extend(dirs::home_dir());
    paths.reverse();

    paths
}

#[test]
fn basic_manifest_matches() {
    for format in ConfigFormat::all() {
        figment::Jail::expect_with(|jail| {
            jail.create_file(format!("support-kit.{format}"), "")?;
            jail.create_file(format!("support-kit.production.{format}"), "")?;

            let sources = ConfigSources::builder().file("support-kit").build();

            assert_eq!(
                sources.manifest().known(),
                ConfigManifest::builder()
                    .definitions(bon::vec![
                        ConfigDefinition::builder()
                            .file("support-kit")
                            .format(format)
                            .build(),
                        ConfigDefinition::builder().env_var("support-kit").build(),
                    ])
                    .build()
            );

            Ok(())
        });
    }
}

#[test]
fn env_specific_manifest_matches() {
    for env in Environment::all() {
        for format in ConfigFormat::all() {
            figment::Jail::expect_with(|jail| {
                jail.create_file(format!("support-kit.{format}"), "")?;
                jail.create_file(format!("support-kit.{env}.{format}"), "")?;

                let sources = ConfigSources::builder()
                    .file("support-kit")
                    .env(env)
                    .build();

                assert_eq!(
                    sources.manifest().known(),
                    ConfigManifest::builder()
                        .definitions(bon::vec![
                            ConfigDefinition::builder().format(format).env(env).build(),
                            ConfigDefinition::builder()
                                .env_var(("support-kit", env))
                                .build(),
                        ])
                        .build()
                );

                Ok(())
            });
        }
    }
}
