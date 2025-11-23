use axum::Router;
use std::sync::Arc;
use crate::storage::Storage;
use crate::routes::routes;

pub fn app(storage: Arc<Storage>) -> Router {
    routes(storage)
}