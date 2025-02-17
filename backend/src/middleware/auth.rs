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

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,  
    pub exp: usize,   
}

// pub async fn auth_middleware<B>(

//     req: Request<B>,
//     next: Next,
// ) -> Result<Response<Body>, StatusCode> {
//     let secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
//         println!("JWT_SECRET environment variable not found, using default value.");
//         "disaster".to_string()
//     });
//     println!("Authorization Header: {:?}", req.headers().get("Authorization"));


//     // Extract token from Authorization header
//     let token = match req.headers().get("Authorization") {
//         Some(hv) => match hv.to_str() {
//             Ok(header_value) => {
//                 if header_value.starts_with("Bearer ") {
//                     Some(header_value.trim_start_matches("Bearer ").to_string())
//                 } else {
//                     println!("Authorization header found but missing 'Bearer ' prefix: {}", header_value);
//                     None
//                 }
//             }
//             Err(_) => {
//                 println!("Authorization header is not a valid string");
//                 None
//             }
//         },
//         None => {
//             println!("No Authorization header found");
//             None
//         }
//     };
    

//     if token.is_none() {
//         println!("Authorization header missing or invalid format.");
//         return Err(StatusCode::UNAUTHORIZED);
//     }

//     if let Some(token) = token {
//         println!("Extracted Token: {}", token);

//         // Decode and validate the JWT token
//         match decode::<Claims>(
//             &token,
//             &DecodingKey::from_secret(secret.as_ref()),
//             &Validation::default(),
//         ) {
//             Ok(token_data) => {
//                 println!("Token Decoded Successfully: ", 
//             );
//                 let user_email = &token_data.claims.sub;
//                 println!("Token Decoded Successfully: Email -> {}", user_email);

//                 // Get MongoDB state from request extensions
//                 if let Some(state) = req.extensions().get::<Arc<AppState>>() {
//                     let db = state.db.clone();
//                     let collection = db.lock().await
//                         .database("disaster")
//                         .collection::<mongodb::bson::Document>("users");

//                     // Validate token against stored value in database
//                     match collection.find_one(doc! { "email": user_email }).await {
//                         Ok(Some(user_doc)) => {
//                             println!("User found in database: {}", user_email);
                            
//                             if let Some(db_token) = user_doc.get_str("token").ok() {
//                                 println!("Stored Token: {}", db_token);

//                                 if db_token == token {
//                                     println!("Token validation successful.");
//                                     let req = req.map(|_| Body::empty());
//                                     return Ok(next.run(req).await);
//                                 } else {
//                                     println!("Stored token does not match provided token.");
//                                 }
//                             } else {
//                                 println!("No token found in the database for user.");
//                             }
//                         }
//                         Ok(None) => println!("No user found with email: {}", user_email),
//                         Err(e) => println!("Database query failed: {:?}", e),
//                     }
//                 } else {
//                     println!("Failed to retrieve AppState from request.");
//                 }
//             }
//             Err(e) => {
//                 println!("JWT token decoding failed: {:?}", e);
//             }
//         }
//     }

//     println!("Authentication failed.");
//     Err(StatusCode::UNAUTHORIZED)
// }



pub async fn auth_middleware<B>(
    req: Request<B>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
        println!("JWT_SECRET environment variable not found, using default value.");
        "disaster".to_string()
    });

    println!("Authorization Header: {:?}", req.headers().get("Authorization"));

    // Extract token from Authorization header
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|hv| hv.to_str().ok())
        .filter(|hv| hv.starts_with("Bearer "))
        .map(|hv| hv.trim_start_matches("Bearer ").to_string());

    if token.is_none() {
        println!("Authorization header missing or invalid format.");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = token.unwrap();
    println!("Extracted Token: {}", token);

    // Decode and validate the JWT token
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(token_data) => {
            let user_email = &token_data.claims.sub;
            println!("Token Decoded Successfully: Email -> {}", user_email);

            // ✅ Implicitly retrieve AppState from request
            let state = match req.extensions().get::<Arc<AppState>>() {
                Some(state) => state.clone(),
                None => {
                    println!("Failed to retrieve AppState from request.");
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            };

            let db = state.db.clone();
            let collection = db.lock().await
                .database("disaster")
                .collection::<mongodb::bson::Document>("users");

            // Validate token against stored value in database
            match collection.find_one(doc! { "email": user_email }).await {
                Ok(Some(user_doc)) => {
                    println!("User found in database: {}", user_email);
                    
                    if let Some(db_token) = user_doc.get_str("token").ok() {
                        println!("Stored Token: {}", db_token);

                        if db_token == token {
                            println!("Token validation successful.");
                            let req = req.map(|_| Body::empty());
                            return Ok(next.run(req).await);
                        } else {
                            println!("Stored token does not match provided token.");
                        }
                    } else {
                        println!("No token found in the database for user.");
                    }
                }
                Ok(None) => println!("No user found with email: {}", user_email),
                Err(e) => println!("Database query failed: {:?}", e),
            }
        }
        Err(e) => {
            println!("JWT token decoding failed: {:?}", e);
        }
    }

    println!("Authentication failed.");
    Err(StatusCode::UNAUTHORIZED)
}
