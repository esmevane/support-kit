use std::{future::Future, sync::Arc, time::Duration};

use async_trait::async_trait;
use russh::{client, keys, ChannelMsg};
use tokio::io::AsyncWriteExt;

use crate::{Deployment, SshError};

struct SshHostConfig {
    address: String,
    port: u16,
    user: String,
    key: String,
}

impl SshHostConfig {
    pub fn new(address: String, port: u16, user: String, key: String) -> Self {
        Self {
            address,
            port,
            user,
            key,
        }
    }
}

pub struct SshHost {
    _config: SshHostConfig,
    session: SshSession,
}

impl SshHost {
    pub async fn connect(
        address: impl AsRef<str>,
        port: u16,
        user: impl AsRef<str>,
        key: impl AsRef<str>,
    ) -> Result<Self, SshError> {
        let config = SshHostConfig::new(
            address.as_ref().into(),
            port,
            user.as_ref().into(),
            key.as_ref().into(),
        );
        Ok(Self {
            session: SshSession::connect(&config).await?,
            _config: config,
        })
    }

    pub async fn run_cmd<T>(&self, cmd: Vec<T>) -> Result<(), SshError>
    where
        T: AsRef<str>,
    {
        self.session.run_cmd(cmd).await
    }
}

struct SshSession {
    connection: client::Handle<SshConnection>,
}

impl SshSession {
    pub async fn connect(host: &SshHostConfig) -> Result<Self, SshError> {
        let config = Arc::new(client::Config {
            inactivity_timeout: Some(Duration::from_secs(5)),
            ..<_>::default()
        });

        let mut session =
            client::connect(config, (host.address.as_ref(), host.port), SshConnection).await?;

        let key_pair = keys::load_secret_key(&host.key, None)?;

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

struct SshConnection;

// the methods are async w/ async_trait, so that should be imported if you want to use them
#[async_trait]
impl client::Handler for SshConnection {
    type Error = russh::Error;
    async fn check_server_key(
        &mut self,
        _server_public_key: &keys::key::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

pub struct SshControl;

impl SshControl {
    pub async fn on_hosts<Func, Fut>(
        deployment: &Deployment,
        callback_fn: Func,
    ) -> Result<(), SshError>
    where
        Func: Fn(SshHost) -> Fut,
        Fut: Future<Output = Result<(), SshError>>,
    {
        for host in deployment.hosts.clone() {
            let connection = SshHost::connect(
                host.address,
                host.port.unwrap_or(22),
                host.user.unwrap_or("root".into()),
                host.auth.unwrap_or("~/.ssh/id_rsa".into()),
            )
            .await?;

            callback_fn(connection).await?;
        }

        Ok(())
    }
}
