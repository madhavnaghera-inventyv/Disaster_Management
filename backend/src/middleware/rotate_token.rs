// use axum::{
//     body::Body,
//     http::{Request, StatusCode, HeaderValue},
//     middleware::Next,
//     response::Response,
// };
// use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
// use chrono::{Utc, Duration};
// use mongodb::bson::{doc, Document};
// use serde::{Deserialize, Serialize};
// use std::{env, sync::Arc};
// use tower_cookies::{cookie::{time, Cookie, SameSite}, Cookies};
// use crate::utils::db::AppState;

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Claims {
//     pub sub: String, // Email
//     pub exp: usize,
// }

// // Middleware function to rotate the token
// pub async fn rotate_token_middleware(
//     mut req: Request<Body>, 
//     next: Next,
// ) -> Result<Response<Body>, StatusCode> {

//     let mut response = next.run(req).await;

//     let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "disaster".to_string());

//     let csrf_token_from_header = req.headers().get("X-CSRF-Token").and_then(|hv| hv.to_str().ok()).map(|s| s.to_string());

//     let token_str = req.headers()
//         .get("Authorization")
//         .and_then(|hv| hv.to_str().ok())
//         .map(|s| s.trim_start_matches("Bearer ").to_string())
//         .or_else(|| {
//             req.extensions()
//                 .get::<Cookies>()
//                 .and_then(|cookies| cookies.get("token").map(|c| c.value().to_string()))
//         });
//     }








   
// }
    

