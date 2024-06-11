//! A connector that fetches data from the HTS service and dumps it into `InfluxDB`.

fn main() {
    #[cfg(not(debug_assertions))]
    {
        use dotenv::dotenv;
        use std::env;

        dotenv().expect("Failed to read .env file");

        let influxdb_url = env::var("INFLUXDB_URL").expect("INFLUX_URL must be set");
        let influxdb_bucket = env::var("INFLUXDB_BUCKET").expect("INFLUX_BUCKET must be set");
        let influxdb_token = env::var("INFLUXDB_TOKEN").expect("INFLUX_TOKEN must be set");
        let influxdb_org = env::var("INFLUXDB_ORG").expect("INFLUX_ORG must be set");
        let text_file_path = env::var("TEXT_FILE_PATH").expect("TEXT_FILE_PATH must be set");

        println!("cargo:rustc-env=INFLUXDB_URL={}", influxdb_url);
        println!("cargo:rustc-env=INFLUXDB_BUCKET={}", influxdb_bucket);
        println!("cargo:rustc-env=INFLUXDB_TOKEN={}", influxdb_token);
        println!("cargo:rustc-env=INFLUXDB_ORG={}", influxdb_org);
        println!("cargo:rustc-env=TEXT_FILE_PATH={}", text_file_path);
    }
}
