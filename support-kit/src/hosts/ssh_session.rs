use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use russh::ChannelMsg;
use tokio::io::AsyncWriteExt;

use crate::SshError;

use super::{HostDetails, SshConnection};

pub struct SshSession {
    pub connection: russh::client::Handle<SshConnection>,
}

impl SshSession {
    #[tracing::instrument(skip(host), level = "debug")]
    pub async fn connect(host: &HostDetails) -> Result<Self, SshError> {
        let config = Arc::new(russh::client::Config {
            inactivity_timeout: Some(Duration::from_secs(5)),
            ..<_>::default()
        });

        let mut session =
            russh::client::connect(config, (host.address.as_ref(), host.port), SshConnection)
                .await?;

        tracing::debug!("canonicalizing path to key: {path}", path = host.auth);
        let path = expand_tilde(&host.auth).ok_or(SshError::InvalidPath(host.auth.clone()))?;

        let key_pair = russh::keys::load_secret_key(path, None)?;
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

    #[tracing::instrument(skip(self, command), level = "debug")]
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
                tracing::trace!("channel closed");
                break;
            };

            match msg {
                // Write data to the terminal
                ChannelMsg::Data { ref data } => {
                    tracing::trace!(
                        "received data: {data}",
                        data = String::from_utf8_lossy(data)
                    );
                    stdout.write_all(data).await?;
                    stdout.flush().await?;
                }
                // The command has returned an exit code
                ChannelMsg::ExitStatus { exit_status } => {
                    tracing::trace!("exit status: {exit_status}", exit_status = exit_status);
                    code = Some(exit_status);
                    // cannot leave the loop immediately, there might still be more data to receive
                }
                other => {
                    tracing::trace!("unhandled channel message: {:?}", other);
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

// definitely an easier way to do this, but for now, cribbed from
// https://stackoverflow.com/questions/54267608/expand-tilde-in-rust-path-idiomatically
#[tracing::instrument(skip(path_user_input), level = "trace")]
fn expand_tilde<P: AsRef<Path>>(path_user_input: P) -> Option<PathBuf> {
    let path = path_user_input.as_ref();
    if !path.starts_with("~") {
        return Some(path.to_path_buf());
    }
    if path == Path::new("~") {
        return dirs::home_dir();
    }
    dirs::home_dir().map(|mut home| {
        if home == Path::new("/") {
            // Corner case: `home` root directory;
            // don't prepend extra `/`, just drop the tilde.
            path.strip_prefix("~").unwrap().to_path_buf()
        } else {
            home.push(path.strip_prefix("~/").unwrap());
            home
        }
    })
}

#[test]
fn test_expand_tilde() {
    // Should work on your linux box during tests, would fail in stranger
    // environments!
    let home = std::env::var("HOME").unwrap();
    let projects = PathBuf::from(format!("{}/Projects", home));
    assert_eq!(expand_tilde("~/Projects"), Some(projects));
    assert_eq!(expand_tilde("/foo/bar"), Some("/foo/bar".into()));
    assert_eq!(
        expand_tilde("~alice/projects"),
        Some("~alice/projects".into())
    );
}
