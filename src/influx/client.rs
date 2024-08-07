use crate::influx::config::Config;
use crate::model::{candle::Candle, indicator::Indicator};
use influxdb::Client as InfluxClient;
use influxdb::WriteQuery;

pub struct Client {
    client: InfluxClient,
}

impl Client {
    pub async fn new(config: Config) -> Result<Self, influxdb::Error> {
        let influx_client =
            InfluxClient::new(&config.url, &config.bucket).with_token(&config.token);
        let client = Self {
            client: influx_client,
        };

        client.ping().await?;

        Ok(client)
    }

    pub async fn ping(&self) -> Result<(), influxdb::Error> {
        self.client.ping().await.map(|_| ())
    }

    pub async fn insert_candle(&self, candle: Candle) -> Result<(), influxdb::Error> {
        let point =
            WriteQuery::new(influxdb::Timestamp::Seconds(candle.timestamp), candle.event)
                .add_field("open", candle.open)
                .add_field("high", candle.high)
                .add_field("low", candle.low)
                .add_field("close", candle.close);

        self.client.query(&point).await.map(|_| ())
    }

    pub async fn insert_candles(&self, candles: Vec<Candle>) -> Result<(), influxdb::Error> {
        let points: Vec<WriteQuery> = candles.into_iter().map(|candle| {
            WriteQuery::new(influxdb::Timestamp::Seconds(candle.timestamp), candle.event)
                .add_field("open", candle.open)
                .add_field("high", candle.high)
                .add_field("low", candle.low)
                .add_field("close", candle.close)
            }).collect();
    
        self.client.query(&points).await.map(|_| ())
    }

    pub async fn insert_indicator(&self, indicator: Indicator) -> Result<(), influxdb::Error> {
        let point = WriteQuery::new(
            influxdb::Timestamp::Seconds(indicator.timestamp),
            indicator.event,
        )
        .add_field(indicator.property, indicator.value);

        self.client.query(&point).await.map(|_| ())
    }

    pub async fn insert_indicators(&self, indicators: Vec<Indicator>) -> Result<(), influxdb::Error> {
        let points: Vec<WriteQuery> = indicators.into_iter().map(|indicator| {
            WriteQuery::new(influxdb::Timestamp::Seconds(indicator.timestamp), indicator.event)
                .add_field(indicator.property, indicator.value)
            }).collect();
    
        self.client.query(&points).await.map(|_| ())
    }
}

#[cfg(test)]
#[cfg(not(target_os = "windows"))]
#[allow(clippy::unreadable_literal)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_insert_candle() {
        // Arrange
        let config = Config::new().expect("Failed to create config");        
        let client = Client::new(config).await.expect("Failed to create client");

        let candle = Candle {
            timestamp: 1714450980,
            event: "BTCUSDT".to_string(),
            open: 100.0,
            high: 200.0,
            low: 50.0,
            close: 150.0,
        };

        // Act
        let result = client.insert_candle(candle).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_insert_candles() {
        // Arrange
        let config = Config::new().expect("Failed to create config");
        let client = Client::new(config).await.expect("Failed to create client");

        let candles = vec![
            Candle {
                timestamp: 1714450980,
                event: "BTCUSDT".to_string(),
                open: 100.0,
                high: 200.0,
                low: 50.0,
                close: 150.0,
            },
            Candle {
                timestamp: 1714451980,
                event: "BTCUSDT".to_string(),
                open: 100.0,
                high: 200.0,
                low: 50.0,
                close: 150.0,
            },
        ];

        // Act
        let result = client.insert_candles(candles).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_insert_indicator() {
        // Arrange
        let config = Config::new().expect("Failed to create config");
        let client = Client::new(config).await.expect("Failed to create client");

        let indicator = Indicator {
            timestamp: 1714450980,
            event: "BTCUSDT".to_string(),
            property: "rsi".to_string(),
            value: 70,
        };

        // Act
        let result = client.insert_indicator(indicator).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_insert_indicators() {
        // Arrange
        let config = Config::new().expect("Failed to create config");
        let client = Client::new(config).await.expect("Failed to create client");

        let indicators = vec![
            Indicator {
                timestamp: 1714450980,
                event: "BTCUSDT".to_string(),
                property: "rsi".to_string(),
                value: 70,
            },
            Indicator {
                timestamp: 1714451980,
                event: "BTCUSDT".to_string(),
                property: "rsi".to_string(),
                value: 70,
            },
        ];

        // Act
        let result = client.insert_indicators(indicators).await;

        // Assert
        assert!(result.is_ok());
    }
}
