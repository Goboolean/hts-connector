use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};
use std::thread;
use std::time::{Duration, Instant};

use crate::file::config::Config;
use crate::model::candle::Candle;

use mockall::*;
use mockall::predicate::*;



#[automock]
pub trait Handler: Send + Sync  {
    fn handle(&self, candle: Candle);
}

struct Reader {
    path: String,
    handler: Box<dyn Handler>,
}

impl Reader {
    pub fn new(config: Config, handler: Box<dyn Handler>) -> io::Result<Self> {
        File::open(&config.path)?;
        Ok(Reader { path: config.path, handler })
    }

    fn read_and_follow(&self, duration: Duration) -> io::Result<()> {
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

                if parts.len() != 6 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Expected 6 parts, found {}", parts.len())
                    ));
                }
                
                let candle = Candle {
                    timestamp: parts[0].parse().unwrap(),
                    name: parts[1].parse().unwrap(),
                    open: parts[2].parse().unwrap(),
                    high: parts[3].parse().unwrap(),
                    low: parts[4].parse().unwrap(),
                    close: parts[5].parse().unwrap(),
                };

                self.handler.handle(candle);
            }

            if start.elapsed() >= duration {
                break;
            }

            if eof_reached {
                thread::sleep(Duration::from_secs(1));
                reader.seek(SeekFrom::Current(0))?;
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
            "1 BTCUSDT 100.0 200.0 50.0 150.0",
            "2 BTCUSDT 150.0 250.0 100.0 200.0",
            "3 BTCUSDT 200.0 300.0 150.0 250.0",
        ];
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true) // Create the file if it doesn't exist
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
                timestamp: 1,
                name: "BTCUSDT".to_string(),
                open: 100.0,
                high: 200.0,
                low: 50.0,
                close: 150.0,
            },
            Candle {
                timestamp: 2,
                name: "BTCUSDT".to_string(),
                open: 150.0,
                high: 250.0,
                low: 100.0,
                close: 200.0,
            },
            Candle {
                timestamp: 3,
                name: "BTCUSDT".to_string(),
                open: 200.0,
                high: 300.0,
                low: 150.0,
                close: 250.0,
            },
        ];

        let mut mock_handler = MockHandler::new();
        mock_handler.expect_handle()
            .with(eq(candles[0].clone()))
            .times(1)
            .return_const(());
        mock_handler.expect_handle()
            .with(eq(candles[1].clone()))
            .times(1)
            .return_const(());
        mock_handler.expect_handle()
            .with(eq(candles[2].clone()))
            .times(1)
            .return_const(());

        let config = Config::new().expect("Failed to create config");
        let reader = Reader::new(config,  Box::new(mock_handler)).expect("Failed to create reader");
        let duration = Duration::from_secs(1);

        // Act
        let result = reader.read_and_follow(duration);

        // Assert
        assert!(result.is_ok());
    }
}