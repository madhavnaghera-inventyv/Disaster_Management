use std::sync::Arc;

use axum::middleware::from_fn;
use axum::{Extension, Router};
use crate::{shelters, user, disaster};
use crate::utils::db::AppState;
use crate::middleware::log::log_request;
use crate::resources;

pub fn merge_routes(state: Arc<AppState>) -> Router  {
    Router::new()

        .nest("/shelters", shelters::shelters_routes(state.clone())) 
        .nest("/user", user::user_routes(state.clone()))
        .nest("/resources", resources::resources_routes((*state).clone()))
        .nest("/disaster", disaster::create_routes((*state).clone())) 
        .layer(Extension(state.clone())) 
        .layer(from_fn(log_request))
}
