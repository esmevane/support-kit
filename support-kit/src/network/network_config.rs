use super::{NetworkHost, NetworkPort};

#[derive(Clone, Debug, Default, serde::Deserialize, PartialEq, bon::Builder)]
pub struct NetworkConfig {
    #[serde(default)]
    #[builder(default, into)]
    host: NetworkHost,

    #[serde(default)]
    #[builder(default, into)]
    port: NetworkPort,
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
