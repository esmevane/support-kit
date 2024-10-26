use std::path::PathBuf;

use minijinja::Environment;

use crate::BoilerplateError;

use super::BoilerplateContext;

#[derive(Debug, Clone, bon::Builder)]
pub struct BoilerplateTemplate {
    #[builder(into, default)]
    path: PathBuf,
    #[builder(into)]
    file_name: String,
    #[builder(default, into)]
    source: String,
}

impl BoilerplateTemplate {
    pub fn key(&self) -> String {
        self.file().display().to_string()
    }

    pub fn file(&self) -> PathBuf {
        self.path.join(&self.file_name)
    }

    pub fn write(&self, config: &BoilerplateContext) -> Result<(), BoilerplateError> {
        let mut env = Environment::new();
        let key = self.key();

        env.add_template(&key, &self.source)?;

        std::fs::create_dir_all(&self.path)?;
        std::fs::write(self.file(), env.get_template(&key)?.render(&config)?)?;

        Ok(())
    }
}
