use envconfig::Envconfig;
use serde::Deserialize;
use std::error::Error;
use std::env;


#[derive(Debug, Deserialize, Envconfig)]
pub struct Config {
    pub path: String,
}

impl Config {
    fn retrieve_env_var(key: &str) -> Result<String, Box<dyn Error>> {
        env::var(key).map_err(|e| {
            Box::new(e) as Box<dyn Error>
        })
    }

    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            path: Self::retrieve_env_var("TEXT_FILE_PATH")?,
        })
    }

    pub fn init() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            path: env!("TEXT_FILE_PATH").to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[ignore]
    #[test]
    fn test_config_fail() {
        // Arrange
        env::remove_var("TEXT_FILE_PATH");

        // Act
        let config = Config::new();

        // Assert
        assert!(config.is_err());
    }

    #[ignore]
    #[test]
    fn test_influx_config_success() {
        // Arrange
        const TEXT_FILE_PATH: &str = "./text.txt";
        env::set_var("TEXT_FILE_PATH", TEXT_FILE_PATH);

        // Act
        let config = Config::new().expect("Failed to create config");

        // Assert
        assert_eq!(config.path, TEXT_FILE_PATH);
    }
}
