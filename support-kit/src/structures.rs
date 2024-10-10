mod config_or_preset {
    use serde::{Deserialize, Serialize};

    // Either a configuration (struct) or a preset (enum).
    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
    #[serde(untagged)]
    pub enum ConfigOrPreset<Config, Preset> {
        Config(Config),
        Preset(Preset),
    }
}

mod one_or_many {

    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    #[serde(untagged)]
    pub enum OneOrMany<Contents> {
        Many(Vec<Contents>),
        One(Contents),
    }

    impl<Contents> From<Vec<Contents>> for OneOrMany<Contents> {
        fn from(value: Vec<Contents>) -> Self {
            Self::Many(value)
        }
    }

    impl<Contents> From<Contents> for OneOrMany<Contents> {
        fn from(value: Contents) -> Self {
            Self::One(value)
        }
    }
}

pub use config_or_preset::ConfigOrPreset;
pub use one_or_many::OneOrMany;
