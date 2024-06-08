use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::thread;
use std::time::Duration;

use crate::file::config::Config;

struct Reader {
    path: String,
}

impl Reader {
    pub fn new(config: Config) -> io::Result<Self> {
        File::open(&config.path)?;
        Ok(Reader { path: config.path })
    }

    fn read_and_follow(&self) -> io::Result<()> {
        loop {
            let file = File::open(&self.path)?;
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line = line?;
                println!("{}", line);
            }

            thread::sleep(Duration::from_secs(1));
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[ignore]
    #[test]
    fn test_new_fail_if_no_file_exists() {
        // Arrange
        const TEXT_FILE_PATH: &str = "tests/example.txt";
        env::set_var("TEXT_FILE_PATH", TEXT_FILE_PATH);

        // Act
        let config = Config::new().expect("Failed to create config");
        let reader = Reader::new(config);

        // Assert
        assert!(reader.is_ok());
    }

    #[ignore]
    #[test]
    fn test_new_success_if_file_exists() {
        // Arrange
        const TEXT_FILE_PATH: &str = "tests/example.txt";
        env::set_var("TEXT_FILE_PATH", TEXT_FILE_PATH);
        let file = File::create(TEXT_FILE_PATH).expect("Failed to create file");

        // Act
        let config = Config::new().expect("Failed to create config");
        let reader = Reader::new(config);

        // Assert
        assert!(reader.is_ok());

        // Cleanup
        drop(file);
        std::fs::remove_file(TEXT_FILE_PATH).expect("Failed to remove file");
    }

}