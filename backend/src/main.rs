mod utils;
use std::sync::Arc;

use utils::db::initialize_db; 
mod routes;
mod resources;
mod  middleware;
mod user;
mod disaster;
use routes::merge_routes;
mod shelters;
#[tokio::main]
async fn main() {
    let state = Arc::new(initialize_db().await);  
    let app = merge_routes(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("server running on port 0.0.0.0:8000");
    axum::serve(listener, app).await.unwrap();
}
