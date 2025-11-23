use crate::models::PriceData;

pub struct Aggregator;

impl Aggregator {
    pub fn calculate_median(mut prices: Vec<PriceData>) -> Option<PriceData> {
        if prices.is_empty() { return None }

        // Sort
        prices.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(std::cmp::Ordering::Equal));

        let mid = prices.len() / 2;
        let median_price = if prices.len() % 2 == 0 {
            (prices[mid - 1].price + prices[mid].price) / 2.0
        } else {
            prices[mid].price
        };

        Some(PriceData {
            symbol: prices[0].symbol.clone(),
            price: median_price,
            confidence: prices[mid].confidence,
            timestamp: prices[mid].timestamp,
            source: "consensus".to_string(),
        })
    }
}