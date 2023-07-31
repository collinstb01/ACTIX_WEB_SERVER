use mongodb::bson::oid::ObjectId; // import ohject id
use serde::{Deserialize, Serialize}; // serde library

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub message: String,
    pub owner_id: ObjectId,
}
