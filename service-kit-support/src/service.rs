use service_manager::*;
use std::ffi::OsString;
use std::path::PathBuf;

use crate::contexts::{self, ServiceContext};
use crate::settings::{self, ServiceOperation};

pub struct Service {
    context: contexts::ServiceContext,
    manager: Box<dyn ServiceManager>,
}

impl Service {
    pub fn init(settings: &settings::Settings) -> crate::Result<Self> {
        // build a service context
        let context = contexts::ServiceContext::try_from(settings)?;
        let mut manager = <dyn ServiceManager>::native()?;

        match manager.set_level(ServiceLevel::User) {
            Ok(_) => {}
            Err(_) => {
                tracing::warn!(
                    "Failed to set user level service manager, defaulting to system level"
                );
                manager = <dyn ServiceManager>::native()?;
            }
        }

        Ok(Self { context, manager })
    }

    pub fn execute(&self, context: &ServiceContext) -> crate::Result<()> {
        match context.operation {
            ServiceOperation::Install => self.install(PathBuf::new(), vec![]),
            ServiceOperation::Start => self.start(),
            ServiceOperation::Stop => self.stop(),
            ServiceOperation::Uninstall => self.uninstall(),
        }
    }

    pub fn install(&self, program: PathBuf, args: Vec<OsString>) -> crate::Result<()> {
        self.manager.install(ServiceInstallCtx {
            label: self.context.label.clone(),
            program,
            args,
            contents: None,
            username: None,
            working_directory: None,
            environment: None,
        })?;

        Ok(())
    }

    pub fn start(&self) -> crate::Result<()> {
        self.manager.start(ServiceStartCtx {
            label: self.context.label.clone(),
        })?;

        Ok(())
    }

    pub fn stop(&self) -> crate::Result<()> {
        self.manager.stop(ServiceStopCtx {
            label: self.context.label.clone(),
        })?;

        Ok(())
    }

    pub fn uninstall(&self) -> crate::Result<()> {
        self.manager.uninstall(ServiceUninstallCtx {
            label: self.context.label.clone(),
        })?;

        Ok(())
    }
}
