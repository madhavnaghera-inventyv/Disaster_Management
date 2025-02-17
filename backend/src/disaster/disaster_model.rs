use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use futures::{StreamExt, TryStreamExt};
use serde_json::json;
use crate::{disaster::disaster_structure::{DisasterGuide, DisasterRecord, GuideItem, CombinedData}, utils::{db::AppState, response::{error_response, success_response}}};
use mongodb::{
    bson::{doc, from_document, oid::ObjectId, to_bson, Bson}, options::{ClientOptions, FindOptions, IndexOptions}, Client, Collection, IndexModel
};




//Create Index
// async fn create_indexes(State(state):State<AppState>) -> Result<(), Box<dyn Error>> {
//     // Connect to MongoDB
//     let db = state.db.lock();
//     let collection: Collection<DisasterRecord> = db.await.database("disaster").collection("disaster_record");

//     // Define the unique index for the "name" field
//     let index_model = IndexModel::builder()
//         .keys(doc! { "name": 1 })  // Index on "name" field
//         .options(IndexOptions::builder().unique(true).build())  // Make it unique
//         .build();

//     // Create the index
//     collection.create_index(index_model).await?;

//     println!("Unique index on 'name' field created successfully!");

//     Ok(())
// }

// #[tokio::main]
// async fn main() {
//     if let Err(e) = create_indexes().await {
//         eprintln!("Error creating index: {}", e);
//     }
// }






