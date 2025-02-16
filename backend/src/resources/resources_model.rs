use axum::{extract::State, Json, http::StatusCode, response::IntoResponse};
use futures::TryStreamExt;
use mongodb::{bson::{self, doc, oid::ObjectId}, Collection};
use crate::utils::{db::AppState, response::{success_response, error_response}};
use super::resources_structure::Resource;

/// Database operations for managing disaster relief resources
/// 
/// This module provides CRUD operations for resources in the MongoDB database.
/// Each function handles database interactions and returns appropriate responses
/// using the common response format.

/// Creates a new resource in the database
/// 
/// # Arguments
/// * `state` - Application state containing the database connection
/// * `resource` - The resource data to be created
/// 
/// # Returns
/// * Success Response (201 Created) with the created resource data
/// * Error Response (500 Internal Server Error) if database operation fails
/// 
/// # Example Success Response
/// ```json
/// {
///     "status": true,
///     "message": "Resource created successfully",
///     "data": {
///         "id": "507f1f77bcf86cd799439011",
///         "name": "Water Supply",
///         "quantity": 1000,
///         "category": "Essential",
///         "description": "Drinking water bottles",
///         "location": {
///             "latitude": 12.9716,
///             "longitude": 77.5946
///         },
///         "status": "available"
///     }
/// }
/// ```
pub async fn create_resource(
    state: State<AppState>,
    resource: Json<Resource>,
) -> impl IntoResponse {
    let db = state.db.lock().await;
    let collection: Collection<Resource> = db.database("disaster").collection("resources");
   
    match collection.insert_one(resource.0.clone()).await {
        Ok(result) => {
            if let Some(inserted_id) = result.inserted_id.as_object_id() {
                let mut created_resource = resource.0;
                created_resource.id = Some(inserted_id);
                success_response(
                    "Resource created successfully",
                    created_resource,
                    StatusCode::CREATED
                )
            } else {
                error_response(
                    "Failed to retrieve inserted ID",
                    StatusCode::INTERNAL_SERVER_ERROR
                )
            }
        }
        Err(e) => error_response(
            &format!("Database error: {}", e),
            StatusCode::INTERNAL_SERVER_ERROR
        ),
    }
}

/// Retrieves all resources from the database
/// 
/// # Arguments
/// * `state` - Application state containing the database connection
/// 
/// # Returns
/// * Success Response (200 OK) with array of resources
/// * Error Response (500 Internal Server Error) if database operation fails
/// 
/// # Example Success Response
/// ```json
/// {
///     "status": true,
///     "message": "Resources retrieved successfully",
///     "data": [
///         {
///             "id": "507f1f77bcf86cd799439011",
///             "name": "Water Supply",
///             "quantity": 1000,
///             "category": "Essential",
///             "description": "Drinking water bottles",
///             "location": {
///                 "latitude": 12.9716,
///                 "longitude": 77.5946
///             },
///             "status": "available"
///         }
///     ]
/// }
/// ```
pub async fn get_resources(
    state: State<AppState>
) -> impl IntoResponse {
    let db = state.db.lock().await;
    let collection: Collection<Resource> = db.database("disaster").collection("resources");

    match collection.find(doc! {}).await {
        Ok(cursor) => {
            match cursor.try_collect::<Vec<Resource>>().await {
                Ok(resources) => success_response(
                    "Resources retrieved successfully",
                    resources,
                    StatusCode::OK
                ),
                Err(e) => error_response(
                    &format!("Failed to collect resources: {}", e),
                    StatusCode::INTERNAL_SERVER_ERROR
                ),
            }
        }
        Err(e) => error_response(
            &format!("Database error: {}", e),
            StatusCode::INTERNAL_SERVER_ERROR
        ),
    }
}

/// Deletes a resource from the database by ID
/// 
/// # Arguments
/// * `state` - Application state containing the database connection
/// * `id` - The ObjectId of the resource to delete
/// 
/// # Returns
/// * Success Response (200 OK) if resource was deleted
/// * Error Response (404 Not Found) if resource doesn't exist
/// * Error Response (400 Bad Request) if ID format is invalid
/// * Error Response (500 Internal Server Error) if database operation fails
/// 
/// # Example Success Response
/// ```json
/// {
///     "status": true,
///     "message": "Resource deleted successfully",
///     "data": "Resource removed from database"
/// }
/// ```
pub async fn delete_resource(
    state: State<AppState>,
    id: String,
) -> impl IntoResponse {
    let db = state.db.lock().await;
    let collection: Collection<Resource> = db.database("disaster").collection("resources");

    let obj_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return error_response("Invalid ID format", StatusCode::BAD_REQUEST),
    };

    match collection.delete_one(doc! { "_id": obj_id }).await {
        Ok(result) => {
            if result.deleted_count == 1 {
                success_response(
                    "Resource deleted successfully",
                    "Resource removed from database",
                    StatusCode::OK
                )
            } else {
                error_response("Resource not found", StatusCode::NOT_FOUND)
            }
        }
        Err(err) => error_response(
            &format!("Database error: {}", err),
            StatusCode::INTERNAL_SERVER_ERROR
        ),
    }
}

/// Updates a resource in the database by ID
/// 
/// # Arguments
/// * `state` - Application state containing the database connection
/// * `resource` - The updated resource data
/// * `id` - The ObjectId of the resource to update
/// 
/// # Returns
/// * Success Response (200 OK) if resource was updated
/// * Error Response (404 Not Found) if resource doesn't exist
/// * Error Response (400 Bad Request) if ID format is invalid
/// * Error Response (500 Internal Server Error) if database operation fails
/// 
/// # Example Success Response
/// ```json
/// {
///     "status": true,
///     "message": "Resource updated successfully",
///     "data": "Resource information updated"
/// }
/// ```
pub async fn update_resource(
    state: State<AppState>,
    resource: Json<Resource>,
    id: String,
) -> impl IntoResponse {
    let db = state.db.lock().await;
    let collection: Collection<Resource> = db.database("disaster").collection("resources");

    let obj_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return error_response("Invalid ID format", StatusCode::BAD_REQUEST),
    };

    let update_doc = match bson::to_document(&resource.0) {
        Ok(doc) => doc! { "$set": doc },
        Err(e) => return error_response(
            &format!("Failed to serialize resource: {}", e),
            StatusCode::INTERNAL_SERVER_ERROR
        ),
    };

    match collection.update_one(doc! { "_id": obj_id }, update_doc).await {
        Ok(result) => {
            if result.matched_count == 1 {
                success_response(
                    "Resource updated successfully",
                    "Resource information updated",
                    StatusCode::OK
                )
            } else {
                error_response("Resource not found", StatusCode::NOT_FOUND)
            }
        }
        Err(err) => error_response(
            &format!("Database error: {}", err),
            StatusCode::INTERNAL_SERVER_ERROR
        ),
    }
}   
