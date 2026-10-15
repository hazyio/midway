use std::{fs, path::Path};

use crate::token_config::TokenConfig;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub tokens: TokenConfig,
}

impl AppConfig {
    pub fn read_from(path: &Path) -> Self {
        match fs::read_to_string(path) {
            Ok(contents) => match Self::from_yaml(&contents) {
                Ok(config) => config,
                Err(err) => panic!("Failed to parse config: {}", err),
            },
            Err(err) => panic!("Failed to read config file: {}", err),
        }
    }
    fn from_yaml(yaml: &str) -> Result<Self, noyalib::Error> {
        noyalib::from_str(yaml)
    }
}
