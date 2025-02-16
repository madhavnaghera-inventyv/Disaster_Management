use axum::Router;
use crate::{shelters, user, resources};
use crate::utils::db::AppState;

pub fn merge_routes(state: AppState) -> Router {
    Router::new()
        .nest("/user", user::user_routes(state.clone()))
        .nest("/shelters", shelters::shelters_routes(state.clone())) 
        .nest("/resources", resources::resources_routes(state.clone()))
}
