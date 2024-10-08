use service_manager::*;
use std::ffi::OsString;
use std::path::PathBuf;

use crate::Config;

use super::{ServiceCommand, ServiceControlError};

pub struct ServiceControl {
    label: ServiceLabel,
    manager: Box<dyn ServiceManager>,
}

impl ServiceControl {
    pub fn init(config: &Config) -> Result<Self, ServiceControlError> {
        let mut manager = <dyn ServiceManager>::native()?;

        match manager.set_level(ServiceLevel::User) {
            Ok(_) => {}
            Err(_) => {
                tracing::warn!(
                    "attempted to set user level service manager but failed, \
                    defaulting to system level."
                );
                manager = <dyn ServiceManager>::native()?;
            }
        }

        let label: ServiceLabel = match config.name() {
            Some(name) => name.parse()?,
            None => {
                return Err(ServiceControlError::InitializationError(
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Service name not provided",
                    ),
                ))
            }
        };

        Ok(Self { label, manager })
    }

    pub fn execute(&self, operation: ServiceCommand) -> Result<(), ServiceControlError> {
        match operation {
            ServiceCommand::Install => self.install(PathBuf::new(), vec![]),
            ServiceCommand::Start => self.start(),
            ServiceCommand::Stop => self.stop(),
            ServiceCommand::Uninstall => self.uninstall(),
        }?;

        Ok(())
    }

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

    pub fn start(&self) -> Result<(), ServiceControlError> {
        self.manager.start(ServiceStartCtx {
            label: self.label.clone(),
        })?;

        Ok(())
    }

    pub fn stop(&self) -> Result<(), ServiceControlError> {
        self.manager.stop(ServiceStopCtx {
            label: self.label.clone(),
        })?;

        Ok(())
    }

    pub fn uninstall(&self) -> Result<(), ServiceControlError> {
        self.manager.uninstall(ServiceUninstallCtx {
            label: self.label.clone(),
        })?;

        Ok(())
    }
}
