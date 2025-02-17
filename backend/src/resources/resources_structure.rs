use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub quantity: u32,
    pub category: String,
    pub description: String,
    pub location: Location,
    pub status: ResourceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceStatus {
    Available,
    Reserved
}
