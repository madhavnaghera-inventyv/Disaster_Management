use axum::{
    extract::State, 
    http::StatusCode, 
    response::IntoResponse, 
    Json
};
use crate::utils::db::AppState;
use super::{user_model, user_structure::{LoginRequest, RegisterRequest}};


pub async fn login_service(
    State(_state): State<AppState>, 
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if payload.email.trim().is_empty() || payload.password.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Email and password must not be empty".to_string()));
    }
    user_model::login(State(_state), Json(payload)).await
}

pub async fn register_service(State(_state): State<AppState>, Json(payload): Json<RegisterRequest>) -> Result<impl IntoResponse, (StatusCode, String)> {
    if payload.email.trim().is_empty() || payload.password.trim().is_empty() || payload.name.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Email and password must not be empty".to_string()));
    }
    user_model::register(State(_state), Json(payload)).await
}
