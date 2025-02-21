use std::sync::Arc;

use axum::middleware::from_fn;
use axum::{Extension, Router};
use tower::ServiceBuilder;
use crate::{shelters, user, disaster};
use crate::utils::db::AppState;
use crate::middleware::log::log_request;
use crate::resources;
use tower_http::cors::{CorsLayer, Any};
use http::{Request, Response, Method, header};

pub fn merge_routes(state: Arc<AppState>) -> Router  {
    let cors = CorsLayer::new()
    // allow GET and POST when accessing the resource
    .allow_methods([Method::GET, Method::POST])
    // allow requests from any origin
    .allow_origin(Any);


    Router::new()

        .nest("/shelters", shelters::shelters_routes(state.clone())) 
        .nest("/user", user::user_routes(state.clone()))
        .nest("/resources", resources::resources_routes((*state).clone()))
        .nest("/disaster", disaster::create_routes(state.clone())) 
        .layer(Extension(state.clone())) 
        .layer(from_fn(log_request))
        .layer(ServiceBuilder::new().layer(cors))
}