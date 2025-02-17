use std::sync::Arc;

use axum::{
    extract::State,
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use mongodb::bson::doc;
use crate::utils::db::AppState;

pub async fn admin_middleware(
    State(state): State<Arc<AppState>>, 
    req: Request<Body>, 
    next: Next
) -> Response {
    let headers = req.headers();

    // Extract user ID from the request headers
    let user_id = match headers.get("id") {
        Some(id) => id.to_str().ok(),
        None => None,
    };

    if let Some(user_id) = user_id {
        let db = state.db.lock().await;
        let collection = db.database("disaster").collection::<mongodb::bson::Document>("user");

        if let Ok(Some(user_doc)) = collection.find_one(doc! { "id": user_id }).await {
            if let Some(role) = user_doc.get_str("role").ok() {
                if role == "admin" {
                    return next.run(req).await; 
                }
            }
        }
    }

    (StatusCode::UNAUTHORIZED, "Unauthorized: Only admin role is allowed").into_response()
}
