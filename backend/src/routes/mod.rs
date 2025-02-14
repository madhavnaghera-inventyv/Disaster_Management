use axum::Router;
use crate::user;
use crate::utils::db::AppState;

pub fn merge_routes(state: AppState) -> Router {
    Router::new()
        .nest("/user", user::create_routes(state)) 
}
