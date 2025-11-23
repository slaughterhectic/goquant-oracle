use axum::{Router, routing::get, extract::{Path, State}, Json, http::StatusCode};
use serde_json::json;
use std::sync::Arc;
use crate::storage::Storage;

pub fn routes(storage: Arc<Storage>) -> Router {
    Router::new()
        .route("/oracle/price/:symbol", get(get_price))
        .with_state(storage)
}

async fn get_price(
    Path(symbol): Path<String>, 
    State(storage): State<Arc<Storage>>
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match storage.get_latest_price(&symbol).await {
        Ok(json) => {
            let v: serde_json::Value = serde_json::from_str(&json).unwrap_or(json!("invalid"));
            Ok(Json(v))
        }
        Err(_) => Err((StatusCode::NOT_FOUND, Json(json!({"error": "Price not found"}))))
    }
}