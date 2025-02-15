use axum::{routing::{post, Route}, Router};
use disaster_model::add_disaster;

use crate::utils::db::AppState;

pub mod disaster_model;
pub mod disaster_service;
pub mod disaster_structure;

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/add_disaster_record", post(add_disaster)) 
        .with_state(state)
}