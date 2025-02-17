use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};
use crate::utils::db::AppState;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,  // User's email (subject)
    pub exp: usize,   // Expiration timestamp
}

pub async fn auth_middleware<B>(
    mut req: Request<axum::body::Body>,
    next: Next, 
) -> Response {
    // Fetch the JWT secret (or use a default value if not set in the environment)
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
        println!("JWT_SECRET environment variable not found, using default value.");
        "disaster".to_string()
    });

    println!("Authorization Header: {:?}", req.headers().get("Authorization"));

    let token = req
        .headers()
        .get("Authorization")
        .and_then(|hv| hv.to_str().ok())
        .filter(|hv| hv.starts_with("Bearer ")) // Ensure the token starts with "Bearer "
        .map(|hv| hv.trim_start_matches("Bearer ").to_string());

    // If no token is found or if the format is incorrect, return an Unauthorized status
    if token.is_none() {
        println!("Authorization header missing or invalid format.");
        return Err(StatusCode::UNAUTHORIZED).into_response();
    }

    // Unwrap the token (safe now because we checked it above)
    let token = token.unwrap();
    println!("Extracted Token: {}", token);

   
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(token_data) => {
            let user_email = &token_data.claims.sub;
            println!("Token Decoded Successfully: Email -> {}", user_email);

            // Retrieve AppState from request extensions (needed to access the database)
            let state = match req.extensions().get::<Arc<AppState>>() {
                Some(state) => state.clone(),
                None => {
                    println!("Failed to retrieve AppState from request.");
                    return Err(StatusCode::INTERNAL_SERVER_ERROR).into_response();
                }
            };

            let db = state.db.clone();
            let collection = db.lock().await
                .database("disaster")
                .collection::<mongodb::bson::Document>("users");

            // Validate token against stored value in the database
            match collection.find_one(doc! { "email": user_email }).await {
                Ok(Some(user_doc)) => {
                    println!("{}",user_doc);
                    if let Some(db_token) = user_doc.get_str("token").ok() {
                        println!("Token found in DB: {}", db_token); // Debugging line

                        if db_token == token {
                            println!("Token validation successful.");
                            // let req = req.map(|_| Body::empty());
                            // return Ok(next.run(req).await);
                            next.run(req).await.into_response()
                            
                            
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

    // If token is invalid or missing, return Unauthorized status
    println!("Authentication failed.");
    Err(StatusCode::UNAUTHORIZED)
}

