use axum::{
    body::Body, extract::{Path, Request, State}, http::StatusCode, response::{IntoResponse, Response}, Extension, Json, RequestPartsExt
};
use mongodb::bson::oid::ObjectId;
use serde_json::json;
use crate::utils::{db::AppState, response::error_response};
use super::{disaster_model, disaster_structure::DisasterRecord};

pub async fn add_disaster_service(
    State(state): State<AppState>, 
    Json(payload): Json<DisasterRecord>,
) -> Response {
    // Validate input
    if payload.name.trim().is_empty() || payload.short_description.trim().is_empty() {
        return error_response("Name and description must not be empty",StatusCode::BAD_REQUEST);
    }

    // Call your model's add_disaster function
    disaster_model::add_disaster(State(state), Json(payload)).await.into_response()
}

pub async fn add_dos_service(
    State(state): State<AppState>,
    Path(dr_id): Path<ObjectId>,
    Json(payload): Json<serde_json::Value>,
) -> Response {
    // println!("User ID: {}", user_id);
    // Validate input
    if payload.get("message").and_then(|m| m.as_str()).is_none() {
        return error_response("Message field is required", StatusCode::BAD_REQUEST);
    }

    // Call your model's add_dos function
    disaster_model::add_dos(State(state), Path(dr_id), Json(payload)).await.into_response()
}


pub async fn add_donts_service(
    State(state): State<AppState>,
    Path(dr_id): Path<ObjectId>,
    Json(payload): Json<serde_json::Value>,
) -> Response {
    // Validate input
    if payload.get("message").and_then(|m| m.as_str()).is_none() {
        return error_response("Message field is required", StatusCode::BAD_REQUEST);
    }

    // Call your model's add_donts function
    disaster_model::add_donts(State(state), Path(dr_id), Json(payload)).await.into_response()
}


pub async fn get_disaster_record_service(
    State(state): State<AppState>,
    Path(dr_id): Path<String>,
) -> Response {
    // Extract the dr_id from the path
    let path_parts: Vec<&str> = dr_id.split('/').collect();
    let dr_id_str = match path_parts.last() {
        Some(id) => *id,
        None => return error_response("Invalid path format", StatusCode::BAD_REQUEST),
    };

    // Validate that the ID is a valid MongoDB ObjectId
    let dr_id = match ObjectId::parse_str(dr_id_str) {
        Ok(id) => id,
        Err(_) => return error_response("Invalid disaster record ID format", StatusCode::BAD_REQUEST),
    };

    // Call the handler with validated ObjectId
    disaster_model::get_disaster_record(
        State(state),
        Path(dr_id),
    ).await.into_response()
}

pub async fn get_all_disaster_record_service(
    State(state): State<AppState>,
    Path(dr_id): Path<String>,
) -> Response {
    // Extract the dr_id from the path
    let path_parts: Vec<&str> = dr_id.split('/').collect();
    let dr_id_str = match path_parts.last() {
        Some(id) => *id,
        None => return error_response("Invalid path format", StatusCode::BAD_REQUEST),
    };

    // Validate that the ID is a valid MongoDB ObjectId
    let dr_id = match ObjectId::parse_str(dr_id_str) {
        Ok(id) => id,
        Err(_) => return error_response("Invalid disaster record ID format", StatusCode::BAD_REQUEST),
    };

    // Call the handler with validated ObjectId
    disaster_model::get_all_disaster_record(
        State(state),
        Path(dr_id),
    ).await.into_response()
}

pub async fn update_dos_service(
    State(state): State<AppState>,
    Path((dr_id, gi_id)): Path<(String, String)>,
    Json(payload): Json<serde_json::Value>,
) -> Response {
    // Validate ObjectIds
    let disaster_id = match ObjectId::parse_str(&dr_id) {
        Ok(valid_id) => valid_id,
        Err(_) => {
            return error_response("Invalid disaster ID format", StatusCode::BAD_REQUEST);
        }
    };

    let guide_item_id = match ObjectId::parse_str(&gi_id) {
        Ok(valid_id) => valid_id,
        Err(_) => {
            return error_response("Invalid guide item ID format", StatusCode::BAD_REQUEST);
        }
    };

    // Validate payload for required status field
    let status = match payload.get("status").and_then(|s| s.as_str()) {
        Some(valid_status) => valid_status,
        None => {
            return error_response("Missing or invalid status field in request body", StatusCode::BAD_REQUEST);
        }
    };

    // Call the model function and get the response
    let update_result = disaster_model::update_dos(
        State(state),
        Path((disaster_id, guide_item_id)),
        Json(json!({ "status": status })),
    )
    .await;

    // Return the response directly from update_dos or handle errors accordingly
    update_result.into_response()
}



pub async fn update_donts_service(
    State(state): State<AppState>,
    Path((dr_id, gi_id)): Path<(String, String)>,
    Json(payload): Json<serde_json::Value>,
) -> Response {
    // Validate ObjectIds
    let disaster_id = match ObjectId::parse_str(&dr_id) {
        Ok(valid_id) => valid_id,
        Err(_) => {
            return error_response("Invalid disaster ID format", StatusCode::BAD_REQUEST);
        }
    };

    let guide_item_id = match ObjectId::parse_str(&gi_id) {
        Ok(valid_id) => valid_id,
        Err(_) => {
            return error_response("Invalid guide item ID format", StatusCode::BAD_REQUEST);
        }
    };

    // Validate payload for required status field
    let status = match payload.get("status").and_then(|s| s.as_str()) {
        Some(valid_status) => valid_status,
        None => {
            return error_response("Missing or invalid status field in request body", StatusCode::BAD_REQUEST);
        }
    };

    // Call the model function and get the response
    let update_result = disaster_model::update_donts(
        State(state),
        Path((disaster_id, guide_item_id)),
        Json(json!({ "status": status })),
    )
    .await;

    // Return the response directly from update_dos or handle errors accordingly
    update_result.into_response()
}