use actix_web::HttpResponse;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::models::recipe_model::{Recipe, RecipeDTO};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub message: String,
}

/*
    In this setup, both insert_recipe and get_recipes_by_email handlers begin by checking the result of the firebase_user extractor.
    If the result is an error (indicating authentication failure),
    the unauthorized_response function is called to generate a standardized HttpResponse indicating the request was unauthorized.
    If the authentication is successful, the handlers proceed with their respective logic.

 */

/// Utility function to create a uniform unauthorized response
pub fn unauthorized_response() -> HttpResponse {
    log::warn!("Unauthorized access attempt detected. Responding with 'Missing or invalid JWT Token'.");
    HttpResponse::Forbidden().json(Response {
        message: "Missing or invalid JWT Token".to_string(),
    })
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}


#[derive(Debug)]
pub enum RecipeStatus {
    Created,
    Updated
}

/// input_recipe: The recipe to be mapped,
/// id: Option<ObjectId> (for update recipe pass ObjectId)
/// status: Created or Updated recipe
pub fn map_input_dto(input_recipe_dto: RecipeDTO, id: Option<ObjectId>, status: RecipeStatus) -> Recipe {
    let bson_date = mongodb::bson::DateTime::now();

    match status {
        RecipeStatus::Created => {
            Recipe {
                id, // Om None Mongo Genererar
                title: input_recipe_dto.title,
                photo_url: input_recipe_dto.photo_url,
                description: input_recipe_dto.description,
                steps: input_recipe_dto.steps,
                ingredients: input_recipe_dto.ingredients,
                email: input_recipe_dto.email,
                tags: input_recipe_dto.tags,
                created: Some(bson_date),
                updated: bson_date
            }
        }
        RecipeStatus::Updated => {
            Recipe {
                id, // Om None Mongo Genererar
                title: input_recipe_dto.title,
                photo_url: input_recipe_dto.photo_url,
                description: input_recipe_dto.description,
                steps: input_recipe_dto.steps,
                ingredients: input_recipe_dto.ingredients,
                email: input_recipe_dto.email,
                tags: input_recipe_dto.tags,
                created: None,
                updated: bson_date
            }
        }
    }
}
