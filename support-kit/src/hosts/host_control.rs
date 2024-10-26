use crate::{DeploymentConfig, ShellCommand, SshError};

use super::HostSession;

pub struct HostControl;

impl HostControl {
    #[tracing::instrument(skip(deployment), level = "trace")]
    pub async fn on_hosts(
        deployment: &DeploymentConfig,
        commands: Vec<ShellCommand>,
    ) -> Result<(), SshError> {
        tracing::trace!(
            "executing on {num_hosts} hosts",
            num_hosts = deployment.hosts.len()
        );

        for host in deployment.hosts.clone() {
            tracing::trace!(host = ?host, "connecting to host");
            let connection = HostSession::connect().host(host).call().await?;

            for command in commands.clone() {
                connection.run_cmd(command.command_and_args()).await?;
            }
        }

        Ok(())
    }
}
