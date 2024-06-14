use hts_connector::influx::adapter::InfluxHandler;
use hts_connector::influx::client::Client as InfluxClient;
use hts_connector::influx::config::Config as InfluxConfig;
use hts_connector::text::config::Config as TextConfig;
use hts_connector::text::reader::Reader as TextReader;

use std::time::Duration;
use tokio::runtime::Runtime;

fn main() {
    let path: &'static str = env!("INFLUXDB_URL");
    println!("the $PATH variable at the time of compiling was: {path}");

    let runtime = Runtime::new().expect("Failed to create runtime");

    let client = runtime.block_on(async {
        let config = InfluxConfig::init().expect("Failed to create config");
        InfluxClient::new(config)
            .await
            .expect("Failed to create client")
    });
    let adapter = InfluxHandler::new(client);

    let config = TextConfig::init().expect("Failed to create config");
    let reader = TextReader::new(config, Box::new(adapter)).expect("Failed to create reader");

    let one_day = Duration::from_secs(24 * 60 * 60);
    reader
        .read_and_follow(one_day)
        .expect("Failed to read and follow");
}
