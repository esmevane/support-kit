use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::ServiceName;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ConfigFile(String);

impl Display for ConfigFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for ConfigFile {
    fn default() -> Self {
        ServiceName::default().into()
    }
}

impl<T> From<T> for ConfigFile
where
    T: Into<ServiceName>,
{
    fn from(name: T) -> Self {
        Self(name.into().to_string())
    }
}

impl From<ConfigFile> for String {
    fn from(file: ConfigFile) -> Self {
        file.0
    }
}
