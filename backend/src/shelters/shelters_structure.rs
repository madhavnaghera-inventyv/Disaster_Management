use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shelter {
    pub name: String,
    pub capacity: u32,
    pub available_beds: u32,
    pub street: String,
    pub district: String,
    pub state: String,
    pub country: String,
}
