use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use pyth_sdk_solana::load_price_feed_from_account_info;
use std::str::FromStr;
use crate::models::PriceData;

#[derive(Clone)] // Added Clone so we can pass it to threads
pub struct OracleClient { 
    // We wrap in Arc to make it cheap to clone
    client: std::sync::Arc<RpcClient> 
}

impl OracleClient {
    pub fn new(rpc_url: &str) -> Self {
        Self { 
            client: std::sync::Arc::new(RpcClient::new(rpc_url.to_string())) 
        }
    }

    pub async fn fetch_pyth_price(&self, symbol: &str, feed_address: &str) -> anyhow::Result<PriceData> {
        let pk = Pubkey::from_str(feed_address)?;
        let mut account = self.client.get_account(&pk).await?;

        // Pyth Mock Setup
        let key = pk;
        let owner = account.owner;
        let mut lamports = account.lamports;
        let mut data = account.data;
        
        let account_info = solana_sdk::account_info::AccountInfo::new(
            &key, false, true, &mut lamports, &mut data, &owner, false, 0,
        );

        let feed = load_price_feed_from_account_info(&account_info)
            .map_err(|_| anyhow::anyhow!("Pyth error"))?;

        let current = feed.get_price_unchecked();
        let price = (current.price as f64) * 10f64.powi(current.expo);
        let conf  = (current.conf as f64) * 10f64.powi(current.expo);

        Ok(PriceData {
            symbol: symbol.to_string(),
            price,
            confidence: conf,
            timestamp: current.publish_time,
            source: "pyth".to_string(),
        })
    }
}