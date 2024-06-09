//! A connector that fetches data from the HTS service and dumps it into `InfluxDB`.

fn main() {
    #[cfg(not(debug_assertions))]
    {
        use dotenv::dotenv;
        use std::env;

        dotenv().expect("Failed to read .env file");

        env::var("INFLUXDB_URL").expect("INFLUX_URL must be set");
        env::var("INFLUXDB_BUCKET").expect("INFLUX_BUCKET must be set");
        env::var("INFLUXDB_TOKEN").expect("INFLUX_TOKEN must be set");
        env::var("INFLUXDB_ORG").expect("INFLUX_ORG must be set");
        env::var("TEXT_FILE_PATH").expect("TEXT_FILE_PATH must be set");
    }
}
