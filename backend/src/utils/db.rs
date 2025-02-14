use lazy_static::lazy_static;
use mongodb::Client;
use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Client>>, 
}

lazy_static! {
    pub static ref DB_CLIENT: Arc<Mutex<Option<Client>>> = Arc::new(Mutex::new(None)); 
}

pub async fn initialize_db() -> AppState {
    let client = Client::with_uri_str("mongodb+srv://smit:dankhra11@cluster0.vn4j6hi.mongodb.net/disaster")
        .await
        .expect("Failed to connect to MongoDB!");

    let mut db_lock = DB_CLIENT.lock().await;
    *db_lock = Some(client.clone()); 

    AppState {
        db: Arc::new(Mutex::new(client)),
    }
}
