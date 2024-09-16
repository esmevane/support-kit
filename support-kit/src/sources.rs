use convert_case::{Case, Casing};
use figment::{
    providers::{Env, Format, Json, Toml, Yaml},
    Figment, Provider,
};
use std::path::PathBuf;

#[derive(Clone, Debug, bon::Builder)]
pub struct Sources {
    #[builder(default, into)]
    name: crate::ServiceName,
    env: Option<crate::Environment>,
}

impl Sources {
    pub fn with_env(&self, env: crate::Environment) -> Self {
        Self::builder().name(self.name.clone()).env(env).build()
    }

    pub fn prefix(&self) -> Figment {
        let prefix = match self.env {
            Some(env) => format!(
                "{name}__{config_env}__",
                name = self.name.to_string().to_case(Case::UpperSnake),
                config_env = env.to_string().to_case(Case::UpperSnake)
            ),
            None => format!(
                "{name}__",
                name = self.name.to_string().to_case(Case::UpperSnake)
            ),
        };

        Figment::new().merge(Env::prefixed(&prefix).split("__"))
    }

    fn sources(&self) -> figment::Result<Figment> {
        let ext = self.env.map(|env| format!("{env}.")).unwrap_or_default();
        let mut figment = Figment::new();

        for path in canonical_paths() {
            let file = path.with_file_name(&self.name);

            let yaml = file.with_extension(format!("{ext}yaml"));
            if yaml.exists() {
                figment = figment.merge(Yaml::file(yaml))
            }

            let json = file.with_extension(format!("{ext}json"));
            if json.exists() {
                figment = figment.merge(Json::file(json))
            }

            let toml = file.with_extension(format!("{ext}toml"));
            if toml.exists() {
                figment = figment.merge(Toml::file(toml))
            }
        }

        Ok(figment)
    }
}

impl Provider for Sources {
    fn metadata(&self) -> figment::Metadata {
        Default::default()
    }

    fn data(
        &self,
    ) -> Result<figment::value::Map<figment::Profile, figment::value::Dict>, figment::Error> {
        self.sources()?.data()
    }
}

fn canonical_paths() -> Vec<PathBuf> {
    let mut paths = vec![PathBuf::new()];

    paths.extend(dirs::config_dir());
    paths.extend(dirs::home_dir());
    paths.reverse();

    paths
}
