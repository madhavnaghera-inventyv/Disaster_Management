use axum::middleware::from_fn;
use axum::{Extension, Router};
use crate::{shelters, user};
use crate::utils::db::AppState;
use crate::middleware::auth::auth_middleware;
use crate::middleware::log::log_request;

pub fn merge_routes(state: AppState) -> Router {
    Router::new()
        .nest("/shelters", shelters::shelters_routes(state.clone())) 
        // .layer(from_fn(auth_middleware))
        .nest("/user", user::user_routes(state.clone()))
        .layer(Extension(state.clone())) 
        .layer(from_fn(log_request))
}
