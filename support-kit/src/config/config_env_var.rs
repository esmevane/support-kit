use bon::builder;
use convert_case::{Case, Casing};
use std::fmt::{Debug, Display};

use crate::Environment;

use super::ConfigFile;

#[derive(Clone, Default, bon::Builder, PartialEq)]
pub struct ConfigEnvVar {
    #[builder(into)]
    file: ConfigFile,
    env: Option<Environment>,
}

impl Display for ConfigEnvVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{prefix}",
            prefix = env_prefix()
                .name(self.file.clone())
                .maybe_env(self.env)
                .call()
        )
    }
}

impl Debug for ConfigEnvVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{prefix:?}",
            prefix = env_prefix()
                .name(self.file.clone())
                .maybe_env(self.env)
                .call()
        )
    }
}

#[bon::builder]
pub fn env_prefix(#[builder(into)] name: String, env: Option<Environment>) -> String {
    let name = name
        .replace(|c: char| !c.is_alphanumeric(), "_")
        .to_case(Case::UpperSnake);

    match env {
        Some(env) => format!(
            "{name}__{config_env}__",
            config_env = env.to_string().to_case(Case::UpperSnake)
        ),
        None => format!("{name}__", name = name.to_case(Case::UpperSnake)),
    }
}

impl<T> From<T> for ConfigEnvVar
where
    T: Into<ConfigFile>,
{
    fn from(name: T) -> Self {
        Self {
            file: name.into().into(),
            env: None,
        }
    }
}

impl<T> From<(T, Environment)> for ConfigEnvVar
where
    T: Into<ConfigFile>,
{
    fn from((name, env): (T, Environment)) -> Self {
        Self {
            file: name.into().into(),
            env: Some(env),
        }
    }
}

impl<T> From<(T, Option<Environment>)> for ConfigEnvVar
where
    T: Into<ConfigFile>,
{
    fn from((name, env): (T, Option<Environment>)) -> Self {
        Self {
            file: name.into().into(),
            env,
        }
    }
}
