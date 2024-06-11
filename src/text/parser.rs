use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use chrono_tz::Asia::Seoul;

use std::error::Error;

use crate::model::{candle::Candle, indicator::Indicator};



fn parse_f64_to_i64(input: f64) -> Result<i64, Box<dyn Error>> {
    if input.fract() != 0.0 {
        return Err(format!("Expected integer, found {}", input).into());
    }

    Ok(input as i64)
}

pub fn parse_candle(input: String) -> Result<Candle, Box<dyn Error>> {
    let parts: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    if parts.len() != 7 {
        return Err(format!("Expected 7 parts, found {}", parts.len()).into());
    }

    let date = NaiveDate::parse_from_str(&parts[0], "%Y-%m-%d").expect("Failed to parse date");
    let time = NaiveTime::parse_from_str(&parts[1], "%H:%M:%S").expect("Failed to parse time");
    let datetime: NaiveDateTime = NaiveDateTime::new(date, time);

    let candle = Candle {
        timestamp: datetime.and_local_timezone(Seoul).unwrap().timestamp() as u128,
        event: parts[2].clone(),
        open: parts[3].parse().expect("Failed to parse open"),
        high: parts[4].parse().expect("Failed to parse high"),
        close: parts[5].parse().expect("Failed to parse close"),
        low: parts[6].parse().expect("Failed to parse low"),
    };

    Ok(candle)
}

pub fn parse_indicator(input: String) -> Result<Indicator, Box<dyn Error>> {
    let parts: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    if parts.len() != 5 {
        return Err(format!("Expected 5 parts, found {}", parts.len()).into());
    }

    let date = NaiveDate::parse_from_str(&parts[0], "%Y-%m-%d")?;
    let time = NaiveTime::parse_from_str(&parts[1], "%H:%M:%S")?;
    let datetime: NaiveDateTime = NaiveDateTime::new(date, time);

    let float_value: f64 = parts[4].parse()?;

    let indicator = Indicator {
        timestamp: datetime.and_local_timezone(Seoul).unwrap().timestamp() as u128,
        event: parts[2].clone(),
        property: parts[3].clone(),
        value: parse_f64_to_i64(float_value)?,
    };

    Ok(indicator)
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_parse_f64_to_i64_success() {
        // Arrange
        let input = 10.0;

        // Act
        let result = parse_f64_to_i64(input);

        // Assert
        assert_eq!(result.unwrap(), 10);
    }

    #[test]
    fn test_parse_f64_to_i64_fail() {
        // Arrange
        let input = 10.5;

        // Act
        let result = parse_f64_to_i64(input);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_candle_success() {
        // Arrange
        let input = "2021-01-01 00:00:00 BTCUSDT 100.0 200.0 150.0 50.0".to_string();
        let expect = Candle {
            timestamp: 1609426800,
            event: "BTCUSDT".to_string(),
            open: 100.0,
            high: 200.0,
            close: 150.0,
            low: 50.0,
        };

        // Act
        let result = parse_candle(input);

        // Assert
        assert_eq!(result.unwrap(), expect);
    }

    #[test]
    fn test_parse_candle_fail() {
        // Arrange
        let input = "2021-01-01 00:00:00 BTCUSDT 100.0 200.0 150.0".to_string();

        // Act
        let result = parse_candle(input);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_indicator_success() {
        // Arrange
        let input = "2021-01-01 00:00:00 이벤트 속성 -70.0".to_string();
        let expect = Indicator {
            timestamp: 1609426800,
            event: "이벤트".to_string(),
            property: "속성".to_string(),
            value: -70,
        };

        // Act
        let result = parse_indicator(input);

        // Assert
        assert_eq!(result.unwrap(), expect);
    }

    #[test]
    fn test_parse_indicator_fail() {
        // Arrange
        let input = "2021-01-01 00:00:00 이벤트 속성 -70.0".to_string();

        // Act
        let result = parse_indicator(input);

        // Assert
        assert!(result.is_err());
    }
}