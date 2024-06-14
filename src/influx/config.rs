use envconfig::Envconfig;
use serde::Deserialize;

use std::error::Error;
use std::env;

#[derive(Debug, Deserialize, Envconfig)]
pub struct Config {
    pub url: String,
    pub token: String,
    pub org: String,
    pub bucket: String,
}

impl Config {
    fn retrieve_env_var(key: &str) -> Result<String, Box<dyn Error>> {
        env::var(key).map_err(|e| {
            Box::new(e) as Box<dyn Error>
        })
    }

    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            url: Self::retrieve_env_var("INFLUXDB_URL")?,
            token: Self::retrieve_env_var("INFLUXDB_TOKEN")?,
            org: Self::retrieve_env_var("INFLUXDB_ORG")?,
            bucket: Self::retrieve_env_var("INFLUXDB_BUCKET")?,
        })
    }

    pub fn init() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            url: env!("INFLUXDB_URL").to_string(),
            token: env!("INFLUXDB_TOKEN").to_string(),
            org: env!("INFLUXDB_ORG").to_string(),
            bucket: env!("INFLUXDB_BUCKET").to_string(),
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
        env::remove_var("INFLUXDB_URL");
        env::remove_var("INFLUXDB_TOKEN");
        env::remove_var("INFLUXDB_ORG");
        env::remove_var("INFLUXDB_BUCKET");

        // Act
        let config = Config::new();

        // Assert
        assert!(config.is_err());
    }

    #[ignore]
    #[test]
    fn test_influx_config_success() {
        // Arrange
        const INFLUXDB_URL: &str = "http://localhost:8086";
        const INFLUXDB_TOKEN: &str = "token";
        const INFLUXDB_ORG: &str = "goboolean";
        const INFLUXDB_BUCKET: &str = "sample-bucket";

        env::set_var("INFLUXDB_URL", INFLUXDB_URL);
        env::set_var("INFLUXDB_TOKEN", INFLUXDB_TOKEN);
        env::set_var("INFLUXDB_ORG", INFLUXDB_ORG);
        env::set_var("INFLUXDB_BUCKET", INFLUXDB_BUCKET);

        // Act
        let config = Config::new().expect("Failed to create config");

        // Assert
        assert_eq!(config.url, INFLUXDB_URL);
        assert_eq!(config.token, INFLUXDB_TOKEN);
        assert_eq!(config.org, INFLUXDB_ORG);
        assert_eq!(config.bucket, INFLUXDB_BUCKET);
    }
}
