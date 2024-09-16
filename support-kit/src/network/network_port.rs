use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkPort(i32);

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
