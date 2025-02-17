use axum::{
    middleware::{ from_fn_with_state}, routing::{delete, get, patch, post}, Router
};
use resources_service::{
    create_resource_service, delete_resource_service, get_resources_service, update_resource_service,
};
use crate::{middleware::auth::auth_middleware, utils::db::AppState};

pub mod resources_model;
pub mod resources_service;
pub mod resources_structure;

pub fn resources_routes(state: AppState) -> Router {
    Router::new()
        .route("/get_resources", get(get_resources_service))
        .route("/create_resource", post(create_resource_service))
        .route("/delete_resource", delete(delete_resource_service))
        .route("/update_resource", patch(update_resource_service))
        .layer(from_fn_with_state(state.clone(),auth_middleware))
        .with_state(state)
}       


