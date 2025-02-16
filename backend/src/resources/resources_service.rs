use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    body::Body,
    Json,
};
use crate::utils::{
    db::AppState, 
    response::error_response
};
use super::{
    resources_model::{create_resource, delete_resource, get_resources, update_resource},
    resources_structure::Resource,
};

pub async fn create_resource_service(
    state: State<AppState>,
    resource: Json<Resource>,
) -> Response<Body> {
    // Validate resource data
    if resource.name.trim().is_empty()
        || resource.category.trim().is_empty()
        || resource.description.trim().is_empty()
    {
        return error_response("All fields must be non-empty", StatusCode::BAD_REQUEST).into_response();
    }

    if resource.quantity == 0 {
        return error_response("Quantity must be greater than zero", StatusCode::BAD_REQUEST).into_response();
    }

    // Validate latitude and longitude
    if !is_valid_coordinates(resource.location.latitude, resource.location.longitude) {
        return error_response(
            "Invalid coordinates. Latitude must be between -90 and 90, longitude between -180 and 180",
            StatusCode::BAD_REQUEST
        ).into_response();
    }

    create_resource(state, resource).await.into_response()
}

pub async fn get_resources_service(state: State<AppState>) -> Response<Body> {
    get_resources(state).await.into_response()
}

pub async fn delete_resource_service(
    state: State<AppState>,
    headers: HeaderMap,
) -> Response<Body> {
    let id = match headers.get("id") {
        Some(value) => match value.to_str() {
            Ok(id) => id.to_string(),
            Err(_) => return error_response("Invalid id format", StatusCode::BAD_REQUEST).into_response(),
        },
        None => return error_response("Missing id header", StatusCode::BAD_REQUEST).into_response(),
    };

    delete_resource(state, id).await.into_response()
}

pub async fn update_resource_service(
    state: State<AppState>,
    headers: HeaderMap,
    resource: Json<Resource>,
) -> Response<Body> {
    if resource.quantity == 0 {
        return error_response("Quantity must be greater than zero", StatusCode::BAD_REQUEST).into_response();
    }

    // Validate latitude and longitude
    if !is_valid_coordinates(resource.location.latitude, resource.location.longitude) {
        return error_response(
            "Invalid coordinates. Latitude must be between -90 and 90, longitude between -180 and 180",
            StatusCode::BAD_REQUEST
        ).into_response();
    }

    let id = match headers.get("id") {
        Some(value) => match value.to_str() {
            Ok(id) => id.to_string(),
            Err(_) => return error_response("Invalid id format", StatusCode::BAD_REQUEST).into_response(),
        },
        None => return error_response("Missing id header", StatusCode::BAD_REQUEST).into_response(),
    };

    update_resource(state, resource, id).await.into_response()
}

/// Validates if the given coordinates are within valid ranges
fn is_valid_coordinates(latitude: f64, longitude: f64) -> bool {
    latitude >= -90.0 && latitude <= 90.0 && longitude >= -180.0 && longitude <= 180.0
}
