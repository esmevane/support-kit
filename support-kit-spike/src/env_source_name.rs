use convert_case::{Case, Casing};

use crate::{Environment, Source, SourceName};

pub struct EnvSourceName(SourceName, Environment);

impl EnvSourceName {
    pub fn new(name: SourceName, env: Environment) -> Self {
        Self(name, env)
    }

    pub fn source(&self) -> Source {
        Source::new(self.file_name(), self.env_prefix())
    }

    fn env(&self) -> String {
        self.1.clone().into_inner()
    }

    fn name(&self) -> String {
        self.0.clone().into_inner()
    }

    pub fn file_name(&self) -> String {
        let name = self.name().to_case(Case::Kebab);
        let env = self.env().to_string().to_case(Case::Kebab);

        format!("{name}.{env}")
    }

    pub fn env_prefix(&self) -> String {
        let name = self.name().to_case(Case::ScreamingSnake);
        let env = self.env().to_string().to_case(Case::ScreamingSnake);

        format!("{name}_{env}")
    }
}

#[test]
fn using_named_sources() -> Result<(), Box<dyn std::error::Error>> {
    let named_source = SourceName::try_new("My Cool Source")?;
    let env = Environment::try_new("development")?;
    let label = EnvSourceName::new(named_source, env);

    assert_eq!(label.name(), "my cool source");
    assert_eq!(label.env(), "development");
    assert_eq!(label.file_name(), "my-cool-source.development");
    assert_eq!(label.env_prefix(), "MY_COOL_SOURCE_DEVELOPMENT");

    Ok(())
}
