use axum::http::HeaderMap;
use axum::{extract::State, Json, http::StatusCode, response::Response};
use std::sync::Arc;
use crate::utils::db::AppState;
use super::shelters_structure::Shelter;
use super::shelters_model::{create_shelters, delete_shelter, get_shelters, update_shelters};
use crate::utils::response::error_response;

pub async fn create_shelter_service(
    State(state): State<Arc<AppState>>,
    Json(shelter): Json<Shelter>,
) -> Response {
    if shelter.name.trim().is_empty()
        || shelter.street.trim().is_empty()
        || shelter.district.trim().is_empty()
        || shelter.state.trim().is_empty()
        || shelter.country.trim().is_empty()
    {
        return error_response("All text fields must be non-empty", StatusCode::BAD_REQUEST);
    }

    if shelter.capacity == 0 {
        return error_response("Capacity must be greater than zero", StatusCode::BAD_REQUEST);
    }

    if shelter.available_beds > shelter.capacity {
        return error_response("Available beds cannot exceed total capacity", StatusCode::BAD_REQUEST);
    }

    create_shelters(State(state), Json(shelter)).await
}

pub async fn get_shelter_service(State(state): State<Arc<AppState>>) -> Response {
    get_shelters(State(state)).await
}

pub async fn delete_shelter_service(State(state): State<Arc<AppState>>, headers: HeaderMap) -> Response {
    let id = match headers.get("id") {
        Some(value) => match value.to_str() {
            Ok(id) => id.to_string(),
            Err(_) => return error_response("Invalid id format", StatusCode::BAD_REQUEST),
        },
        None => return error_response("Missing id header", StatusCode::BAD_REQUEST),
    };

    delete_shelter(State(state), id).await
}

pub async fn update_shelter_service(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(shelter): Json<Shelter>,
) -> Response {
    if shelter.available_beds > shelter.capacity {
        return error_response("Available beds cannot exceed total capacity", StatusCode::BAD_REQUEST);
    }

    let id = match headers.get("id") {
        Some(value) => match value.to_str() {
            Ok(id) => id.to_string(),
            Err(_) => return error_response("Invalid id format", StatusCode::BAD_REQUEST),
        },
        None => return error_response("Missing id header", StatusCode::BAD_REQUEST),
    };

    update_shelters(State(state), Json(shelter), id).await
}
