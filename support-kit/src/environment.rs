use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIs, VariantArray};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumIs,
    VariantArray,
    AsRefStr,
    ValueEnum,
    Serialize,
    PartialEq,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Environment {
    Test,
    #[default]
    Development,
    Production,
}

impl Environment {
    pub fn all() -> Vec<Environment> {
        Environment::VARIANTS.to_vec()
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "test" => Ok(Environment::Test),
            "development" => Ok(Environment::Development),
            "production" => Ok(Environment::Production),
            _ => Err(value),
        }
    }
}

#[test]
fn all() {
    assert_eq!(
        Environment::all(),
        vec![
            Environment::Test,
            Environment::Development,
            Environment::Production
        ]
    );
}

#[test]
fn from_string() {
    assert_eq!(
        Environment::try_from("test".to_owned()),
        Ok(Environment::Test)
    );
    assert_eq!(
        Environment::try_from("development".to_owned()),
        Ok(Environment::Development)
    );

    assert_eq!(
        Environment::try_from("production".to_owned()),
        Ok(Environment::Production)
    );
}
