use convert_case::{Case, Casing};
use nutype::nutype;
use std::str::FromStr;

use crate::Source;

#[nutype(
    sanitize(trim, lowercase),
    validate(not_empty),
    derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq),
    default = SourceName::DEFAULT_NAME
)]
pub struct SourceName(String);

impl SourceName {
    pub const DEFAULT_NAME: &'static str = "support-kit";

    pub fn name(&self) -> String {
        self.clone().into_inner()
    }

    pub fn source(&self) -> Source {
        Source::new(self.file_name(), self.env_prefix())
    }

    pub fn file_name(&self) -> String {
        self.name().to_case(Case::Kebab)
    }

    pub fn env_prefix(&self) -> String {
        self.name().to_case(Case::ScreamingSnake)
    }
}

impl FromStr for SourceName {
    type Err = SourceNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_new(s)
    }
}

#[test]
fn using_named_sources() -> Result<(), Box<dyn std::error::Error>> {
    let named_source = SourceName::try_new("My Cool Source")?;

    assert_eq!(named_source.name(), "my cool source");
    assert_eq!(named_source.file_name(), "my-cool-source");
    assert_eq!(named_source.env_prefix(), "MY_COOL_SOURCE");

    Ok(())
}