pub async fn add_disaster(
    State(state): State<AppState>,
    Json(req_record): Json<DisasterRecord>,
) -> impl IntoResponse {
    // Debug: log incoming request record
    // println!("{:?}", req_record);
    
    let db = state.db.lock().await;
    let dr_collection: Collection<DisasterRecord> = db.database("disaster").collection("disaster_record");

    // Create a new disaster record
    let new_disaster_record = DisasterRecord {
        id: None,  // Let MongoDB generate `_id`
        name: req_record.name,
        effects: req_record.effects,
        short_description: req_record.short_description,
        youtube_link: req_record.youtube_link,
    };

    // Insert the new disaster record into the collection
    let dr_bson_id = match dr_collection.insert_one(&new_disaster_record).await {
        Ok(insert_result) => insert_result.inserted_id,
        Err(e) => return error_response("Failed to insert record", StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Extract the ObjectId from the inserted record
    let dr_id = match dr_bson_id {
        Bson::ObjectId(oid) => oid,
        _ => return error_response("Failed to extract ObjectId from inserted record", StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Create a DisasterGuide entry linked to the new disaster record
    let dg_collection: Collection<DisasterGuide> = db.database("disaster").collection("disaster_guide");
    let dg_entry = DisasterGuide {
        id: None,
        disaster_id: dr_id,
        do_s: vec![],
        dont_s: vec![],
    };

    // Insert the disaster guide entry
    let dg_bson_id = match dg_collection.insert_one(&dg_entry).await {
        Ok(insert_result) => insert_result.inserted_id,
        Err(e) => return error_response("Failed to insert guide record", StatusCode::INTERNAL_SERVER_ERROR)
    };

    // Respond with success message
    success_response("New disaster record created successfully.", dr_id.to_string() , StatusCode::OK)
}

pub async fn add_dos(State(state): State<AppState>,
Path(dr_id): Path<ObjectId>,  // Extract `dr_id` from the URL
Json(req_message): Json<serde_json::Value>) -> impl IntoResponse{

    let db = state.db.lock().await;
    let dg_collection: Collection<DisasterGuide> = db.database("disaster").collection("disaster_guide");

    //println!("Updating disaster guide for ID: {:?} with data: {:?}", dr_id, req_message);

    let do_message = req_message.get("message").and_then(|m| m.as_str()).unwrap(); // We assume it's always present.

    let id_str: &str = "67b17ff47acc96908fe325d8";  // Placeholder user_id for demonstration
    let _do = GuideItem {
        id: Some(ObjectId::new()),
        user_id: match ObjectId::parse_str(id_str) {
            Ok(object_id) => object_id,
            Err(e) => {
                return error_response("Failed to parse User ID", StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        status: String::from("Accepted"),
        message: String::from(do_message),
    };

    let do_bson = match to_bson(&_do) {
        Ok(bson) => bson,
        Err(e) => {
            return error_response("Failed to serialize", StatusCode::INTERNAL_SERVER_ERROR)
        }
    };

    let update_result = dg_collection
        .update_one(
            doc! { "disaster_id": dr_id },
            doc! { "$push": { "do_s": do_bson } }
        )
        .await;

        match update_result {
            Ok(update_result) => {
                if update_result.matched_count == 0 {
                    return error_response("No record found", StatusCode::NOT_FOUND);
                }
                success_response(
                    "Successfully added to do_s array",
                    json!({
                        "disaster_id": dr_id,
                        "new_do": do_message
                    }),
                    StatusCode::OK
                )
            }
            Err(_) => {
                return error_response("Failed to update record", StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
   
}



pub async fn add_donts(
    State(state): State<AppState>,
    Path(dr_id): Path<ObjectId>,
    Json(req_message): Json<serde_json::Value>,
) -> impl IntoResponse {
    let db = state.db.lock().await;
    let dg_collection: Collection<DisasterGuide> = db.database("disaster").collection("disaster_guide");

    //println!("Updating disaster guide for ID: {:?} with data: {:?}", dr_id, req_message);

    // Directly access the message field since it's already validated by add_donts_service
    let dont_message = req_message.get("message").and_then(|m| m.as_str()).unwrap(); // We assume it's always present.

    let id_str: &str = "67b17ff47acc96908fe325d8";  // Placeholder user_id for demonstration
    let _dont = GuideItem {
        id: Some(ObjectId::new()),
        user_id: match ObjectId::parse_str(id_str) {
            Ok(object_id) => object_id,
            Err(e) => {
                return error_response(
                    &format!("Failed to parse user ID: {}", e),
                    StatusCode::BAD_REQUEST,
                );
            }
        },
        status: String::from("Accepted"),
        message: String::from(dont_message),
    };

    let dont_bson = match to_bson(&_dont) {
        Ok(bson) => bson,
        Err(e) => {
            return error_response(
                &format!("Failed to serialize GuideItem: {}", e),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
        }
    };

    let update_result = dg_collection
        .update_one(
            doc! { "disaster_id": dr_id },
            doc! { "$push": { "dont_s": dont_bson } }
        )
        .await;

    match update_result {
        Ok(update_result) => {
            if update_result.matched_count == 0 {
                return error_response("No record found with the given ID", StatusCode::NOT_FOUND);
            }
            success_response(
                "Successfully added to dont_s array",
                json!({
                    "disaster_id": dr_id,
                    "new_dont": dont_message
                }),
                StatusCode::OK,
            )
        }
        Err(e) => {
            error_response(
                &format!("Failed to update record: {}", e),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}



// pub async fn get_dos_donts(State(state):State<AppState>,
// Path(dr_id): Path<ObjectId>
// ) -> Json<Vec<DisasterGuide>>{
//     let db = state.db.lock().await;
//     let dg_collection: Collection<DisasterGuide> = db.database("disaster").collection("disaster_guide");

//     println!("Getting dos and donts for disaster guide for ID: {:?}", dr_id);

//     // let filter = doc! {
//     //     "disaster_id": dr_id
//     // };

//     // // Projection to return only do_s and dont_s with status "Accepted"
//     // let projection = doc! {
//     //     "_id": 1,
//     //     "disaster_id": 1,
//     //     "do_s": { "$filter": {
//     //         "input": "$do_s",
//     //         "as": "item",
//     //         "cond": { "$eq": ["$$item.status", "Accepted"] }
//     //     }},
//     //     "dont_s": { "$filter": {
//     //         "input": "$dont_s",
//     //         "as": "item",
//     //         "cond": { "$eq": ["$$item.status", "Accepted"] }
//     //     }}
//     // };

//     // let find_options = FindOptions::builder()
//     // .projection(projection)
//     // .build();

//     // match dg_collection.find(filter).await{
//     //     Ok(cursor) =>{
//     //         let dg_data = cursor.try_collect().await.unwrap_or_else(|_| vec![]);
//     //         Json(dg_data)
//     //     }
//     //     Err(e) => {
//     //         eprintln!("Error fetching users: {:?}", e);
//     //         Json(vec![]) 
//     //     }
//     // }

// let pipeline = vec![
//     doc! { "$match": { "disaster_id": dr_id } },
//     doc! { "$project": {
//         "_id": 1,
//         "disaster_id": 1,
//         "do_s": {
//             "$filter": {
//                 "input": "$do_s",
//                 "as": "item",
//                 "cond": { "$eq": ["$$item.status", "Accepted"] }
//             }
//         },
//         "dont_s": {
//             "$filter": {
//                 "input": "$dont_s",
//                 "as": "item",
//                 "cond": { "$eq": ["$$item.status", "Accepted"] }
//             }
//         }
//     }}
// ];

// let mut cursor = dg_collection.aggregate(pipeline, None).await.unwrap();

// let mut dg_data = vec![];
// while let Some(doc) = cursor.try_next().await.unwrap() {
//     dg_data.push(doc);
// }

// Json(dg_data)


// }


pub async fn get_disaster_record(
    State(state): State<AppState>,
    Path(dr_id): Path<ObjectId>,
) -> impl IntoResponse {
    let db = state.db.lock().await;
    let dg_collection: Collection<DisasterGuide> = db.database("disaster").collection("disaster_guide");
    let dr_collection: Collection<DisasterRecord> = db.database("disaster").collection("disaster_record");

    //println!("Fetching disaster record and accepted dos and don'ts for disaster ID: {:?}", dr_id);

    let dg_pipeline = vec![

    // Project the necessary fields
    doc! {
        "$project": {
            "_id": 1,                             // Include _id from disaster_guide
            "disaster_id": 1,                     // Include disaster_id from disaster_guide
            "disaster_record": 1,                 // Include full disaster_record data
            "do_s": {
                "$filter": {
                    "input": "$do_s",          // Filter the do_s array
                    "as": "item",               // Alias for each element in do_s
                    "cond": {                   // Condition to filter only accepted items
                        "$eq": ["$$item.status", "Accepted"]
                    }
                }
            },
            "dont_s": {
                "$filter": {
                    "input": "$dont_s",         // Filter the dont_s array
                    "as": "item",               // Alias for each element in dont_s
                    "cond": {                   // Condition to filter only accepted items
                        "$eq": ["$$item.status", "Accepted"]
                    }
                }
            }
        }
    },

    // Match disaster_id to fetch only the relevant records
    doc! { "$match": { "disaster_id": dr_id } }
];


    let dg_data = match dg_collection.aggregate(dg_pipeline).await {
        Ok(mut cursor) => {
            let mut dg_data: Vec<DisasterGuide> = Vec::new();

            while let Some(doc) = cursor.try_next().await.unwrap_or(None) {
                match from_document::<DisasterGuide>(doc) {
                    Ok(disaster_guide) => dg_data.push(disaster_guide),
                    Err(e) => eprintln!("Error deserializing document: {:?}", e),
                }
            }

            if dg_data.is_empty() {
                return error_response("No matching records found", StatusCode::NOT_FOUND);
            }
            dg_data
        }
        Err(e) => {
            eprintln!("Error executing aggregation: {:?}", e);
            return error_response("Failed to fetch disaster record", StatusCode::INTERNAL_SERVER_ERROR)
        }
    };


    let filter = doc! { "_id": dr_id };  // Filter by ObjectId

    let dr_data = match dr_collection.find(filter).await {
        Ok(cursor) => {
            // Collect the documents matching the filter (in this case, it should be one or none)
            let disaster_records: Vec<DisasterRecord> = cursor.try_collect().await.unwrap_or_else(|_| vec![]);
            disaster_records
        }
        Err(e) => {
            eprintln!("Error executing aggregation: {:?}", e);
            return error_response("Failed to fetch disaster record", StatusCode::INTERNAL_SERVER_ERROR)
        }
    };

    let response_data = json!({
        "disaster_record": dr_data,
        "disaster_guide": dg_data
        
    });

    success_response(
                "Successfully fetched disaster record with accepted dos and donts",
                response_data,
                StatusCode::OK,
            )
}





pub async fn get_all_disaster_record(
    State(state): State<AppState>,
    Path(dr_id): Path<ObjectId>,
) -> impl IntoResponse {
    let db = state.db.lock().await;
    let dg_collection: Collection<DisasterGuide> = db.database("disaster").collection("disaster_guide");
    let dr_collection: Collection<DisasterRecord> = db.database("disaster").collection("disaster_record");

    //println!("Fetching disaster record and accepted dos and don'ts for disaster ID: {:?}", dr_id);

    let dg_pipeline = vec![

    // Project the necessary fields without filtering do_s and dont_s by status
    doc! {
        "$project": {
            "_id": 1,                             // Include _id from disaster_guide
            "disaster_id": 1,                     // Include disaster_id from disaster_guide
            "disaster_record": 1,                 // Include full disaster_record data
            "do_s": 1,                            // Include the full do_s array (no filter)
            "dont_s": 1                           // Include the full dont_s array (no filter)
        }
    },

    // Match disaster_id to fetch only the relevant records
    doc! { "$match": { "disaster_id": dr_id } }
];



    let dg_data = match dg_collection.aggregate(dg_pipeline).await {
        Ok(mut cursor) => {
            let mut dg_data: Vec<DisasterGuide> = Vec::new();

            while let Some(doc) = cursor.try_next().await.unwrap_or(None) {
                match from_document::<DisasterGuide>(doc) {
                    Ok(disaster_guide) => dg_data.push(disaster_guide),
                    Err(e) => eprintln!("Error deserializing document: {:?}", e),
                }
            }

            if dg_data.is_empty() {
                return error_response("No matching records found", StatusCode::NOT_FOUND);
            }
            dg_data
        }
        Err(e) => {
            eprintln!("Error executing aggregation: {:?}", e);
            return error_response("Failed to fetch disaster record", StatusCode::INTERNAL_SERVER_ERROR)
        }
    };


    let filter = doc! { "_id": dr_id };  // Filter by ObjectId

    let dr_data = match dr_collection.find(filter).await {
        Ok(cursor) => {
            // Collect the documents matching the filter (in this case, it should be one or none)
            let disaster_records: Vec<DisasterRecord> = cursor.try_collect().await.unwrap_or_else(|_| vec![]);
            disaster_records
        }
        Err(e) => {
            eprintln!("Error executing aggregation: {:?}", e);
            return error_response("Failed to fetch disaster record", StatusCode::INTERNAL_SERVER_ERROR)
        }
    };

    let response_data = json!({
        "disaster_record": dr_data,
        "disaster_guide": dg_data
        
    });

    success_response(
                "Successfully fetched disaster record with accepted dos and donts",
                response_data,
                StatusCode::OK,
            )
}




pub async fn update_dos(
    State(state): State<AppState>,
    Path((dr_id, gi_id)): Path<(ObjectId, ObjectId)>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let db = state.db.lock().await;
    let dg_collection: Collection<DisasterGuide> = db.database("disaster").collection("disaster_guide");

    //println!("Updating do_s entry for disaster_id: {:?}, do_id: {:?} and body is {:?}", dr_id, gi_id, payload);

    // Extract the status directly since it's already validated in the service
    let status = payload.get("status").and_then(|s| s.as_str()).unwrap();

    //println!("Updating do_s entry for disaster_id: {:?}, do_id: {:?} with status {:?}", dr_id, gi_id, status);

    let update_result = match status {
        "Accepted" => {
            dg_collection
                .update_one(
                    doc! { "disaster_id": &dr_id, "do_s._id": &gi_id },
                    doc! { "$set": { "do_s.$.status": "Accepted" } }
                )
                .await
        }
        "Rejected" => {
            dg_collection
                .update_one(
                    doc! { "disaster_id": &dr_id },
                    doc! { "$pull": { "do_s": { "_id": &gi_id } } }  // Remove matching element
                )
                .await
        }
        _ => {
            return error_response("Invalid status provided", StatusCode::BAD_REQUEST);
        }
    };

    match update_result {
        Ok(res) => {
            if res.matched_count == 0 {
                return error_response("No matching record found", StatusCode::NOT_FOUND);
            }

            if status == "Rejected" {
                return success_response(
                    "Item removed successfully",
                    json!({
                        "item_id": gi_id,
                        "matched_count": res.matched_count
                    }),
                    StatusCode::OK,
                );
            }

            // Fetch the updated item for "Accepted" case
            let pipeline = vec![
                doc! { "$match": { "disaster_id": &dr_id } },
                doc! { "$unwind": "$do_s" },
                doc! { "$match": { "do_s._id": &gi_id } },
                doc! { "$project": { "_id": 0, "do_s": 1 } },
            ];

            let mut cursor = dg_collection.aggregate(pipeline).await.unwrap();
            if let Some(doc) = cursor.next().await {
                if let Ok(updated_doc) = doc {
                    return success_response(
                        "Status updated successfully",
                        json!({
                            "item_id": gi_id,
                            "matched_count": res.matched_count,
                            "updated_item": updated_doc.get("do_s")
                        }),
                        StatusCode::OK,
                    );
                }
            }

            success_response(
                "Status updated successfully, but no updated item found",
                json!({
                    "item_id": gi_id,
                    "matched_count": res.matched_count,
                    "updated_item": null
                }),
                StatusCode::OK,
            )
        }
        Err(e) => error_response(&format!("Update failed: {}", e), StatusCode::INTERNAL_SERVER_ERROR),
    }
}



pub async fn update_donts(
    State(state): State<AppState>,
    Path((dr_id, gi_id)): Path<(ObjectId, ObjectId)>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let db = state.db.lock().await;
    let dg_collection: Collection<DisasterGuide> = db.database("disaster").collection("disaster_guide");

    // Extract status directly, assuming it's already validated in the service
    let status = payload.get("status").and_then(|s| s.as_str()).unwrap();

   // println!("Updating dont_s entry for disaster_id: {:?}, dont_id: {:?} with status {:?}", dr_id, gi_id, status);

    let update_result = match status {
        "Accepted" => {
            dg_collection
                .update_one(
                    doc! { "disaster_id": &dr_id, "dont_s._id": &gi_id },
                    doc! { "$set": { "dont_s.$.status": "Accepted" } }
                )
                .await
        }
        "Rejected" => {
            dg_collection
                .update_one(
                    doc! { "disaster_id": &dr_id },
                    doc! { "$pull": { "dont_s": { "_id": &gi_id } } },  // Remove matching element
                )
                .await
        }
        _ => {
            return error_response("Invalid status provided", StatusCode::BAD_REQUEST);
        }
    };

    match update_result {
        Ok(res) => {
            if res.matched_count == 0 {
                return error_response("No matching record found", StatusCode::NOT_FOUND);
            }

            if status == "Rejected" {
                return success_response(
                    "Item removed successfully",
                    json!({
                        "item_id": gi_id,
                        "matched_count": res.matched_count
                    }),
                    StatusCode::OK,
                );
            }

            // Fetch the updated item for "Accepted" case
            let pipeline = vec![
                doc! { "$match": { "disaster_id": &dr_id } },
                doc! { "$unwind": "$dont_s" },
                doc! { "$match": { "dont_s._id": &gi_id } },
                doc! { "$project": { "_id": 0, "dont_s": 1 } },
            ];

            let mut cursor = dg_collection.aggregate(pipeline).await.unwrap();
            if let Some(doc) = cursor.next().await {
                if let Ok(updated_doc) = doc {
                    return success_response(
                        "Status updated successfully",
                        json!({
                            "item_id": gi_id,
                            "matched_count": res.matched_count,
                            "updated_item": updated_doc.get("dont_s")
                        }),
                        StatusCode::OK,
                    );
                }
            }

            success_response(
                "Status updated successfully, but no updated item found",
                json!({
                    "item_id": gi_id,
                    "matched_count": res.matched_count,
                    "updated_item": null
                }),
                StatusCode::OK,
            )
        }
        Err(e) => error_response(&format!("Update failed: {}", e), StatusCode::INTERNAL_SERVER_ERROR),
    }
}


// pub async fn update_guide_item(
//     State(state): State<AppState>,
//     Path((dr_id, item_id)): Path<(ObjectId, ObjectId)>,
//     Json(payload): Json<serde_json::Value>,
// ) -> impl IntoResponse {
//     let db = state.db.lock().await;
//     let dg_collection: Collection<DisasterGuide> = db.database("disaster").collection("disaster_guide");

//     // Extract the status directly since it's already validated in the service
//     let status = match payload.get("status").and_then(|s| s.as_str()) {
//         Some(s) => s,
//         None => return error_response("No status provided", StatusCode::BAD_REQUEST),
//     };

//     println!("Updating guide item for disaster_id: {:?}, item_id: {:?} with status {:?}", 
//              dr_id, item_id, status);

//     // First, find which array contains the item (do_s or dont_s)
//     let find_query = doc! {
//         "disaster_id": &dr_id,
//         "$or": [
//             { "do_s._id": &item_id },
//             { "dont_s._id": &item_id }
//         ]
//     };

//     let find_result = dg_collection.find_one(find_query).await;
    
//     let document = match find_result {
//         Ok(Some(doc)) => doc,
//         Ok(None) => return error_response("No matching disaster guide found", StatusCode::NOT_FOUND),
//         Err(e) => return error_response(&format!("Database error: {}", e), StatusCode::INTERNAL_SERVER_ERROR),
//     };

//     // Determine if the item is in do_s or dont_s array
//     let item_type = if document.do_s.iter().any(|item| item.id == item_id) {
//         "do_s"
//     } else if document.dont_s.iter().any(|item| item.id == item_id) {
//         "dont_s"
//     } else {
//         return error_response("Item not found in either do_s or dont_s arrays", StatusCode::NOT_FOUND);
//     };

//     // Now that we know which array the item belongs to, proceed with the update
//     let update_result = match status {
//         "Accepted" => {
//             dg_collection
//                 .update_one(
//                     doc! { "disaster_id": &dr_id, format!("{}._id", item_type): &item_id },
//                     doc! { "$set": { format!("{}.$.status", item_type): "Accepted" } }
//                 )
//                 .await
//         }
//         "Rejected" => {
//             dg_collection
//                 .update_one(
//                     doc! { "disaster_id": &dr_id },
//                     doc! { "$pull": { item_type: { "_id": &item_id } } }  // Remove matching element
//                 )
//                 .await
//         }
//         _ => {
//             return error_response("Invalid status provided", StatusCode::BAD_REQUEST);
//         }
//     };

//     match update_result {
//         Ok(res) => {
//             if res.matched_count == 0 {
//                 return error_response("No matching record found", StatusCode::NOT_FOUND);
//             }

//             if status == "Rejected" {
//                 return success_response(
//                     "Item removed successfully",
//                     json!({
//                         "item_id": item_id,
//                         "matched_count": res.matched_count
//                     }),
//                     StatusCode::OK,
//                 );
//             }

//             // Fetch the updated item for "Accepted" case
//             let pipeline = vec![
//                 doc! { "$match": { "disaster_id": &dr_id } },
//                 doc! { "$unwind": format!("${}", item_type) },
//                 doc! { "$match": { format!("{}._id", item_type): &item_id } },
//                 doc! { "$project": { "_id": 0, item_type: 1 } },
//             ];

//             let mut cursor = dg_collection.aggregate(pipeline).await.unwrap();
//             if let Some(doc) = cursor.next().await {
//                 if let Ok(updated_doc) = doc {
//                     return success_response(
//                         "Status updated successfully",
//                         json!({
//                             "item_id": item_id,
//                             "matched_count": res.matched_count,
//                             "updated_item": updated_doc.get(item_type)
//                         }),
//                         StatusCode::OK,
//                     );
//                 }
//             }

//             success_response(
//                 "Status updated successfully, but no updated item found",
//                 json!({
//                     "item_id": item_id,
//                     "matched_count": res.matched_count,
//                     "updated_item": null
//                 }),
//                 StatusCode::OK,
//             )
//         }
//         Err(e) => error_response(&format!("Update failed: {}", e), StatusCode::INTERNAL_SERVER_ERROR),
//     }
// }