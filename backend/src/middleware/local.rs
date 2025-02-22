use axum::{
    extract::State,
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use mongodb::bson::{doc, oid::ObjectId};
use std::sync::Arc;
use crate::utils::db::AppState;

pub async fn local_middleware(
    State(state): State<Arc<AppState>>, 
    req: Request<Body>,
    next: Next,  
) -> Result<Response, StatusCode> {

    let user_id = req.extensions().get::<String>();

    if user_id.is_none() {
        println!("Id not found!");
    }
    let mut isLocal = false;
    if let Some(user_id) = user_id {
        
        let db = state.db.lock().await;
        let collection = db.database("disaster").collection::<mongodb::bson::Document>("users");

        if let Ok(Some(user_doc)) = collection.find_one(doc!{ "_id": ObjectId::parse_str(user_id).unwrap() }).await {
            if let Ok(role) = user_doc.get_str("role") {
                if role == "local" {
                    isLocal = true;
                }
            }
        }
    }
    if isLocal == true{
        return Ok(next.run(req).await); 
    }
    Ok((StatusCode::UNAUTHORIZED, "Unauthorized: Only Local role is allowed").into_response())
}

