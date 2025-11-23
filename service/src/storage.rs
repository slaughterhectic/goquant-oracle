use sqlx::{PgPool, types::BigDecimal};
use redis::{Client, AsyncCommands};
use crate::models::PriceData;
use std::str::FromStr;

pub struct Storage { db: PgPool, redis: Client }

impl Storage {
    pub async fn new(db_url: &str, redis_url: &str) -> Self {
        let db = PgPool::connect(db_url).await.expect("Failed to connect to Postgres");
        let redis = Client::open(redis_url).expect("Invalid Redis URL");
        Self { db, redis }
    }

    pub async fn save_price(&self, data: &PriceData) -> anyhow::Result<()> {
        // Redis
        let mut conn = self.redis.get_async_connection().await?;
        let json = serde_json::to_string(data)?;
        let _: () = conn.set_ex(format!("price:{}", data.symbol), json, 60).await?;

        // DB
        // Note: We use simple query for compatibility here
        let price_bd = BigDecimal::from_str(&data.price.to_string()).unwrap_or_default();
        let conf_bd  = BigDecimal::from_str(&data.confidence.to_string()).unwrap_or_default();

        sqlx::query!(
            r#"INSERT INTO price_history (symbol, price, confidence, source, timestamp)
            VALUES ($1, $2, $3, $4, $5)"#,
            data.symbol,
            price_bd,
            conf_bd,
            data.source,
            data.timestamp
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn get_latest_price(&self, symbol: &str) -> anyhow::Result<String> {
        let mut conn = self.redis.get_async_connection().await?;
        let data: String = conn.get(format!("price:{}", symbol)).await?;
        Ok(data)
    }
}