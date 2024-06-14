//! A connector that fetches data from the HTS service and dumps it into `InfluxDB`.

fn main() {
    use dotenv::dotenv;
    use std::env;

    dotenv().unwrap();

    let influxdb_url = env::var("INFLUXDB_URL");
    let influxdb_bucket = env::var("INFLUXDB_BUCKET");
    let influxdb_token = env::var("INFLUXDB_TOKEN");
    let influxdb_org = env::var("INFLUXDB_ORG");
    let text_file_path = env::var("TEXT_FILE_PATH");

    #[cfg(not(debug_assertions))]
    {
        if influxdb_url.is_err() {
            panic!("INFLUXDB_URL must be set");
        }

        if influxdb_bucket.is_err() {
            panic!("INFLUXDB_BUCKET must be set");
        }

        if influxdb_token.is_err() {
            panic!("INFLUXDB_TOKEN must be set");
        }

        if influxdb_org.is_err() {
            panic!("INFLUXDB_ORG must be set");
        }

        if text_file_path.is_err() {
            panic!("TEXT_FILE_PATH must be set");
        }
    }

    println!("cargo:rustc-env=INFLUXDB_URL={}", influxdb_url.unwrap());
    println!("cargo:rustc-env=INFLUXDB_BUCKET={}", influxdb_bucket.unwrap());
    println!("cargo:rustc-env=INFLUXDB_TOKEN={}", influxdb_token.unwrap());
    println!("cargo:rustc-env=INFLUXDB_ORG={}", influxdb_org.unwrap());
    println!("cargo:rustc-env=TEXT_FILE_PATH={}", text_file_path.unwrap());
}
