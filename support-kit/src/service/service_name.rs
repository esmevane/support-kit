use serde::{Deserialize, Serialize};
use service_manager::ServiceLabel;
use std::{fmt::Display, str::FromStr};

pub fn get_runtime_name() -> String {
    std::env::var("CARGO_PKG_NAME").unwrap_or_default()
}

#[derive(Debug, thiserror::Error)]
#[error("invalid service label: {0}")]
pub struct InvalidServiceLabelError(#[from] std::io::Error);

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ServiceName(String);

impl ServiceName {
    pub fn as_default_label(&self) -> Result<ServiceLabel, InvalidServiceLabelError> {
        let label_candidate = format!("local.{name}.service", name = self.0);
        Ok(label_candidate.parse()?)
    }
}

impl Default for ServiceName {
    fn default() -> Self {
        Self(get_runtime_name())
    }
}

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

#[test]
fn default_name() -> Result<(), Box<dyn std::error::Error>> {
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
fn custom_name() -> Result<(), Box<dyn std::error::Error>> {
    let config: ServiceName = serde_json::from_str(r#""custom-name""#)?;

    assert_eq!(config, ServiceName::from("custom-name"));

    Ok(())
}
