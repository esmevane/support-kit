// Either a configuration (struct) or a preset (enum).
#[derive(Clone, Debug, serde::Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ConfigOrPreset<Config, Preset> {
    Config(Config),
    Preset(Preset),
}
