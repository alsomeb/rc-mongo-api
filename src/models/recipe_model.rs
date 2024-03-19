use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String,
    pub steps: Vec<String>,
    pub ingredients: Vec<String>,
    pub email: String,
    pub created: Option<mongodb::bson::DateTime>, // Då vi inte vill create alltid
    pub updated: mongodb::bson::DateTime
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeDTO {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String,
    pub steps: Vec<String>,
    pub ingredients: Vec<String>,
    pub email: String,
    // Created & Updated will be done in the code not from request
}
