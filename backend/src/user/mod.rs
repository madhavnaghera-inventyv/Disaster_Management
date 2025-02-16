pub mod user_model;
pub mod user_service;
pub mod user_structure;
use axum::routing::post;
use axum::Router;
use user_service::login_service;
use crate::utils::db::AppState; 

pub fn user_routes(state: AppState) -> Router {
    Router::new()
        .route("/login", post(login_service))
        .route("/register", post(user_service::register_service)) 
        .with_state(state)
}
