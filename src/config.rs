use std::fs;
use serde_yaml;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub structure: Folder,
}

#[derive(Debug, Deserialize)]
pub struct Folder {
    #[serde(default)]
    pub files: Vec<String>,

    #[serde(default)]
    pub folders: std::collections::HashMap<String, Folder>,
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}