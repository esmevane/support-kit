use std::path::PathBuf;

use minijinja::Environment;

use crate::{BoilerplateError, Configuration};

pub struct BoilerplateTemplate {
    path: PathBuf,
    file_name: String,
    source: String,
}

impl BoilerplateTemplate {
    pub fn new(path: impl AsRef<str>, file_name: impl AsRef<str>, source: impl AsRef<str>) -> Self {
        Self {
            path: PathBuf::from(path.as_ref()),
            file_name: file_name.as_ref().to_string(),
            source: source.as_ref().to_string(),
        }
    }

    pub fn key(&self) -> String {
        self.file().display().to_string()
    }

    pub fn file(&self) -> PathBuf {
        self.path.join(&self.file_name)
    }

    pub fn write(&self, config: &Configuration) -> Result<(), BoilerplateError> {
        let mut env = Environment::new();
        let key = self.key();

        env.add_template(&key, &self.source)?;

        std::fs::create_dir_all(&self.path)?;
        std::fs::write(self.file(), env.get_template(&key)?.render(&config)?)?;

        Ok(())
    }
}
