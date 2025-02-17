use std::sync::Arc;

use axum::{middleware::from_fn, routing::{delete, get, patch, post}, Router};
use shelters_service::{create_shelter_service, delete_shelter_service, get_shelter_service, update_shelter_service};
use crate::utils::db::AppState;

pub mod shelters_model;
pub mod shelters_service;
pub mod shelters_structure;
use crate::middleware::auth::auth_middleware;


pub fn shelters_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/get_shelters", get(get_shelter_service)) 
        .route("/create_shelter", post(create_shelter_service))
        .route("/delete_shelter", delete(delete_shelter_service)) 
        .route("/update_shelter", patch(update_shelter_service)) 
        .layer(from_fn(auth_middleware))
        .with_state((*state).clone())
}
