use service_manager::*;
use std::ffi::OsString;
use std::path::PathBuf;

use crate::Config;

use super::{ServiceCommand, ServiceControlError, ServiceName};

pub struct ServiceControl {
    name: ServiceName,
    label: ServiceLabel,
    manager: Box<dyn ServiceManager>,
}

impl ServiceControl {
    pub fn init(config: &Config) -> Result<Self, ServiceControlError> {
        let mut manager = match config.service.service_manager {
            Some(manager) => <dyn ServiceManager>::target(manager),
            None => <dyn ServiceManager>::native()?,
        };

        if config.service.system {
            match manager.set_level(ServiceLevel::System) {
                Ok(_) => {}
                Err(_) => {
                    tracing::warn!(
                        "attempted to set system level service manager but failed, \
                        continuing at user level."
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

    pub fn execute(&self, operation: ServiceCommand) -> Result<(), ServiceControlError> {
        let program = std::env::current_exe()?;
        let args: Vec<std::ffi::OsString> = vec![
            "-n".into(),
            self.name.to_string().into(),
            "server".into(),
            "api".into(),
        ];

        tracing::trace!(
            program = ?program,
            args = ?args,
            "executing service command"
        );

        match operation {
            ServiceCommand::Install => self.install(program, args),
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
