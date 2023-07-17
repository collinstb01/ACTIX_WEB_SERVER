use mongodb::bson::oid::ObjectId; // import ohject id
use serde::{Deserialize, Serialize}; // serde library

#[derive(Debug, Serialize, Deserialize)]
struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub location: String,
    pub title: String,
}
