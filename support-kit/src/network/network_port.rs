use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkPort(i32);

impl std::fmt::Display for NetworkPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i32> for NetworkPort {
    fn from(port: i32) -> Self {
        Self(port)
    }
}

impl Default for NetworkPort {
    fn default() -> Self {
        Self(80)
    }
}
