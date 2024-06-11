use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Seek};
use std::thread;
use std::time::{Duration, Instant};

use crate::model::{candle::Candle, indicator::Indicator};
use crate::text::config::Config;
use crate::text::parser::{parse_candle, parse_indicator};

use mockall::automock;

#[automock]
pub trait Handler: Send + Sync {
    fn handle_candle(&self, candle: Candle) -> Result<(), io::Error>;
    fn handle_indicator(&self, indicator: Indicator) -> Result<(), io::Error>;
}

pub struct Reader {
    path: String,
    handler: Box<dyn Handler>,
}

impl Reader {
    pub fn new(config: Config, handler: Box<dyn Handler>) -> Result<Self, io::Error> {
        match File::open(&config.path) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        Ok(Self {
            path: config.path,
            handler,
        })
    }

    #[allow(clippy::unreadable_literal)]
    #[allow(clippy::cast_sign_loss)]
    pub fn read_and_follow(&self, duration: Duration) -> Result<(), io::Error> {
        let start = Instant::now();

        let file = OpenOptions::new().read(true).open(&self.path)?;
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
                match (parse_candle(line.clone()), parse_indicator(line.clone())) {
                    (Ok(candle), _) => {
                        self.handler.handle_candle(candle)?;
                    }
                    (_, Ok(indicator)) => {
                        self.handler.handle_indicator(indicator)?;
                    }
                    (Err(_), Err(_)) => {
                    }
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

    use mockall::predicate::eq;
    use scopeguard::defer;
    use std::env;
    use std::fs::OpenOptions;
    use std::io::Write;

    #[ignore]
    #[test]
    fn test_new_fail_if_no_file_exists() {
        // Arrange
        const TEXT_FILE_PATH: &str = "tests/example.txt";
        env::set_var("TEXT_FILE_PATH", TEXT_FILE_PATH);

        let mut mock_handler = MockHandler::new();
        mock_handler.expect_handle_candle().times(0);
        mock_handler.expect_handle_indicator().times(0);

        // Act
        let config = Config::new().expect("Failed to create config");
        let reader: Result<Reader, io::Error> = Reader::new(config, Box::new(mock_handler));

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
        mock_handler.expect_handle_candle().times(0);
        mock_handler.expect_handle_indicator().times(0);

        // Act
        let config = Config::new().expect("Failed to create config");
        let reader = Reader::new(config, Box::new(mock_handler));

        // Assert
        assert!(reader.is_ok());

        // Cleanup
        drop(file);
        std::fs::remove_file(TEXT_FILE_PATH).expect("Failed to remove file");
    }

    #[ignore]
    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_read_data_candle() {
        // Arrange
        const TEXT_FILE_PATH: &str = "tests/example.txt";
        env::set_var("TEXT_FILE_PATH", TEXT_FILE_PATH);

        let datas: [&str; 3] = [
            "2024-04-30 13:21:00  테스트 368.850000 368.900000 368.750000 368.700000",
            "2024-04-30 13:22:00  테스트 368.800000 368.800000 368.700000 368.650000",
            "2024-04-30 13:23:00  테스트 368.750000 368.850000 368.800000 368.750000",
        ];
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(TEXT_FILE_PATH)
            .expect("Failed to open file");

        for data in &datas {
            writeln!(file, "{data}").expect("Failed to write to file");
        }

        defer! {
            std::fs::remove_file(TEXT_FILE_PATH).expect("Failed to remove file");
        }

        let candles: [Candle; 3] = [
            Candle {
                timestamp: 1714450860,
                event: "테스트".to_string(),
                open: 368.850000,
                high: 368.900000,
                close: 368.750000,
                low: 368.700000,
            },
            Candle {
                timestamp: 1714450920,
                event: "테스트".to_string(),
                open: 368.800000,
                high: 368.800000,
                close: 368.700000,
                low: 368.650000,
            },
            Candle {
                timestamp: 1714450980,
                event: "테스트".to_string(),
                open: 368.750000,
                high: 368.850000,
                close: 368.800000,
                low: 368.750000,
            },
        ];

        let mut mock_handler = MockHandler::new();
        mock_handler
            .expect_handle_candle()
            .with(eq(candles[0].clone()))
            .times(1)
            .returning(|_| Ok(()));
        mock_handler
            .expect_handle_candle()
            .with(eq(candles[1].clone()))
            .times(1)
            .returning(|_| Ok(()));
        mock_handler
            .expect_handle_candle()
            .with(eq(candles[2].clone()))
            .times(1)
            .returning(|_| Ok(()));

        let config = Config::new().expect("Failed to create config");
        let reader = Reader::new(config, Box::new(mock_handler)).expect("Failed to create reader");
        let duration = Duration::from_secs(1);

        // Act
        let result = reader.read_and_follow(duration);

        // Assert
        assert!(result.is_ok());
    }

    #[ignore]
    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_read_data_indicator() {
        // Arrange
        const TEXT_FILE_PATH: &str = "tests/example.txt";
        env::set_var("TEXT_FILE_PATH", TEXT_FILE_PATH);

        let datas: [&str; 2] = [
            "2024-05-02 11:00:00  옵션 풋외국인 -13.000000",
            "2024-05-02 11:01:00  옵션 풋외국인 -14.000000",
        ];
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(TEXT_FILE_PATH)
            .expect("Failed to open file");

        for data in &datas {
            writeln!(file, "{data}").expect("Failed to write to file");
        }

        defer! {
            std::fs::remove_file(TEXT_FILE_PATH).expect("Failed to remove file");
        }

        let indicators: [Indicator; 2] = [
            Indicator {
                timestamp: 1714615200,
                event: "옵션".to_string(),
                property: "풋외국인".to_string(),
                value: -13,
            },
            Indicator {
                timestamp: 1714615260,
                event: "옵션".to_string(),
                property: "풋외국인".to_string(),
                value: -14,
            },
        ];

        let mut mock_handler = MockHandler::new();
        mock_handler
            .expect_handle_indicator()
            .with(eq(indicators[0].clone()))
            .times(1)
            .returning(|_| Ok(()));
        mock_handler
            .expect_handle_indicator()
            .with(eq(indicators[1].clone()))
            .times(1)
            .returning(|_| Ok(()));

        let config = Config::new().expect("Failed to create config");
        let reader = Reader::new(config, Box::new(mock_handler)).expect("Failed to create reader");
        let duration = Duration::from_secs(1);

        // Act
        let result: Result<(), io::Error> = reader.read_and_follow(duration);

        // Assert
        assert!(result.is_ok());
    }

    #[ignore]
    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_read_data_composite() {
        // Arrange
        const TEXT_FILE_PATH: &str = "tests/example.txt";
        env::set_var("TEXT_FILE_PATH", TEXT_FILE_PATH);

        let datas: [&str; 4] = [
            "2024-04-30 13:21:00  테스트 368.850000 368.900000 368.750000 368.700000",
            "2024-05-02 11:00:00  옵션 풋외국인 -13.000000",
            "2024-04-30 13:22:00  테스트 368.800000 368.800000 368.700000 368.650000",
            "2024-05-02 11:01:00  옵션 풋외국인 -14.000000",
        ];

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(TEXT_FILE_PATH)
            .expect("Failed to open file");

        for data in &datas {
            writeln!(file, "{data}").expect("Failed to write to file");
        }

        defer! {
            std::fs::remove_file(TEXT_FILE_PATH).expect("Failed to remove file");
        }

        let candles: [Candle; 2] = [
            Candle {
                timestamp: 1714450860,
                event: "테스트".to_string(),
                open: 368.850000,
                high: 368.900000,
                close: 368.750000,
                low: 368.700000,
            },
            Candle {
                timestamp: 1714450920,
                event: "테스트".to_string(),
                open: 368.800000,
                high: 368.800000,
                close: 368.700000,
                low: 368.650000,
            },
        ];

        let indicators: [Indicator; 2] = [
            Indicator {
                timestamp: 1714615200,
                event: "옵션".to_string(),
                property: "풋외국인".to_string(),
                value: -13,
            },
            Indicator {
                timestamp: 1714615260,
                event: "옵션".to_string(),
                property: "풋외국인".to_string(),
                value: -14,
            },
        ];

        let mut mock_handler = MockHandler::new();
        mock_handler
            .expect_handle_candle()
            .with(eq(candles[0].clone()))
            .times(1)
            .returning(|_| Ok(()));
        mock_handler
            .expect_handle_candle()
            .with(eq(candles[1].clone()))
            .times(1)
            .returning(|_| Ok(()));
        mock_handler
            .expect_handle_indicator()
            .with(eq(indicators[0].clone()))
            .times(1)
            .returning(|_| Ok(()));
        mock_handler
            .expect_handle_indicator()
            .with(eq(indicators[1].clone()))
            .times(1)
            .returning(|_| Ok(()));

        let config = Config::new().expect("Failed to create config");
        let reader = Reader::new(config, Box::new(mock_handler)).expect("Failed to create reader");
        let duration = Duration::from_secs(1);

        // Act
        let result = reader.read_and_follow(duration);

        // Assert
        assert!(result.is_ok());
    }
}
