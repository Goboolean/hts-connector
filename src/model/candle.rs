#[derive(Debug, PartialEq, Clone)]
pub struct Candle {
    pub event: String,
    pub timestamp: u128,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}
