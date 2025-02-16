use axum::{
    body::Body,
    extract::{State, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;
use std::{convert::Infallible, env, sync::Arc};
use crate::utils::db::AppState;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub username: String,
    pub exp: usize,
}

pub async fn auth<B>(
    State(state): State<Arc<AppState>>, // Use Arc to avoid locks
    cookies: Cookies,
    req: Request<Body>,
    next: Next,
) -> Result<Response, Infallible> {
    if let Some(cookie) = cookies.get("csrf_token") {
        let csrf_token = cookie.value().to_string();
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "supersecret".to_string());

        if let Ok(token) = decode::<Claims>(
            &csrf_token, 
            &DecodingKey::from_secret(secret.as_ref()), 
            &Validation::default()
        ) {
            let user_id = token.claims.user_id.clone();
            let username = token.claims.username.clone();

            let db = state.db.clone();
            let collection = db.lock().await.database("disaster").collection::<mongodb::bson::Document>("users");

            // Ensure MongoDB `_id` handling works for both string and ObjectId
            let filter = doc! {
                "$or": [
                    { "_id": ObjectId::parse_str(&user_id).ok() },
                    { "id": &user_id }
                ],
                "username": &username
            };

            if let Ok(Some(user_doc)) = collection.find_one(filter).await {
                if let Some(db_token) = user_doc.get_str("token").ok() {
                    if db_token == csrf_token {
                        return Ok(next.run(req.map(Body::from)).await);
                    }
                }
            }
        }
    }

    Ok(Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(Body::from("Unauthorized: Invalid or missing CSRF token"))
        .unwrap())
}
