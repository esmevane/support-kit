use std::{sync::Arc, time::Duration};

use russh::ChannelMsg;
use tokio::io::AsyncWriteExt;

use crate::SshError;

use super::{HostDetails, SshConnection};

pub struct SshSession {
    pub connection: russh::client::Handle<SshConnection>,
}

impl SshSession {
    pub async fn connect(host: &HostDetails) -> Result<Self, SshError> {
        let config = Arc::new(russh::client::Config {
            inactivity_timeout: Some(Duration::from_secs(5)),
            ..<_>::default()
        });

        let mut session =
            russh::client::connect(config, (host.address.as_ref(), host.port), SshConnection)
                .await?;

        let key_pair = russh::keys::load_secret_key(&host.auth, None)?;

        let auth_res = session
            .authenticate_publickey(&host.user, Arc::new(key_pair))
            .await?;

        if !auth_res {
            return Err(SshError::AuthenticationFailed);
        }

        tracing::debug!("ssh session established: {address}", address = host.address);

        Ok(SshSession {
            connection: session,
        })
    }

    pub async fn run_cmd<T>(&self, command: Vec<T>) -> Result<(), SshError>
    where
        T: AsRef<str>,
    {
        let mut channel = self.connection.channel_open_session().await?;
        let command = command
            .into_iter()
            .map(|x| shell_escape::escape(x.as_ref().to_owned().into()))
            .collect::<Vec<_>>()
            .join(" ");

        channel.exec(true, command).await?;

        let mut code = None;
        let mut stdout = tokio::io::stdout();

        loop {
            // There's an event available on the session channel
            let Some(msg) = channel.wait().await else {
                break;
            };
            match msg {
                // Write data to the terminal
                ChannelMsg::Data { ref data } => {
                    stdout.write_all(data).await?;
                    stdout.flush().await?;
                }
                // The command has returned an exit code
                ChannelMsg::ExitStatus { exit_status } => {
                    code = Some(exit_status);
                    // cannot leave the loop immediately, there might still be more data to receive
                }
                other => {
                    tracing::debug!("channel message: {:?}", other);
                }
            }
        }

        // Wait for the channel to close
        channel.close().await?;

        // report code

        if let Some(code) = code {
            println!("Exit code: {}", code);
        }

        Ok(())
    }
}
