use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;


#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String,
    pub steps: Vec<String>,
    pub ingredients: Vec<String>,
    pub firebase_uid: String,
    pub email: String,
}
