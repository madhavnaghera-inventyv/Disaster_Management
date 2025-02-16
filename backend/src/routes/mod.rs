use axum::Router;
use crate::{shelters, user};
use crate::utils::db::AppState;

pub fn merge_routes(state: AppState) -> Router {
    Router::new()
        .nest("/user", user::create_routes(state.clone()))
        .nest("/shelters", shelters::shelters_routes(state.clone())) 
}
