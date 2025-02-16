use axum::{extract::State, Json, http::StatusCode, response::IntoResponse};
use futures::TryStreamExt;
use mongodb::{bson::{self, doc, oid::ObjectId}, Collection};
use crate::utils::db::AppState;
use super::shelters_structure::Shelter;


pub async fn create_shelters(
    State(state): State<AppState>,
    Json(shelter): Json<Shelter>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let db = state.db.lock().await;
    let collection: Collection<Shelter> = db.database("disaster").collection("shelters");

    // Check if a shelter with the same name already exists
    if collection.find_one(doc! { "name": &shelter.name }).await.unwrap_or(None).is_some() {
        return Err((StatusCode::BAD_REQUEST, "Shelter already present".to_string()));
    }

    // Insert shelter into the database
    match collection.insert_one(shelter.clone()).await {
        Ok(result) => {
            if let Some(inserted_id) = result.inserted_id.as_object_id() {
                Ok(Json(serde_json::json!({ "id": inserted_id.to_hex() })))
            } else {
                Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to retrieve inserted ID".to_string()))
            }
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))),
    }
}

pub async fn get_shelters(State(state): State<AppState>) -> impl IntoResponse {
    let db = state.db.lock();
    let collections:Collection<Shelter> = db.await.database("disaster").collection("shelters");

    match collections.find(mongodb::bson::Document::new()).await{
        Ok(cursor) => {
            let users: Vec<Shelter> = cursor.try_collect().await.unwrap_or_else(|_| vec![]);
            Json(users)
        }
        Err(e) => {
            eprintln!("Error fetching users: {:?}", e);
            Json(vec![]) 
        }
    }
}


pub async fn delete_shelter(
    State(state): State<AppState>,
    id: String,
) -> (StatusCode, Json<std::string::String>) {
    let db = state.db.lock().await;
    let collection: Collection<Shelter> = db.database("disaster").collection("shelters");

    let obj_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return (StatusCode::BAD_REQUEST, Json("Invalid ID format".to_string())),
    };

    match collection.delete_one(doc! { "_id": obj_id }).await {
        Ok(result) => {
            if result.deleted_count == 1 {
                (StatusCode::OK, Json("Shelter deleted successfully".to_string()))
            } else {
                (StatusCode::NOT_FOUND, Json("Shelter not found".to_string()))
            }
        }
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Database error: {}", err))),
    }
}

pub async fn update_shelters(State(state): State<AppState>,Json(shelter): Json<Shelter>,id:String) -> (StatusCode, Json<std::string::String>) {
    let db = state.db.lock().await;
    let collections:Collection<Shelter> = db.database("disaster").collection("shelters");

    let obj_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return (StatusCode::BAD_REQUEST, Json("Invalid ID format".to_string())),
    };

    let update_doc = doc! { "$set": bson::to_document(&shelter).unwrap() };

    match collections.update_one(doc! { "_id": obj_id }, update_doc).await {
        Ok(result) => {
            if result.matched_count == 1 {
                (StatusCode::OK, Json("Shelter updated successfully".to_string()))
            } else {
                (StatusCode::NOT_FOUND, Json("Shelter not found".to_string()))
            }
        }
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Database error: {}", err))),
    }
}
