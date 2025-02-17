use axum::{
    body::Body,
    http::{Request, StatusCode, HeaderValue},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use chrono::{Utc, Duration};
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};
use tower_cookies::{cookie::{time, Cookie, SameSite}, Cookies};
use crate::utils::db::AppState;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String, // Email
    pub exp: usize,
}

// Middleware function to rotate the token
pub async fn rotate_token_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let state = req.extensions().get::<Arc<AppState>>().cloned().unwrap();
    
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "disaster".to_string());

    // Get the token from the Authorization header or cookies
    let token_str = req.headers()
        .get("Authorization")
        .and_then(|hv| hv.to_str().ok())
        .map(|s| s.trim_start_matches("Bearer ").to_string())
        .or_else(|| {
            // Safely access cookies
            req.extensions()
                .get::<Cookies>()
                .and_then(|cookies| cookies.get("token").map(|c| c.value().to_string()))
        });

    // If a token exists, decode and verify it
    if let Some(token_str) = token_str {
        println!("Token found: {}", token_str); // Debug print
        
        if let Ok(token_data) = decode::<Claims>(
            &token_str,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        ) {
            let email = token_data.claims.sub.clone();
            let token_exp = token_data.claims.exp;
            println!("Token Decoded: {:?}", token_data); // Debug print

            // Check if the token is expired
            if token_exp > Utc::now().timestamp() as usize {
                println!("Token is valid, continuing with the request.");
                // Token is still valid, proceed with the request
                return Ok(next.run(req).await);
            }

            // Token expired, proceed to rotate the token
            println!("Token expired, rotating the token...");

            let db = state.db.clone();
            let collection = db.lock().await.database("disaster").collection::<Document>("users");

            // Fetch the user from the database
            if let Ok(Some(user_doc)) = collection.find_one(doc! { "email": &email }).await {
                if let Some(db_token) = user_doc.get_str("token").ok() {
                    // Compare the tokens
                    if db_token == token_str {
                        // Generate a new token
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

                        // Print the new rotated token
                        println!("New Rotated Token: {}", new_token);

                        // Update the token in the database with the new one
                        let update_result = collection.update_one(
                            doc! { "email": &email },
                            doc! { "$set": { "token": &new_token } },
                        ).await;

                        if update_result.is_ok() {
                            // Safely access cookies
                            if let Some(mut cookies) = req.extensions().get::<Cookies>().cloned() {
                                let mut response = next.run(req).await;

                                // Update the Authorization header with the new token
                                response.headers_mut().insert(
                                    "Authorization",
                                    HeaderValue::from_str(&format!("Bearer {}", new_token)).unwrap(),
                                );

                                // Set new cookie with the updated token
                                let mut new_cookie = Cookie::new("token", new_token.clone());
                                new_cookie.set_path("/");
                                new_cookie.set_http_only(true);
                                new_cookie.set_same_site(SameSite::Strict);
                                new_cookie.set_max_age(time::Duration::hours(1));

                                cookies.add(new_cookie);

                                // Return the response with the updated token
                                return Ok(response);
                            }
                        } else {
                            println!("Failed to update the token in the database.");
                        }
                    } else {
                        println!("Token mismatch! Stored token does not match provided token.");
                    }
                }
            }
        } else {
            println!("Failed to decode token.");
        }
    } else {
        println!("No token found in request.");
    }

    // Unauthorized if token is invalid or expired
    Err(StatusCode::UNAUTHORIZED)
}
