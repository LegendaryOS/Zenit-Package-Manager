use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Default)]
pub struct Config {
    pub settings: Settings,
}

#[derive(Deserialize)]
pub struct Settings {
    pub default_manager: String,
    pub confirm: bool,
    pub progress_style: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            default_manager: "pacman".to_string(),
            confirm: false,
            progress_style: "fancy".to_string(),
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Self, toml::de::Error> {
        let contents = fs::read_to_string(path)?;
        toml::from_str(&contents)
    }
}
