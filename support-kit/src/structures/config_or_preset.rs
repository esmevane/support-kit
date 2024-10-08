use serde::{Deserialize, Serialize};

// Either a configuration (struct) or a preset (enum).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ConfigOrPreset<Config, Preset> {
    Config(Config),
    Preset(Preset),
}
