use crate::{shell, Configuration, HostDetails, Registry, ShellCommand};

#[derive(Debug, Clone, bon::Builder)]
pub struct HostDeploymentContext {
    pub config: Configuration,
    #[builder(into)]
    pub host: HostDetails,
    pub registry: Registry,
}

impl HostDeploymentContext {
    pub fn send_file(
        &self,
        from_path: impl AsRef<str>,
        to_path: impl AsRef<str>,
    ) -> crate::Result<ShellCommand> {
        shell(format!(
            "scp -i {key} {local} {user}@{host}:{remote}",
            local = from_path.as_ref(),
            key = self.host.auth,
            user = self.host.user,
            host = self.host.address,
            remote = to_path.as_ref(),
        ))
    }
}
