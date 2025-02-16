mod utils;
use utils::db::initialize_db; 
mod routes;
mod user;
use routes::merge_routes;
mod shelters;
#[tokio::main]
async fn main() {
    let state = initialize_db().await;
    
    let app = merge_routes(state);

    let listener= tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("server running on port 0.0.0.0:8000");
    axum::serve(listener,app).await.unwrap();
}
