use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceData {
    pub symbol: String,
    pub price: f64,
    pub confidence: f64,
    pub timestamp: i64,
    pub source: String,
}