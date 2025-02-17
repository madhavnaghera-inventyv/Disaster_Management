use axum::{
    extract::State,
    body::Body,
    http::{Request, StatusCode, HeaderValue},
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use chrono::{Utc, Duration};
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};
use tower_cookies::{cookie::{time, Cookie, SameSite}, Cookies};
use crate::utils::db::AppState;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Email
    pub exp: usize,
}

pub async fn rotate_token_middleware(
    State(state): State<Arc<AppState>>, 
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "disaster".to_string());

    // Extract token from Authorization header or Cookies
    let token_str = req.headers()
        .get("Authorization")
        .and_then(|hv| hv.to_str().ok())
        .map(|s| s.trim_start_matches("Bearer ").to_string())
        .or_else(|| cookies.get("token").map(|c| c.value().to_string()));

    if let Some(token_str) = token_str {
        // Decode the existing token
        if let Ok(token_data) = decode::<Claims>(
            &token_str,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        ) {
            let email = token_data.claims.sub.clone();
            let token_exp = token_data.claims.exp;

            // If the token is still valid, proceed with the request without modification
            if token_exp > Utc::now().timestamp() as usize {
                return next.run(req).await;
            }

            // Database operations
            let db = state.db.clone();
            let collection = db.lock().await.database("disaster").collection::<Document>("users");

            // Validate token against the database
            if let Ok(Some(user_doc)) = collection.find_one(doc! { "email": &email }).await {
                if let Some(db_token) = user_doc.get_str("token").ok() {
                    if db_token == token_str {
                        // Generate a new token with extended expiry
                        let new_exp = Utc::now() + Duration::hours(1);
                        let new_claims = Claims {
                            sub: email.clone(),
                            exp: new_exp.timestamp() as usize,
                        };

                        let new_token = encode(
                            &Header::default(),
                            &new_claims,
                            &EncodingKey::from_secret(secret.as_ref()),
                        ).expect("Failed to encode new JWT");

                        // Update the token in the database
                        let update_result = collection.update_one(
                            doc! { "email": &email },
                            doc! { "$set": { "token": &new_token } },
                        ).await;

                        if update_result.is_ok() {
                            let mut response = next.run(req).await;

                            // Update Authorization header with new token
                            response.headers_mut().insert(
                                "Authorization",
                                HeaderValue::from_str(&format!("Bearer {}", new_token)).unwrap(),
                            );

                            // Set Cookie Correctly
                            use tower_cookies::{Cookies, cookie::{Cookie, SameSite}};

                            // Create a new secure HTTP-only cookie for the refreshed token
                            let mut new_cookie = Cookie::new("token", new_token.clone());
                            new_cookie.set_path("/");
                            new_cookie.set_http_only(true);
                            new_cookie.set_same_site(SameSite::Strict);
                            new_cookie.set_max_age(time::Duration::hours(1)); // Use `time` crate for `Duration`

                            // Add cookie to response
                            cookies.add(new_cookie);


                            return response;
                        }
                    }
                }
            }
        }
    }

    (StatusCode::UNAUTHORIZED, "Unauthorized: Invalid or expired token").into_response()
}
