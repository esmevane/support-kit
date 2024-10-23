use figment::{
    providers::{Env, Format, Json, Toml, Yaml},
    Provider,
};
use std::{fmt::Debug, path::PathBuf};

use crate::{Environment, ServiceName};

use super::{ConfigEnvVar, ConfigFile, ConfigFormat};

#[derive(Clone, Debug, PartialEq)]
pub enum ConfigDefinition {
    NotFound(PathBuf),
    Yaml(PathBuf),
    Json(PathBuf),
    Toml(PathBuf),
    EnvVar(ConfigEnvVar),
}

#[bon::bon]
impl ConfigDefinition {
    #[builder(derive(Clone))]
    pub fn new(
        #[builder(default, into)] path: PathBuf,
        #[builder(default, into)] file: ConfigFile,
        env: Option<Environment>,
        #[builder(into)] format: Option<ConfigFormat>,
        #[builder(into)] env_var: Option<ConfigEnvVar>,
    ) -> ConfigDefinition {
        if let Some(env_var) = env_var {
            return env_var.into();
        }

        if let Some(format) = format {
            return format
                .construct()
                .maybe_env(env)
                .path(path)
                .file(file)
                .call();
        }

        ConfigDefinition::NotFound(path)
    }
}

impl Provider for ConfigDefinition {
    fn metadata(&self) -> figment::Metadata {
        Default::default()
    }

    fn data(
        &self,
    ) -> Result<figment::value::Map<figment::Profile, figment::value::Dict>, figment::Error> {
        match self {
            ConfigDefinition::NotFound(_) => Ok(Default::default()),
            ConfigDefinition::Yaml(path) => Yaml::file(path).data(),
            ConfigDefinition::Json(path) => Json::file(path).data(),
            ConfigDefinition::Toml(path) => Toml::file(path).data(),
            ConfigDefinition::EnvVar(env_var) => {
                Env::prefixed(&env_var.to_string()).split("__").data()
            }
        }
    }
}

// we know we can build a source definition from a few different combinations of builder fields
// 1. format + name and format + name + env = a file source
// 2. name and name + env = an env var source
// therefore if we can turn T into a service name, we can turn T into a source definition

impl<T> From<(ConfigFormat, T)> for ConfigDefinition
where
    T: Into<ServiceName>,
{
    fn from((format, name): (ConfigFormat, T)) -> Self {
        ConfigDefinition::builder()
            .format(format)
            .file(name.into())
            .build()
    }
}

impl<T> From<(ConfigFormat, T, Environment)> for ConfigDefinition
where
    T: Into<ServiceName>,
{
    fn from((format, name, env): (ConfigFormat, T, Environment)) -> Self {
        (format, name, Some(env)).into()
    }
}

impl<T> From<(ConfigFormat, T, Option<Environment>)> for ConfigDefinition
where
    T: Into<ServiceName>,
{
    fn from((format, name, maybe_env): (ConfigFormat, T, Option<Environment>)) -> Self {
        ConfigDefinition::builder()
            .format(format)
            .file(name.into())
            .maybe_env(maybe_env)
            .build()
    }
}

impl<T> From<T> for ConfigDefinition
where
    T: Into<ConfigEnvVar>,
{
    fn from(env_var: T) -> Self {
        ConfigDefinition::EnvVar(env_var.into())
    }
}
