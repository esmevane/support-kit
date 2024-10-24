use crate::HostDefinition;

#[derive(Debug, Clone)]
pub struct HostDetails {
    pub address: String,
    pub port: u16,
    pub user: String,
    pub auth: String,
}

#[bon::bon]
// pub port: Option<u16>,
// pub user: Option<String>,
// pub auth: Option<String>,
impl HostDetails {
    #[builder]
    pub fn new(
        #[builder(into)] address: String,
        #[builder(into)] port: Option<u16>,
        #[builder(into)] user: Option<String>,
        #[builder(into)] auth: Option<String>,
    ) -> Self {
        Self {
            address,
            port: port.unwrap_or(22),
            user: user.unwrap_or("root".into()),
            auth: auth.unwrap_or("~/.ssh/id_rsa".into()),
        }
    }
}

impl From<HostDefinition> for HostDetails {
    fn from(host: HostDefinition) -> Self {
        Self::builder()
            .address(host.address)
            .maybe_port(host.port)
            .maybe_user(host.user)
            .maybe_auth(host.auth)
            .build()
    }
}
