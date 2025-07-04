use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub sound: Vec<SoundConfig>,
}

#[derive(Debug, Deserialize)]
pub struct SoundConfig {
    pub name: String,
    pub file: String,
    pub icon: String,
}