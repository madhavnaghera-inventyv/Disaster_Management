use axum::{
    extract::State,
    body::Body,
    http::{Request, StatusCode, HeaderValue},
    middleware::Next,
    response::{IntoResponse, Response},
};
use mongodb::bson::{doc, Document};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use chrono::{Utc, Duration};
use std::{sync::Arc, env};
use crate::utils::db::AppState;
use crate::middleware::auth::Claims;

pub async fn rotate_token_middleware(
    State(state): State<Arc<AppState>>, // Use Arc to prevent locking
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let headers = req.headers();

    // Extract token from Authorization header
    let token_str = headers.get("Authorization")
        .and_then(|val| val.to_str().ok())
        .and_then(|val| val.strip_prefix("Bearer "))
        .unwrap_or("");

    if !token_str.is_empty() {
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "supersecret".to_string());

        // Decode existing token
        if let Ok(token) = decode::<Claims>(
            token_str,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        ) {
            let user_id = token.claims.user_id.clone();
            let username = token.claims.username.clone();
            let token_exp = token.claims.exp;

            // Prevent unnecessary token rotation
            if token_exp > Utc::now().timestamp() as usize {
                return next.run(req).await;
            }

            // Access MongoDB without blocking
            let db = state.db.clone();
            let collection = db.lock().await.database("disaster").collection::<Document>("users");

            // Validate token against database
            if let Ok(Some(user_doc)) = collection.find_one(doc! { "id": &user_id }).await {
                if let Some(db_token) = user_doc.get_str("token").ok() {
                    if db_token == token_str {
                        // Generate a new token with extended expiry
                        let new_exp = Utc::now() + Duration::hours(1);
                        let new_claims = Claims {
                            user_id: user_id.clone(),
                            username: username.clone(),
                            exp: new_exp.timestamp() as usize,
                        };

                        let new_token = encode(
                            &Header::default(),
                            &new_claims,
                            &EncodingKey::from_secret(secret.as_ref()),
                        ).expect("Failed to encode new JWT");

                        // Update the token in the database
                        if collection.update_one(
                            doc! { "id": &user_id },
                            doc! { "$set": { "token": &new_token } }
                        ).await.is_ok() {
                            let mut response = next.run(req).await;
                            response.headers_mut().insert(
                                "Authorization",
                                HeaderValue::from_str(&format!("Bearer {}", new_token)).unwrap(),
                            );
                            return response;
                        }
                    }
                }
            }
        }
    }

    (StatusCode::UNAUTHORIZED, "Unauthorized: Invalid or expired token").into_response()
}
