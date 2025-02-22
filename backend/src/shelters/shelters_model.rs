use axum::{extract::State, http::StatusCode, response:: Response, Json};
use futures::TryStreamExt;
use mongodb::{bson::{self, doc, oid::ObjectId}, Collection};
use std::sync::Arc;
use crate::utils::db::AppState;
use super::shelters_structure::Shelter;
use crate::utils::response::{success_response, error_response};

pub async fn create_shelters(
    State(state): State<Arc<AppState>>,
    Json(shelter): Json<Shelter>,
) -> Response {
    let db = state.db.lock().await;
    let collection: Collection<Shelter> = db.database("disaster").collection("shelters");

    // Check if a shelter with the same name already exists
    if collection.find_one(doc! { "name": &shelter.name }).await.unwrap_or(None).is_some() {
        return error_response("Shelter already present", StatusCode::BAD_REQUEST);
    }

    // Insert shelter into the database
    match collection.insert_one(&shelter).await {
        Ok(result) => {
            if let Some(inserted_id) = result.inserted_id.as_object_id() {
                success_response("Shelter created successfully", inserted_id.to_hex(), StatusCode::CREATED)
            } else {
                error_response("Failed to retrieve inserted ID", StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        Err(e) => error_response(&format!("Database error: {}", e), StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_shelters(State(state): State<Arc<AppState>>) -> Response {
    let db = state.db.lock().await;
    let collection: Collection<Shelter> = db.database("disaster").collection("shelters");
    match collection.find(doc! {}).await {
        Ok(cursor) => {
            let shelters: Vec<Shelter> = cursor.try_collect().await.unwrap_or_else(|_| vec![]);
            success_response("Shelters retrieved successfully", shelters, StatusCode::OK)
        }   
        Err(e) => error_response(&format!("Error fetching shelters: {}", e), StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_shelter(
    State(state): State<Arc<AppState>>,
    id: String,
) -> Response {
    let db = state.db.lock().await;
    let collection: Collection<Shelter> = db.database("disaster").collection("shelters");

    let obj_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return error_response("Invalid ID format", StatusCode::BAD_REQUEST),
    };

    match collection.delete_one(doc! { "_id": obj_id }).await {
        Ok(result) => {
            if result.deleted_count == 1 {
                success_response("Shelter deleted successfully", id, StatusCode::OK)
            } else {
                error_response("Shelter not found", StatusCode::NOT_FOUND)
            }
        }
        Err(err) => error_response(&format!("Database error: {}", err), StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_shelters(
    State(state): State<Arc<AppState>>,
    Json(shelter): Json<Shelter>,
    id: String,
) -> Response {
    let db = state.db.lock().await;
    let collection: Collection<Shelter> = db.database("disaster").collection("shelters");

    let obj_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return error_response("Invalid ID format", StatusCode::BAD_REQUEST),
    };

    let update_doc = doc! { "$set": bson::to_document(&shelter).unwrap() };

    match collection.update_one(doc! { "_id": obj_id }, update_doc).await {
        Ok(result) => {
            if result.matched_count == 1 {
                success_response("Shelter updated successfully", id, StatusCode::OK)
            } else {
                error_response("Shelter not found", StatusCode::NOT_FOUND)
            }
        }
        Err(err) => error_response(&format!("Database error: {}", err), StatusCode::INTERNAL_SERVER_ERROR),
    }
}
