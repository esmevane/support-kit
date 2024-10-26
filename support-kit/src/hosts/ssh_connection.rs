use async_trait::async_trait;

pub struct SshConnection;

// the methods are async w/ async_trait, so that should be imported if you want to use them
#[async_trait]
impl russh::client::Handler for SshConnection {
    type Error = russh::Error;
    #[tracing::instrument(skip(self, _server_public_key), level = "debug")]
    async fn check_server_key(
        &mut self,
        _server_public_key: &russh::keys::key::PublicKey,
    ) -> Result<bool, Self::Error> {
        tracing::trace!("checking server key");
        Ok(true)
    }
}
