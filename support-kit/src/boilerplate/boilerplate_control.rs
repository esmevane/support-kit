use std::path::PathBuf;

use crate::Configuration;

use super::{BoilerplateContext, BoilerplatePreset, BoilerplateTemplate};

#[derive(Debug, Clone, bon::Builder)]
pub struct BoilerplateControl {
    #[builder(default)]
    pub config: Configuration,
    #[builder(into)]
    pub context: BoilerplateContext,
    #[builder(into, default)]
    pub base_path: PathBuf,
}

impl BoilerplateControl {
    pub fn write(&self, preset: BoilerplatePreset) -> crate::Result<()> {
        Ok(self.template(preset).write(&self.context)?)
    }

    pub fn template(&self, preset: BoilerplatePreset) -> BoilerplateTemplate {
        preset.init(&self)
    }
}

impl From<Configuration> for BoilerplateControl {
    fn from(config: Configuration) -> Self {
        Self::builder()
            .context(config)
            .base_path(PathBuf::from(std::env::current_dir().unwrap()))
            .build()
    }
}
