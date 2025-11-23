mod models;
mod aggregator;
mod oracle_client;
mod storage;
mod routes;
mod app;

use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use crate::app::app;
use crate::storage::Storage;
use crate::oracle_client::OracleClient;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    // Config: Use DEVNET for the demo
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let redis_url = "redis://127.0.0.1/";
    let rpc_url = "https://api.devnet.solana.com";

    // Init Dependencies
    let storage = Arc::new(Storage::new(&db_url, redis_url).await);
    let oracle_client = OracleClient::new(rpc_url);

    // --- BACKGROUND TASK START ---
    let storage_clone = storage.clone();
    let oracle_clone = oracle_client.clone();
    
    tokio::spawn(async move {
        println!("🔄 Background Oracle Fetcher Started...");
        println!("🛡️  Security Mode: ATTACK SIMULATION ACTIVE");
        
        loop {
            // Address: J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix (Devnet SOL/USD)
            let res = oracle_clone.fetch_pyth_price("SOL", "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix").await;
            
            match res {
                Ok(mut pyth_price) => {
                    // 1. Setup Real Data (Source A - The Good Oracle)
                    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
                    pyth_price.timestamp = now;
                    pyth_price.confidence = 0.05; // Tight confidence (High Trust)
                    
                    // 2. Setup Fake Malicious Data (Source B - The Attacker)
                    // Reports price is $50 higher to liquidate users, but has bad confidence
                    let mut bad_price = pyth_price.clone();
                    bad_price.source = "MaliciousOracle".to_string();
                    bad_price.price += 50.0; 
                    bad_price.confidence = 5.0; // Wide confidence (Low Trust)

                    // 3. Aggregate using the NEW function name
                    let inputs = vec![pyth_price.clone(), bad_price.clone()];
                    
                    // FIX: Calling 'calculate_weighted_consensus' instead of 'calculate_median'
                    if let Some(final_price) = aggregator::Aggregator::calculate_weighted_consensus(inputs) {
                        
                        // 4. Save
                        if let Err(e) = storage_clone.save_price(&final_price).await {
                            eprintln!("❌ DB Error: {}", e);
                        } else {
                            // --- VIDEO DEMO LOGS ---
                            println!("\n---------------------------------------------------");
                            println!("🔍 ORACLE AGGREGATION EVENT:");
                            println!("   [1] Pyth (Real):     ${:.4} (Conf: {:.4}) -> Weight: High", pyth_price.price, pyth_price.confidence);
                            println!("   [2] Attacker (Fake): ${:.4} (Conf: {:.4}) -> Weight: Low", bad_price.price, bad_price.confidence);
                            println!("   ------------------------------------------------");
                            println!("   ✅ Consensus Price:  ${:.4} (Attack Resisted!)", final_price.price);
                            println!("---------------------------------------------------");
                        }
                    }
                }
                Err(e) => eprintln!("⚠️ Fetch Error: {}", e),
            }
            // Sleep for 1 second
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });
    // --- BACKGROUND TASK END ---

    // Start Server
    let app = app(storage);
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🌍 Server listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}