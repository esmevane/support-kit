use crate::SshError;

use super::{HostDetails, SshSession};

pub struct HostSession {
    _config: HostDetails,
    session: SshSession,
}

#[bon::bon]
impl HostSession {
    #[builder]
    #[tracing::instrument(skip(host), level = "trace")]
    pub async fn connect(#[builder(into)] host: HostDetails) -> Result<Self, SshError> {
        Ok(Self {
            session: SshSession::connect(&host).await?,
            _config: host,
        })
    }

    #[tracing::instrument(skip(self, cmd), level = "trace")]
    pub async fn run_cmd<T>(&self, cmd: Vec<T>) -> Result<(), SshError>
    where
        T: AsRef<str>,
    {
        self.session.run_cmd(cmd).await
    }
}
