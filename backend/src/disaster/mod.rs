use std::sync::Arc;

use axum::{
    extract::State, middleware::{from_fn, from_fn_with_state}, routing::{get, patch, post}, Router
};
use disaster_model::get_pending_guidelines;
use disaster_service::{
    add_disaster_service, add_donts_service, add_dos_service,
    get_disaster_record_service, update_donts_service, update_dos_service,
};
use tower::layer;

use crate::{
    middleware::{auth::auth_middleware, ngo::ngo_middleware},
    utils::db::AppState,
};

pub mod disaster_model;
pub mod disaster_service;
pub mod disaster_structure;

pub fn create_routes(state: AppState) -> Router {
    let restricted_routes = Router::new()
        .route("/update_do/{dr_id}/{gi_id}", patch(update_dos_service))
        .route("/update_dont/{dr_id}/{gi_id}", patch(update_donts_service))
        .route("/get_pending_guidelines", get(get_pending_guidelines))
        .layer(from_fn_with_state(state.clone(), ngo_middleware));

    let open_routes = Router::new()
        .route("/add_disaster_record", post(add_disaster_service))
        .route("/add_do/{dr_id}", patch(add_dos_service))
        .route("/add_dont/{dr_id}", patch(add_donts_service))
        .route("/get_disaster_record/{dr_id}", get(get_disaster_record_service));

    Router::new()
        .merge(restricted_routes)
        .merge(open_routes)
        .layer(from_fn(auth_middleware))
        .with_state(state.clone())  
}
