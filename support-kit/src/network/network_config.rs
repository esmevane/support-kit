use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

use crate::NetworkInitError;

use super::{NetworkHost, NetworkPort};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, bon::Builder)]
pub struct NetworkConfig {
    #[serde(default)]
    #[builder(default, into)]
    pub host: NetworkHost,

    #[serde(default)]
    #[builder(default, into)]
    pub port: NetworkPort,
}

impl NetworkConfig {
    pub fn address(&self) -> crate::Result<SocketAddr> {
        Ok(format!("{}:{}", self.host, self.port)
            .parse()
            .map_err(|error| NetworkInitError::from(error))?)
    }
}

impl From<&str> for NetworkConfig {
    fn from(host: &str) -> Self {
        NetworkConfig::builder().host(host).port(80).build()
    }
}

impl<T, U> From<(T, U)> for NetworkConfig
where
    T: AsRef<str>,
    U: Into<NetworkPort>,
{
    fn from((host, port): (T, U)) -> Self {
        NetworkConfig::builder()
            .host(host.as_ref())
            .port(port)
            .build()
    }
}
