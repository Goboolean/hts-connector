use serde::Deserialize;
use envconfig::Envconfig;
use dotenv::dotenv;


#[derive(Debug, Deserialize, Envconfig)]
pub struct Config {
    #[envconfig(from = "INFLUXDB_URL")]
    pub url: String,
    #[envconfig(from = "INFLUXDB_TOKEN")]
    pub token: String,
    #[envconfig(from = "INFLUXDB_ORG")]
    pub org: String,
    #[envconfig(from = "INFLUXDB_BUCKET")]
    pub bucket: String
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