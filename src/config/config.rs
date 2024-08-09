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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_from_file_valid() {
        // Create a temporary file with a valid TOML content
        let mut temp_file = NamedTempFile::new().unwrap();
        let toml_content = r#"
            solana_rpc_url = "https://api.devnet.solana.com"
            solana_ws_url = "wss://api.devnet.solana.com"
            api_bind_address = "127.0.0.1:8080"
            database_url = "postgres://user:password@localhost/db"
            transaction_signature = "5N7k3w3Asd5Lk2D8"
            account_pubkey = "6k3MnB5z3Q2N7E"
            port = "8080"
        "#;

        temp_file.write_all(toml_content.as_bytes()).unwrap();

        // Test the from_file function
        let config = Config::from_file(temp_file.path().to_str().unwrap()).unwrap();

        assert_eq!(config.solana_rpc_url, "https://api.devnet.solana.com");
        assert_eq!(config.solana_ws_url, "wss://api.devnet.solana.com");
        assert_eq!(config.api_bind_address, "127.0.0.1:8080");
        assert_eq!(
            config.database_url.as_deref(),
            Some("postgres://user:password@localhost/db")
        );
        assert_eq!(config.transaction_signature, "5N7k3w3Asd5Lk2D8");
        assert_eq!(config.account_pubkey, "6k3MnB5z3Q2N7E");
        assert_eq!(config.port, "8080");
    }

    #[test]
    fn test_config_from_file_invalid() {
        // Create a temporary file with invalid TOML content
        let mut temp_file = NamedTempFile::new().unwrap();
        let toml_content = r#"
            solana_rpc_url = "https://api.devnet.solana.com"
            solana_ws_url = "wss://api.devnet.solana.com"
            api_bind_address = 127.0.0.1:8080   # Invalid: Missing quotes
            transaction_signature = "5N7k3w3Asd5Lk2D8"
            account_pubkey = "6k3MnB5z3Q2N7E"
            port = "8080"
        "#;

        temp_file.write_all(toml_content.as_bytes()).unwrap();

        // Test the from_file function for error handling
        let result = Config::from_file(temp_file.path().to_str().unwrap());

        assert!(result.is_err());
    }

    #[test]
    fn test_config_from_file_missing_fields() {
        // Create a temporary file with missing fields in TOML content
        let mut temp_file = NamedTempFile::new().unwrap();
        let toml_content = r#"
            solana_rpc_url = "https://api.devnet.solana.com"
            api_bind_address = "127.0.0.1:8080"
            transaction_signature = "5N7k3w3Asd5Lk2D8"
            account_pubkey = "6k3MnB5z3Q2N7E"
            port = "8080"
        "#;

        temp_file.write_all(toml_content.as_bytes()).unwrap();

        // Test the from_file function when fields are missing
        let result = Config::from_file(temp_file.path().to_str().unwrap());

        assert!(result.is_err());
    }
}
