use figment::providers::{Env, Format, Json, Toml, Yaml};
use figment::{Figment, Metadata, Profile, Provider};

pub type FigmentDataResult = figment::Result<figment::value::Map<Profile, figment::value::Dict>>;

pub struct Source {
    file_basename: String,
    env_prefix: String,
}

impl Source {
    pub fn new(file_basename: impl AsRef<str>, env_prefix: impl AsRef<str>) -> Self {
        Self {
            file_basename: file_basename.as_ref().to_string(),
            env_prefix: env_prefix.as_ref().to_string(),
        }
    }

    fn figment(&self) -> Figment {
        let prefix = &self.env_prefix;

        Figment::new()
            .merge(self.file_sources())
            .merge(Env::prefixed(&format!("{prefix}_")))
    }

    fn sources(&self) -> Vec<String> {
        let file_name = self.file_basename.clone();
        let config_dir_file =
            dirs::config_dir().and_then(|dir| dir.join(&file_name).to_str().map(String::from));

        let mut files = vec![file_name];

        files.extend(config_dir_file);
        files
    }

    fn file_sources(&self) -> Figment {
        let mut figment = Figment::new();

        for name in self.sources() {
            figment = figment.merge(Yaml::file(format!("{name}.yaml")));
        }

        for name in self.sources() {
            figment = figment.merge(Json::file(format!("{name}.json")));
        }

        for name in self.sources() {
            figment = figment.merge(Toml::file(format!("{name}.toml")));
        }

        figment
    }
}

impl Provider for Source {
    fn metadata(&self) -> Metadata {
        let basename = format!("file_basename = {}", self.file_basename);
        let prefix = format!("env_prefix = {}", self.env_prefix);
        let label = format!("Source({basename}, {prefix})");

        Metadata::named(label)
    }

    fn data(&self) -> FigmentDataResult {
        self.figment().data()
    }
}
