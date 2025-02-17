use axum::{body::Body, middleware::{from_fn_with_state}, Router};
use std::sync::Arc;
mod auth;
mod rotate_token;
use crate::utils::db::AppState;

pub fn merge_middlewares(state: Arc<AppState>) -> Router {
    Router::new()
        .layer(from_fn_with_state(state.clone(), auth::auth::<Body>))
        .layer(from_fn_with_state(state.clone(),rotate_token::rotate_token_middleware))
}
