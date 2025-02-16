use axum::http::HeaderMap;
use axum::{extract::State, Json, http::StatusCode,response::IntoResponse};
use crate::utils::db::AppState;
use super::shelters_structure::Shelter;
use super::shelters_model::{create_shelters, delete_shelter, get_shelters, update_shelters};


pub async fn create_shelter_service(
    State(state): State<AppState>,
    Json(shelter): Json<Shelter>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    if shelter.name.trim().is_empty()
        || shelter.street.trim().is_empty()
        || shelter.district.trim().is_empty()
        || shelter.state.trim().is_empty()
        || shelter.country.trim().is_empty()
    {
        return Err((StatusCode::BAD_REQUEST, "All text fields must be non-empty".to_string()));
    }

    if shelter.capacity == 0 {
        return Err((StatusCode::BAD_REQUEST, "Capacity must be greater than zero".to_string()));
    }

    if shelter.available_beds > shelter.capacity {
        return Err((StatusCode::BAD_REQUEST, "Available beds cannot exceed total capacity".to_string()));
    }

    create_shelters(State(state), Json(shelter)).await
}



pub async fn get_shelter_service(State(state): State<AppState>) -> impl IntoResponse {
    get_shelters(State(state)).await
}


pub async fn delete_shelter_service(State(state): State<AppState>,headers:HeaderMap) -> impl IntoResponse {
    let id = match headers.get("id") {
        Some(value) => match value.to_str() {
            Ok(id) => id.to_string(),
            Err(_) => return (StatusCode::BAD_REQUEST, Json("Invalid id format".to_string())),
        },
        None => return (StatusCode::BAD_REQUEST, Json("Missing id header".to_string())),
    };

    delete_shelter(State(state),id).await
}


pub async fn update_shelter_service(State(state): State<AppState>,headers:HeaderMap,Json(shelter): Json<Shelter>) -> impl IntoResponse {
    if shelter.available_beds > shelter.capacity {
        return (StatusCode::BAD_REQUEST, Json("Available beds cannot exceed total capacity".to_string()))
    }
    let id = match headers.get("id") {
        Some(value) => match value.to_str() {
            Ok(id) => id.to_string(),
            Err(_) => return (StatusCode::BAD_REQUEST, Json("Invalid id format".to_string())),
        },
        None => return (StatusCode::BAD_REQUEST, Json("Missing id header".to_string())),
    };
    update_shelters(State(state),Json(shelter),id).await
}
