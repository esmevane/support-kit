use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkHost(String);

impl std::fmt::Display for NetworkHost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> From<T> for NetworkHost
where
    T: AsRef<str>,
{
    fn from(host: T) -> Self {
        Self(host.as_ref().to_string())
    }
}

impl Default for NetworkHost {
    fn default() -> Self {
        Self("0.0.0.0".to_string())
    }
}
