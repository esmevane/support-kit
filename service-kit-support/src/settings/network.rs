use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Network {
    pub host: String,
    pub port: u16,
}

impl Network {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub async fn listener(&self) -> crate::Result<tokio::net::TcpListener> {
        Ok(tokio::net::TcpListener::bind(self.address()).await?)
    }
}
