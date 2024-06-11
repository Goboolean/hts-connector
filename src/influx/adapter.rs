use crate::influx::client::Client as InfluxClient;
use crate::model::candle::Candle;
use crate::text::reader::Handler;
use std::io;
use tokio::runtime::Runtime;

pub struct InfluxHandler {
    client: InfluxClient,
}

impl InfluxHandler {
    #[must_use]
    pub const fn new(client: InfluxClient) -> Self {
        Self { client }
    }
}

impl Handler for InfluxHandler {
    fn handle_candle(&self, candle: Candle) -> Result<(), io::Error> {
        let runtime = Runtime::new().map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to create runtime: {e}"),
            )
        })?;
        match runtime.block_on(self.client.insert_candle(candle)) {
            Ok(()) => Ok(()),
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to insert data: {e}"),
            )),
        }
    }

    fn handle_indicator(
        &self,
        indicator: crate::model::indicator::Indicator,
    ) -> Result<(), io::Error> {
        let runtime = Runtime::new().map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to create runtime: {e}"),
            )
        })?;
        match runtime.block_on(self.client.insert_indicator(indicator)) {
            Ok(()) => Ok(()),
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to insert data: {e}"),
            )),
        }
    }
}
