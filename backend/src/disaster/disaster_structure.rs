use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DisasterRecord {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[validate(length(min = 2, message = "Name must be at least 2 characters long"))]
    pub name: String,

    #[validate(length(min = 5, message = "Effects must be at least 5 characters long"))]
    pub effects: String,

    #[validate(length(min = 10, message = "Short description must be at least 10 characters long"))]
    pub short_description: String,

    #[serde(default)]
    #[validate(url(message = "Invalid YouTube link format"))]
    pub youtube_link: String,
}


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DisasterGuide {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,

    #[serde(rename = "disaster_id")]
    pub disaster_id: ObjectId, // Reference to a DisasterRecord

    #[validate(length(min = 1, message = "At least one 'do' should be provided"))]
    pub dos: Vec<GuideItem>,

    #[validate(length(min = 1, message = "At least one 'don't' should be provided"))]
    pub donts: Vec<GuideItem>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GuideItem {
    pub user_id: ObjectId, // User who added this guide entry

    #[validate(length(min = 2, message = "Status must be at least 2 characters long"))]
    pub status: String, // Example: "Accepted", "Pending". Delete entry if rejected

    #[validate(length(min = 5, message = "Message must be at least 5 characters long"))]
    pub message: String, // The actual guidance message
}
