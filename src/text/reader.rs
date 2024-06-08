use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Seek};
use std::thread;
use std::time::{Duration, Instant};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use crate::text::config::Config;
use crate::model::candle::Candle;

use mockall::*;
use mockall::predicate::*;
use chrono_tz::Asia::Seoul;




#[automock]
pub trait Handler: Send + Sync  {
    fn handle(&self, candle: Candle) -> Result<(), io::Error>;
}

pub struct Reader {
    path: String,
    handler: Box<dyn Handler>,
}

impl Reader {
    pub fn new(config: Config, handler: Box<dyn Handler>) -> io::Result<Self> {
        match File::open(&config.path) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        Ok(Reader { path: config.path, handler })
    }

    pub fn read_and_follow(&self, duration: Duration) -> io::Result<()> {
        let start = Instant::now();

        let file = OpenOptions::new()
            .read(true)
            .open(&self.path)?;
        let mut reader = BufReader::new(file);

        loop {
            let mut eof_reached = false;

            while let Some(line) = {
                let mut buffer = String::new();
                match reader.read_line(&mut buffer) {
                    Ok(0) => {
                        eof_reached = true;
                        None
                    }
                    Ok(_) => Some(buffer),
                    Err(e) => return Err(e),
                }
            } {
                let parts: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

                if parts.len() != 7 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Expected 6 parts, found {}", parts.len())
                    ));
                }

                let date = NaiveDate::parse_from_str(&parts[0], "%Y-%m-%d").expect("Failed to parse date");
                let time = NaiveTime::parse_from_str(&parts[1], "%H:%M:%S").expect("Failed to parse time");
                let datetime: NaiveDateTime = NaiveDateTime::new(date, time);

                let candle = Candle {
                    timestamp: datetime.and_local_timezone(Seoul).unwrap().timestamp() as u128,
                    name: parts[2].parse().unwrap(),
                    open: parts[3].parse().unwrap(),
                    high: parts[4].parse().unwrap(),
                    low: parts[5].parse().unwrap(),
                    close: parts[6].parse().unwrap(),
                };
                
                match self.handler.handle(candle) {
                    Ok(_) => (),
                    Err(e) => return Err(e),                    
                }
            }

            if start.elapsed() >= duration {
                break;
            }

            if eof_reached {
                thread::sleep(Duration::from_secs(1));
                match reader.stream_position() {
                    Ok(_) => (),
                    Err(e) => return Err(e),                    
                }
            }
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::env;
    use std::fs::OpenOptions;
    use std::io::Write;
    use scopeguard::defer;



    #[ignore]
    #[test]
    fn test_new_fail_if_no_file_exists() {
        // Arrange
        const TEXT_FILE_PATH: &str = "tests/example.txt";
        env::set_var("TEXT_FILE_PATH", TEXT_FILE_PATH);

        let mut mock_handler = MockHandler::new();
        mock_handler.expect_handle()
            .times(0);


        // Act
        let config = Config::new().expect("Failed to create config");
        let reader: Result<Reader, io::Error> = Reader::new(config,  Box::new(mock_handler));

        // Assert
        assert!(reader.is_err());
    }

    #[ignore]
    #[test]
    fn test_new_success_if_file_exists() {
        // Arrange
        const TEXT_FILE_PATH: &str = "tests/example.txt";
        env::set_var("TEXT_FILE_PATH", TEXT_FILE_PATH);
        let file = File::create(TEXT_FILE_PATH).expect("Failed to create file");

        let mut mock_handler = MockHandler::new();
        mock_handler.expect_handle()
            .times(0);

        // Act
        let config = Config::new().expect("Failed to create config");
        let reader = Reader::new(config,  Box::new(mock_handler));

        // Assert
        assert!(reader.is_ok());

        // Cleanup
        drop(file);
        std::fs::remove_file(TEXT_FILE_PATH).expect("Failed to remove file");
    }

    #[ignore]
    #[test]
    fn test_read_data() {
        // Arrange
        const TEXT_FILE_PATH: &str = "tests/example.txt";
        env::set_var("TEXT_FILE_PATH", TEXT_FILE_PATH);

        let datas: [&str; 3] = [
            "2024-04-30 13:21:00  test 368.850000 368.900000 368.750000 368.700000",
            "2024-04-30 13:22:00  test 368.800000 368.800000 368.700000 368.650000",
            "2024-04-30 13:23:00  test 368.750000 368.850000 368.800000 368.750000",
        ];
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(TEXT_FILE_PATH)
            .expect("Failed to open file");

        for data in datas.iter() {
            writeln!(file, "{}", data).expect("Failed to write to file");
        }

        defer! {
            std::fs::remove_file(TEXT_FILE_PATH).expect("Failed to remove file");
        }

        let candles: [Candle; 3] = [
            Candle {
                timestamp: 1714450860,
                name: "test".to_string(),
                open: 368.850000,
                high: 368.900000,
                low: 368.750000,
                close: 368.700000,
            },
            Candle {
                timestamp: 1714450920,
                name: "test".to_string(),
                open: 368.800000,
                high: 368.800000,
                low: 368.700000,
                close: 368.650000,
            },
            Candle {
                timestamp: 1714450980,
                name: "test".to_string(),
                open: 368.750000,
                high: 368.850000,
                low: 368.800000,
                close: 368.750000,
            },
        ];

        let mut mock_handler = MockHandler::new();
        mock_handler.expect_handle()
            .with(eq(candles[0].clone()))
            .times(1)
            .returning(|_| Ok(()));
        mock_handler.expect_handle()
            .with(eq(candles[1].clone()))
            .times(1)
            .returning(|_| Ok(()));
        mock_handler.expect_handle()
            .with(eq(candles[2].clone()))
            .times(1)
            .returning(|_| Ok(()));

        let config = Config::new().expect("Failed to create config");
        let reader = Reader::new(config,  Box::new(mock_handler)).expect("Failed to create reader");
        let duration = Duration::from_secs(1);

        // Act
        let result = reader.read_and_follow(duration);

        // Assert
        assert!(result.is_ok());
    }
}