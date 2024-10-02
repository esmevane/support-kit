#[derive(Clone, Debug, Default, serde::Deserialize, PartialEq, bon::Builder)]
#[serde(rename_all = "kebab-case")]
pub struct ServiceConfig {
    label: Option<String>,
}
