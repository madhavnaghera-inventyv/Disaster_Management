use super::user_structure::{Claims, LoginRequest, RegisterRequest};
use crate::utils::db::AppState;
use argon2::password_hash::{rand_core::OsRng, PasswordHash, SaltString};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use axum::{
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::{bson::doc, Collection};
use serde_json::json;

const SECRET_KEY: &[u8] = b"disaster";

// ✅ **Hash Password**
fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed_password = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(hashed_password)
}

// ✅ **Verify Password**
fn verify_password(password: &str, hashed_password: &str) -> bool {
    let argon2 = Argon2::default();
    match PasswordHash::new(hashed_password) {
        Ok(parsed_hash) => argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok(),
        Err(_) => false,
    }
}

pub fn generate_jwt(email: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: email.to_owned(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    )
}

// ✅ **Register User**
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let db = state.db.lock().await;
    let collection: Collection<mongodb::bson::Document> =
        db.database("disaster").collection("users");

    // ❌ **Check if User Already Exists**
    if collection
        .find_one(doc! { "email": &payload.email })
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            )
        })?
        .is_some()
    {
        return Err((StatusCode::BAD_REQUEST, "User already exists".to_string()));
    }

    // ✅ **Hash Password**
    let hashed_password = hash_password(&payload.password).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to hash password".to_string(),
        )
    })?;

    // ✅ **Insert User with Token set to `null`**
    let new_user = doc! {
        "email": &payload.email,
        "password": hashed_password,
        "name": &payload.name,
        "token": null
    };

    collection.insert_one(new_user).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create user".to_string(),
        )
    })?;

    Ok(Json(json!({"message": "User registered successfully"})))
}

// ✅ **Login User**
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = state.db.lock().await;
    let collection: Collection<mongodb::bson::Document> =
        db.database("disaster").collection("users");

    // ❌ **Check if User Exists**
    let user = collection
        .find_one(doc! { "email": &payload.email })
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            )
        })?;

    match user {
        Some(user_doc) => {
            let stored_password = user_doc.get_str("password").unwrap_or_default();

            // ❌ **Incorrect Password**
            if !verify_password(&payload.password, stored_password) {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    "Invalid email or password".to_string(),
                ));
            }

            // ✅ **Generate JWT Token**
            let token = generate_jwt(&payload.email).map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to generate token".to_string(),
                )
            })?;

            // ✅ **Update Token in Database**
            collection
                .update_one(
                    doc! { "email": &payload.email },
                    doc! { "$set": { "token": token.clone() } },
                    
                )
                .await
                .map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to save token to database".to_string(),
                    )
                })?;

            // ✅ **Set Token in HTTP Headers & Cookies**
            let mut headers = HeaderMap::new();
            headers.insert(
                "Authorization",
                HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
            );  

            let cookie_value = format!(
                "token={}; HttpOnly; Path=/; Max-Age=86400; SameSite=Strict",
                token
            );
            headers.insert("Set-Cookie", HeaderValue::from_str(&cookie_value).unwrap());

            let response = (
                StatusCode::OK,
                headers,
                Json(json!({"message": "Login successful", "token": token})),
            );

            Ok(response)
        }
        None => Err((
            StatusCode::UNAUTHORIZED,
            "Invalid email or password".to_string(),
        )),
    }
}
