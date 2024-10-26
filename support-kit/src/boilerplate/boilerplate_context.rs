use convert_case::Casing;
use serde::Serialize;

use crate::Configuration;

#[derive(Debug, Clone, bon::Builder, Serialize)]
pub struct BoilerplateContext {
    #[builder(into)]
    pub name: String,
    #[builder(default, into)]
    pub secret_name: String,
    pub image: Option<String>,
}

impl From<Configuration> for BoilerplateContext {
    fn from(config: Configuration) -> Self {
        let secret_name = config
            .name()
            .to_string()
            .to_case(convert_case::Case::UpperSnake);

        Self::builder()
            .name(config.name())
            .secret_name(secret_name)
            .build()
    }
}
