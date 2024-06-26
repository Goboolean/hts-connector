#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Indicator {
    pub timestamp: u128,
    pub event: String,
    pub property: String,
    pub value: i64,
}
