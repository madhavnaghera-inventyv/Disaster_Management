use axum::Router;
use crate::{disaster, user};
use crate::utils::db::AppState;

pub fn merge_routes(state: AppState) -> Router {
    Router::new()
        .nest("/disaster", disaster::create_routes(state.clone())) 
}
