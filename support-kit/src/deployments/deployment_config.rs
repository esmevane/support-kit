use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct DeploymentConfig {
    pub artifacts: Option<Artifacts>,
    pub hosts: Vec<HostDefinition>,
    #[serde(default)]
    pub security: SecurityConfig,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Artifacts {
    pub containers: Option<Containers>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Containers {
    pub registry: Option<Registry>,
    pub images: Vec<ImageDefinition>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Registry {
    pub account: String,
    pub host: String,
    pub token: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct ImageDefinition {
    pub definition: String,
    pub name: String,
    pub label: String,
    pub namespace: String,
    pub repo: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct HostDefinition {
    pub address: String,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub auth: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
struct Security {
    certificates: Option<SecurityConfig>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum SecurityConfig {
    Acme {
        domains: Vec<String>,
        emails: Vec<String>,
        cache: Option<String>,
        production: bool,
    },
    #[serde(untagged)]
    #[default]
    Off,
    #[serde(untagged)]
    Unknown(serde_json::Value),
}
