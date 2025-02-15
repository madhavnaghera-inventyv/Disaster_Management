use axum::{extract::State, Json};
use futures::TryStreamExt;
use mongodb::Collection;
use serde_json::json;
use crate::{disaster::disaster_structure::{DisasterRecord, DisasterGuide}, utils::db::AppState};

pub async fn add_disaster(State(state):State<AppState>,
Json(req_record): Json<DisasterRecord>) -> Json<serde_json::Value>{
    println!("{:?}", req_record);
    let db = state.db.lock();
    let collection: Collection<DisasterRecord> = db.await.database("disaster").collection("disaster_record");

    let new_disaster_record = DisasterRecord {
        id: None,  // Let MongoDB generate `_id`
        name: req_record.name,
        effects: req_record.effects,
        short_description: req_record.short_description,
        youtube_link: req_record.youtube_link,
    };

    let result = match collection.insert_one(&new_disaster_record).await {
        Ok(insert_result) => Json(json!({
            "message": "Disaster record added successfully",
            "inserted_id": insert_result.inserted_id
        })),
        Err(e) => Json(json!({
            "error": format!("Failed to insert record: {}", e)
        })),
    };
    println!("{:?}", result);
    result
    // Json(DisasterRecord{
    //     id:None,
    //     name:String::from("dfsdfds"),
    //     effects:String::from("dsfdsfdsfs"),
    //     short_description:String::from("dsfndsfldskfmdfsoffsoin"),
    //     youtube_link:String::from("https://www.youtube.com/")
    // })
}