use serde::Deserialize;
use std::{error::Error, fs};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub solana_rpc_url:        String,
    pub solana_ws_url:         String,
    pub api_bind_address:      String,
    pub database_url:          Option<String>,
    pub transaction_signature: String,
    pub account_pubkey:        String,
    pub port:                  String,
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let config_string = fs::read_to_string(file_path)?;
        let config: Config = toml::from_str(&config_string)?;
        Ok(config)
    }
}
