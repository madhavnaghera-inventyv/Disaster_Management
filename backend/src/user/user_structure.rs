use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,

    #[validate(length(min = 2, message = "Name must be at least 2 characters long"))]
    pub name: String,

    #[serde(default)]
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[serde(default)]
    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password: String,
}
