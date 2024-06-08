use influxdb::Client as InfluxClient;
use influxdb::WriteQuery;
use crate::model::candle::Candle;
use crate::influx::config::Config;

struct Client {
    client: InfluxClient,
}


impl Client {
    pub fn new(config: Config) -> Self {
        let client = InfluxClient::new(config.url, config.bucket).with_token(config.token);
        Client { client }
    }

    pub async fn insert_data(&self, candle: Candle) -> Result<(), influxdb::Error> {
        let write_query = WriteQuery::new(influxdb::Timestamp::Seconds(candle.timestamp), candle.name)
            .add_field("open", candle.open)
            .add_field("high", candle.high)
            .add_field("low", candle.low)
            .add_field("close", candle.close);

        self.client.query(&write_query).await.map(|_| ())
    }
}


#[cfg(test)]
#[cfg(not(target_os = "windows"))]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_insert_data() {
        // Arrange
        let config = Config::new().expect("Failed to create config");
        let client = Client::new(config);

        let candle = Candle {
            timestamp: Utc::now().timestamp() as u128,
            name: "BTCUSDT".to_string(),
            open: 100.0,
            high: 200.0,
            low: 50.0,
            close: 150.0,
        };

        // Act
        let result = client.insert_data(candle).await;

        println!("{:?}", result);

        // Assert
        assert!(result.is_ok());
    }
}