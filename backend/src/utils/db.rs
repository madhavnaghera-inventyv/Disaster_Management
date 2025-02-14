use lazy_static::lazy_static;
use mongodb::Client;
use tokio::sync::Mutex;
use std::{env, sync::Arc};
use dotenv::dotenv;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Client>>, 
}

lazy_static! {
    pub static ref DB_CLIENT: Arc<Mutex<Option<Client>>> = Arc::new(Mutex::new(None)); 
}

pub async fn initialize_db() -> AppState {
    
    dotenv().ok();
    let database_uri = env::var("DATABASE_URI").expect("DATABASE_URI must be set in .env");
    
    let client = Client::with_uri_str(&database_uri)
        .await
        .expect("Failed to connect to MongoDB!");

    let mut db_lock = DB_CLIENT.lock().await;
    *db_lock = Some(client.clone()); 

    AppState {
        db: Arc::new(Mutex::new(client)),
    }
}
