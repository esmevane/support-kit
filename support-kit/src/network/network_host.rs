#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct NetworkHost(String);

impl From<&str> for NetworkHost {
    fn from(host: &str) -> Self {
        Self(host.to_string())
    }
}

impl Default for NetworkHost {
    fn default() -> Self {
        Self("0.0.0.0".to_string())
    }
}
