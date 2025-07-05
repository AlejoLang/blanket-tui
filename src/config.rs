use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub sound: Vec<SoundConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SoundConfig {
    pub name: String,
    pub file: String,
    pub icon: String,
}