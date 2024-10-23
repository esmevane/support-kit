use service_manager::*;
use std::ffi::OsString;
use std::path::PathBuf;

use crate::{Configuration, ServiceControlError};

use super::{ServiceCommand, ServiceName};

pub struct ServiceControl {
    name: ServiceName,
    label: ServiceLabel,
    manager: Box<dyn ServiceManager>,
}

impl std::fmt::Debug for ServiceControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServiceControl")
            .field("name", &self.name)
            .field("label", &self.label)
            .field("manager level", &self.manager.level())
            .field("manager available", &self.manager.available())
            .field("program", &self.program())
            .finish()
    }
}

impl ServiceControl {
    pub fn init(config: &Configuration) -> Result<Self, ServiceControlError> {
        let mut manager = match config.service.service_manager {
            Some(manager) => <dyn ServiceManager>::target(manager),
            None => <dyn ServiceManager>::native()?,
        };

        if !config.service.system {
            match manager.set_level(ServiceLevel::User) {
                Ok(_) => {}
                Err(_) => {
                    tracing::warn!(
                        "attempted to set user level service manager but failed, \
                        continuing at system level."
                    );
                }
            }
        }

        Ok(Self {
            name: config.name(),
            label: config.name().as_default_label()?,
            manager,
        })
    }

    fn program(&self) -> Result<PathBuf, ServiceControlError> {
        Ok(std::env::current_exe()?)
    }

    #[tracing::instrument(level = "trace")]
    pub fn execute(&self, operation: ServiceCommand) -> Result<(), ServiceControlError> {
        match operation {
            ServiceCommand::Install(install) => {
                tracing::trace!(
                    install_args = ?install,
                    "installing with args"
                );

                self.install(self.program()?, install.args)
            }
            ServiceCommand::Start => self.start(),
            ServiceCommand::Stop => self.stop(),
            ServiceCommand::Uninstall => self.uninstall(),
        }?;

        Ok(())
    }

    #[tracing::instrument(level = "trace")]
    pub fn install(
        &self,
        program: PathBuf,
        args: Vec<OsString>,
    ) -> Result<(), ServiceControlError> {
        self.manager.install(ServiceInstallCtx {
            label: self.label.clone(),
            program,
            args,
            contents: None,
            username: None,
            working_directory: None,
            environment: None,
        })?;

        Ok(())
    }

    #[tracing::instrument(level = "trace")]
    pub fn start(&self) -> Result<(), ServiceControlError> {
        self.manager.start(ServiceStartCtx {
            label: self.label.clone(),
        })?;

        Ok(())
    }

    #[tracing::instrument(level = "trace")]
    pub fn stop(&self) -> Result<(), ServiceControlError> {
        self.manager.stop(ServiceStopCtx {
            label: self.label.clone(),
        })?;

        Ok(())
    }

    #[tracing::instrument(level = "trace")]
    pub fn uninstall(&self) -> Result<(), ServiceControlError> {
        self.manager.uninstall(ServiceUninstallCtx {
            label: self.label.clone(),
        })?;

        Ok(())
    }
}
