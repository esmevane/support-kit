use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum OneOrMany<Contents> {
    Many(Vec<Contents>),
    One(Contents),
}

impl<Contents> From<Vec<Contents>> for OneOrMany<Contents> {
    fn from(value: Vec<Contents>) -> Self {
        Self::Many(value)
    }
}

impl<Contents> From<Contents> for OneOrMany<Contents> {
    fn from(value: Contents) -> Self {
        Self::One(value)
    }
}
