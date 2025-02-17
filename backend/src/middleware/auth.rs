use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};
use crate::utils::db::AppState;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // User email
    pub exp: usize,   // Expiry timestamp
}

pub async fn auth_middleware<B>(
    req: Request<B>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "disaster".to_string());

    // Extract token from Authorization header
    let token = req.headers()
        .get("Authorization")
        .and_then(|hv| hv.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer ").map(String::from));

    if let Some(token) = token {
        // Decode and validate the JWT token
        if let Ok(token_data) = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        ) {
            let user_email = &token_data.claims.sub;

            // Get MongoDB state from request extensions
            if let Some(state) = req.extensions().get::<Arc<AppState>>() {
                let db = state.db.clone();
                let collection = db.lock().await
                    .database("disaster")
                    .collection::<mongodb::bson::Document>("users");

                // Validate token against stored value in database
                if let Ok(Some(user_doc)) = collection.find_one(doc! { "email": user_email }).await {
                    if let Some(db_token) = user_doc.get_str("token").ok() {
                        if db_token == token {
                            // Token is valid, allow request to proceed
                            let req = req.map(|_| Body::empty());
                            return Ok(next.run(req).await);

                        }
                    }
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
