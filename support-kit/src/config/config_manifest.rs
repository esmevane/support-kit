use figment::{Figment, Provider};
use std::fmt::Debug;

use super::ConfigDefinition;

#[derive(Default, Debug, bon::Builder, PartialEq)]
pub struct ConfigManifest {
    definitions: Vec<ConfigDefinition>,
}

impl ConfigManifest {
    pub fn merge(&mut self, source: ConfigManifest) {
        self.definitions.extend(source.definitions);
    }

    pub fn missing(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .filter(|definition| matches!(definition, ConfigDefinition::NotFound(_)))
            .cloned()
            .collect();

        Self::builder().definitions(definitions).build()
    }

    pub fn known(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .filter(|definition| !matches!(definition, ConfigDefinition::NotFound(_)))
            .cloned()
            .collect();

        Self::builder().definitions(definitions).build()
    }
}

impl Provider for ConfigManifest {
    fn metadata(&self) -> figment::Metadata {
        Default::default()
    }

    fn data(
        &self,
    ) -> Result<figment::value::Map<figment::Profile, figment::value::Dict>, figment::Error> {
        let mut figment = Figment::new();
        for source in &self.definitions {
            figment = figment.merge(source);
        }

        Ok(figment.data()?)
    }
}
