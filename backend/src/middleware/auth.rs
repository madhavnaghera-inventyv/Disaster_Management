use std::{env, sync::Arc};

use axum::{body::Body, extract::Request, http::HeaderValue, middleware::Next, response::Response};
use hyper::StatusCode;
use jsonwebtoken::{decode, DecodingKey, Validation};
use mongodb::bson::{doc, Bson};


use crate::{user::{user_model::generate_jwt, user_structure::Claims}, utils::db::AppState};

pub async fn auth_middleware(
    mut req: Request<Body>,  
    next: Next,   
) -> Result<Response, StatusCode> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "disaster".to_string());

    let token = req
        .headers()
        .get("Authorization")
        .and_then(|hv| hv.to_str().ok())
        .filter(|hv| hv.starts_with("Bearer "))
        .map(|hv| hv.trim_start_matches("Bearer ").to_string());

    if token.is_none() {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Unauthorized: Invalid or missing token"))
            .unwrap());
    }

    let token = token.unwrap();

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(token_data) => {
            let user_email = &token_data.claims.sub;
            let state = match req.extensions().get::<Arc<AppState>>() {
                Some(state) => state.clone(),
                None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            };

            let db = state.db.clone(); 
            let collection = db.lock().await
                .database("disaster")
                .collection::<mongodb::bson::Document>("users");

            match collection.find_one(doc! { "email": user_email }).await {
                
                Ok(Some(user_doc)) => {
                    if let Some(db_token) = user_doc.get_str("token").ok() {
                        if db_token == token {
                            // let new_token = match generate_jwt(user_email) {
                            //     Ok(new_token) => new_token,
                            //     Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
                            // };


                            // println!("New Token: {}", new_token);
                            
                           
                            // Update token in DB
                            // collection.update_one(
                                // doc! { "email": user_email },
                                // doc! { "$set": { "token": new_token.clone() } },
                            // )
                            // .await
                            // .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                            let user_id = user_doc.get("_id")
                                .and_then(Bson::as_object_id)
                                .map(|oid| oid.to_string())
                                .unwrap_or_else(|| "unknown".to_string());
                                
                                req.extensions_mut().insert(user_id.clone());

                                let response = next.run(req).await; // Pass the modified request forward
                                
                               
                            
                            // headers.insert(
                            //     "Authorization",
                            //     HeaderValue::from_str(&format!("Bearer {}", new_token)).unwrap(),
                            // );

                            // // Set the token in a cookie
                            // let cookie_value = format!(
                            //     "token={}; HttpOnly; Path=/; Max-Age=86400; SameSite=Strict",
                            //     new_token
                            // );
                            // headers.insert("Set-Cookie", HeaderValue::from_str(&cookie_value).unwrap());

                            return Ok(response);
                        }
                    }
                }
                _ => {}
            }
        }
        Err(_) => {}
    }

    Ok(Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(Body::from("Unauthorized: Invalid or expired token"))
        .unwrap())
}
