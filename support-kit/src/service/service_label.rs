use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ServiceLabel(String);

impl Display for ServiceLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ServiceLabel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl From<&str> for ServiceLabel {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Default for ServiceLabel {
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

    let config: ServiceLabel = serde_json::from_str(r#""support-kit""#)?;

    assert_eq!(config, ServiceLabel::default());
    assert_eq!(config, ServiceLabel::from("support-kit"));

    Jail::expect_with(|jail| {
        jail.set_env("CARGO_PKG_NAME", "consumer-package");

        let config: ServiceLabel =
            serde_json::from_str(r#""consumer-package""#).expect("failed to parse");

        assert_eq!(config, ServiceLabel::default());
        assert_eq!(config, ServiceLabel::from("consumer-package"));

        Ok(())
    });
    Ok(())
}

#[test]
fn custom_service_label() -> Result<(), Box<dyn std::error::Error>> {
    let config: ServiceLabel = serde_json::from_str(r#""custom-name""#)?;

    assert_eq!(config, ServiceLabel::from("custom-name"));

    Ok(())
}
