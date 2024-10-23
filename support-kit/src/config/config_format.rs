use std::path::PathBuf;

use strum::{EnumString, VariantArray};

use crate::Environment;

use super::{config_definition::ConfigDefinition, ConfigFile};

#[derive(Clone, Copy, Debug, strum::Display, EnumString, VariantArray)]
#[strum(serialize_all = "snake_case")]
pub enum ConfigFormat {
    Yaml,
    Json,
    Toml,
}

#[bon::bon]
impl ConfigFormat {
    #[builder]
    pub fn construct(
        &self,
        #[builder(into)] path: PathBuf,
        #[builder(default, into)] file: ConfigFile,
        env: Option<Environment>,
    ) -> ConfigDefinition {
        let ext = format!(
            "{ext}{kind}",
            ext = env.map(|env| format!("{env}.")).unwrap_or_default(),
            kind = self
        );

        let file = path.join(format!("{file}.{ext}"));
        if file.exists() {
            match self {
                Self::Yaml => ConfigDefinition::Yaml(file),
                Self::Json => ConfigDefinition::Json(file),
                Self::Toml => ConfigDefinition::Toml(file),
            }
        } else {
            ConfigDefinition::NotFound(file)
        }
    }

    pub fn all() -> Vec<ConfigFormat> {
        ConfigFormat::VARIANTS.to_vec()
    }

    pub fn empty_file_contents(&self) -> &'static str {
        match self {
            Self::Json => "{}",
            Self::Yaml => "",
            Self::Toml => "",
        }
    }
}
