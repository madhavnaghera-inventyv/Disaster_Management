use std::sync::Arc;

use axum::{routing::{get, patch, post}, Router};
use disaster_service::{add_disaster_service, add_donts_service, add_dos_service, get_all_disaster_record_service, get_disaster_record_service, update_donts_service, update_dos_service};

use crate::utils::db::AppState;

pub mod disaster_model;
pub mod disaster_service;
pub mod disaster_structure;

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/add_disaster_record", post(add_disaster_service))
        .route("/add_do/{dr_id}", patch(add_dos_service))
        .route("/add_dont/{dr_id}", patch(add_donts_service))
        .route("/get_disaster_record/{dr_id}", get(get_disaster_record_service))
        .route("/update_do/{dr_id}/{gi_id}", patch(update_dos_service))
        .route("/update_dont/{dr_id}/{gi_id}", patch(update_donts_service))
        .route("/get_all_disaster_record/{dr_id}", get(get_all_disaster_record_service)) 
        .with_state((*state).clone())
}

