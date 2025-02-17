use axum::{
    body::{Body, Bytes},
    http::{Request, StatusCode},
    middleware::Next,
    response::{Response, IntoResponse},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use mongodb::{bson::doc, Client};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};
use crate::utils::db::AppState; // Your custom AppState containing database connection

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,  // User's email (subject)
    pub exp: usize,   // Expiration timestamp
}

pub async fn auth_middleware(
    req: Request<Body>,  // Accept any type of request body
    next: Next,   
) -> Result<Response<Body>, StatusCode> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
        println!("JWT_SECRET environment variable not found, using default value.");
        "disaster".to_string()
    });

    // Extract the Authorization header
    println!("Authorization Header: {:?}", req.headers().get("Authorization"));

    let token = req
        .headers()
        .get("Authorization")
        .and_then(|hv| hv.to_str().ok())
        .filter(|hv| hv.starts_with("Bearer ")) // Ensure the token starts with "Bearer "
        .map(|hv| hv.trim_start_matches("Bearer ").to_string());

    // If no token is found or the format is incorrect, return an Unauthorized status
    if token.is_none() {
        println!("Authorization header missing or invalid format.");
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Unauthorized: Invalid or missing CSRF token"))
            .unwrap());
    }

    // Extracted token (safe now as we checked it earlier)
    let token = token.unwrap();
    println!("Extracted Token: {}", token);

    // Decode the JWT token to validate it
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(token_data) => {
            let user_email = &token_data.claims.sub;
            println!("Token Decoded Successfully: Email -> {}", user_email);

            // Retrieve AppState (which contains the database connection) from the request extensions
            let state = match req.extensions().get::<Arc<AppState>>() {
                Some(state) => state.clone(),
                None => {
                    println!("Failed to retrieve AppState from request.");
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            };

            let db = state.db.clone(); // Clone the database handle
            let collection = db.lock().await
                .database("disaster")
                .collection::<mongodb::bson::Document>("users");

            // Validate token against the one stored in the database
            match collection.find_one(doc! { "email": user_email }).await {
                Ok(Some(user_doc)) => {
                    if let Some(db_token) = user_doc.get_str("token").ok() {
                        println!("Token found in DB: {}", db_token); // Debugging line

                        if db_token == token {
                            println!("Token validation successful.");
                            return Ok(next.run(req).await); // Proceed to the next middleware/handler
                        } else {
                            println!("Token mismatch! Stored: {}, Provided: {}", db_token, token);
                        }
                    } else {
                        println!("User exists but no token found in the database.");
                    }
                }
                Ok(None) => {
                    println!("User not found in the database.");
                }
                Err(e) => {
                    println!("Database query failed: {:?}", e);
                }
            }
        }
        Err(e) => {
            // If JWT decoding fails, log the error
            println!("JWT token decoding failed: {:?}", e);
        }
    }

    // If the token is invalid, missing, or decoding fails, return Unauthorized status
    println!("Authentication failed.");
    Ok(Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(Body::from("Unauthorized: Invalid or missing CSRF token"))
        .unwrap())
}
