use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ServiceName(String);

impl Display for ServiceName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ServiceName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl From<&str> for ServiceName {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Default for ServiceName {
    fn default() -> Self {
        Self(get_runtime_name())
    }
}

pub fn get_runtime_name() -> String {
    std::env::var("CARGO_PKG_NAME").unwrap_or_default()
}

#[test]
fn default_label() -> Result<(), Box<dyn std::error::Error>> {
    use figment::Jail;

    let config: ServiceName = serde_json::from_str(r#""support-kit""#)?;

    assert_eq!(config, ServiceName::default());
    assert_eq!(config, ServiceName::from("support-kit"));

    Jail::expect_with(|jail| {
        jail.set_env("CARGO_PKG_NAME", "consumer-package");

        let config: ServiceName =
            serde_json::from_str(r#""consumer-package""#).expect("failed to parse");

        assert_eq!(config, ServiceName::default());
        assert_eq!(config, ServiceName::from("consumer-package"));

        Ok(())
    });
    Ok(())
}

#[test]
fn custom_service_label() -> Result<(), Box<dyn std::error::Error>> {
    let config: ServiceName = serde_json::from_str(r#""custom-name""#)?;

    assert_eq!(config, ServiceName::from("custom-name"));

    Ok(())
}
