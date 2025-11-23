use crate::models::PriceData;

pub struct Aggregator;

impl Aggregator {
    /// Advanced Algorithm: Confidence-Weighted Average
    /// Giving more weight to sources with lower uncertainty (tighter confidence intervals).
    pub fn calculate_weighted_consensus(prices: Vec<PriceData>) -> Option<PriceData> {
        if prices.is_empty() { return None }

        // 1. Filter out invalid data (Infinite or Zero confidence)
        let valid_prices: Vec<&PriceData> = prices.iter()
            .filter(|p| p.price.is_finite() && p.confidence > 0.0)
            .collect();

        if valid_prices.is_empty() { return None }

        // 2. Calculate Weights (Weight = 1 / Confidence)
        // Smaller confidence interval = Higher Weight (More Trust)
        let mut total_weight = 0.0;
        let mut weighted_price_sum = 0.0;
        let mut weighted_conf_sum = 0.0;

        for p in &valid_prices {
            let weight = 1.0 / p.confidence;
            
            weighted_price_sum += p.price * weight;
            weighted_conf_sum += p.confidence * weight; // Aggregate confidence too
            total_weight += weight;
        }

        // 3. Normalize
        let final_price = weighted_price_sum / total_weight;
        let final_conf = weighted_conf_sum / total_weight;

        // 4. Return Consensus Object
        Some(PriceData {
            symbol: valid_prices[0].symbol.clone(),
            price: final_price,
            confidence: final_conf,
            timestamp: valid_prices[0].timestamp,
            source: "weighted_consensus".to_string(),
        })
    }
}