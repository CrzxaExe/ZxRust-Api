use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

// Database models

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryItem {
    pub _id: ObjectId,
    pub name: String,
    pub amount: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub username: String,
    pub password: String,
}