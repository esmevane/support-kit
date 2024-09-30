use nutype::nutype;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{Display, EnumIs};
use thiserror::Error;

#[nutype(
    sanitize(trim, lowercase),
    validate(with = validate_environment, error = EnvironmentError),
    derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq),
    default = Environment::DEFAULT_ENVIRONMENT
)]
pub struct Environment(String);

impl Environment {
    pub const TEST: &'static str = "test";
    pub const DEVELOPMENT: &'static str = "development";
    pub const PRODUCTION: &'static str = "production";

    pub const DEFAULT_ENVIRONMENT: &'static str = Self::DEVELOPMENT;

    pub fn as_enum(&self) -> EnvironmentEnum {
        self.clone().into()
    }
}

#[derive(Error, Debug, PartialEq)]
#[error(
    "The environment is not valid: {0}, \
            expected one of: test, development, production."
)]
pub struct EnvironmentError(String);

fn validate_environment(value: impl AsRef<str>) -> Result<(), EnvironmentError> {
    let value = value.as_ref();

    match value {
        "test" | "development" | "production" => Ok(()),
        _ => Err(EnvironmentError(value.to_string())),
    }
}

impl FromStr for Environment {
    type Err = EnvironmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Environment::try_new(s)
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Display, EnumIs, Serialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum EnvironmentEnum {
    Test,
    #[default]
    Development,
    Production,
}

impl From<Environment> for EnvironmentEnum {
    fn from(environment: Environment) -> Self {
        match environment.into_inner().as_str() {
            Environment::TEST => Self::Test,
            Environment::DEVELOPMENT => Self::Development,
            Environment::PRODUCTION => Self::Production,
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_environment() -> Result<(), EnvironmentError> {
    for raw_value in ["test", "development", "production"] {
        let environment = Environment::try_new(raw_value)?;
        assert_eq!(environment.into_inner(), *raw_value);
    }

    assert_eq!(
        Environment::try_new("invalid").unwrap_err().to_string(),
        "The environment is not valid: invalid, \
            expected one of: test, development, production."
    );

    Ok(())
}
