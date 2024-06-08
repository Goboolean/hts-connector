use serde::Deserialize;
use envconfig::Envconfig;
use dotenv::dotenv;


#[derive(Debug, Deserialize, Envconfig)]
pub struct Config {
    #[envconfig(from = "TEXT_FILE_PATH")]
    pub path: String
}

impl Config {
    pub fn new() -> Result<Self, envconfig::Error> {
        dotenv().ok();
        Self::init_from_env()
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
        let config: Result<Config, envconfig::Error> = Config::new();

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