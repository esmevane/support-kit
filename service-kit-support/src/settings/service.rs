use service_manager::ServiceManagerKind;
use strum::{EnumString, VariantNames};

#[derive(Clone, Debug)]
pub struct Service {
    pub service_label: Option<String>,
    pub service_manager: Option<ServiceManagerKind>,
    pub system: bool,
}

#[derive(Clone, Debug, EnumString, VariantNames)]
#[strum(serialize_all = "kebab-case")]
pub enum ServiceOperation {
    Install,
    Uninstall,
    Start,
    Stop,
}

impl ServiceOperation {
    pub fn options() -> &'static [&'static str] {
        Self::VARIANTS
    }
}
