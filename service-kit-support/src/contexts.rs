use service_manager::ServiceLabel;

pub struct ServiceContext {
    pub operation: crate::settings::ServiceOperation,
    pub label: ServiceLabel,
    pub settings: crate::settings::Settings,
}

impl TryFrom<&crate::settings::Settings> for ServiceContext {
    type Error = crate::errors::Error;

    fn try_from(settings: &crate::settings::Settings) -> crate::Result<Self> {
        if let (Some(operation), Some(label)) = (
            &settings.config.service.operation,
            &settings.config.service.service_label,
        ) {
            Ok(Self {
                operation: operation.clone(),
                label: label.parse()?,
                settings: settings.clone(),
            })
        } else {
            Err(crate::errors::Error::ServiceNotConfiguredError)
        }
    }
}
