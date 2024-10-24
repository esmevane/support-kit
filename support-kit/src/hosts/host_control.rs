use std::future::Future;

use crate::{Deployment, ShellCommand, SshError};

use super::HostSession;

pub struct HostControl;

impl HostControl {
    pub async fn on_hosts(
        deployment: &Deployment,
        commands: Vec<ShellCommand>,
    ) -> Result<(), SshError> {
        Self::exec_on_hosts(&deployment, |connection| {
            let commands = commands.clone();
            async move {
                for command in commands {
                    connection.run_cmd(command.command_and_args()).await?;
                }
                Ok(())
            }
        })
        .await?;

        Ok(())
    }

    async fn exec_on_hosts<Func, Fut>(
        deployment: &Deployment,
        callback_fn: Func,
    ) -> Result<(), SshError>
    where
        Func: Fn(HostSession) -> Fut,
        Fut: Future<Output = Result<(), SshError>>,
    {
        for host in deployment.hosts.clone() {
            callback_fn(HostSession::connect().host(host).call().await?).await?;
        }

        Ok(())
    }
}
