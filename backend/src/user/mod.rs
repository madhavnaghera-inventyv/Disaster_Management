pub mod user_model;
pub mod user_service;
pub mod user_structure;
use axum::routing::get;
use axum::Router;
use user_model::get_users;
use crate::utils::db::AppState; 

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_users)) 
        .with_state(state)
}
