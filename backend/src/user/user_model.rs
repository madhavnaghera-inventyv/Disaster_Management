use axum::{extract::State, Json};
use futures::TryStreamExt;
use mongodb::Collection;
use crate::{user::user_structure::User, utils::db::AppState};


pub async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let db = state.db.lock();
    let collection: Collection<User> = db.await.database("disaster").collection("user");
    match collection.find(mongodb::bson::Document::new()).await {

        Ok(cursor) => {
            let users: Vec<User> = cursor.try_collect().await.unwrap_or_else(|_| vec![]);
            Json(users)
        }
        Err(e) => {
            eprintln!("Error fetching users: {:?}", e);
            Json(vec![]) 
        }
    }
}