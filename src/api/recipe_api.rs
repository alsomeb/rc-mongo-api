use actix_web::{get, HttpResponse, post};
use actix_web::web::{Data, Json};
use firebase_auth::FirebaseUser;
use serde::{Deserialize, Serialize};

use crate::models::recipe_model::Recipe;
use crate::repository::mongo_repo::MongoRepo;

#[derive(Serialize, Deserialize)]
pub(crate) struct Response {
    pub(crate) message: String,
}

#[post("/recipes")]
pub async fn insert_recipe(db: Data<MongoRepo>, new_recipe: Json<Recipe>, firebase_user: Result<FirebaseUser, actix_web::Error>) -> HttpResponse {
    match firebase_user {
        // vi behöver ej user object, bara kollar om token verified
        Ok(_) => {
            // Take ownership of the inner `Recipe` to avoid cloning
            let new_recipe = new_recipe.into_inner();

            let data = Recipe {
                id: None, // Mongo Genererar
                title: new_recipe.title,
                description: new_recipe.description,
                steps: new_recipe.steps,
                ingredients: new_recipe.ingredients,
                email: new_recipe.email,
            };

            let recipe_result = db.insert_recipe(data).await;

            match recipe_result {
                Ok(recipe_id) => HttpResponse::Created().json(Response { message: format!("Recipe added with ID: {}", recipe_id) }),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()), // Bara om de blir Error i Servern
            }
        }
        Err(_) => {
            // Authentication failed, return 403 Forbidden.
            HttpResponse::Forbidden().json(Response { message: "Missing JWT Token".to_string() })
        }
    }

}

#[get("/recipes/user")]
pub async fn get_recipes_by_email(db: Data<MongoRepo>, firebase_user: Result<FirebaseUser, actix_web::Error>) -> HttpResponse {
    match firebase_user {
        Ok(user) => {
            // Authentication succeeded, proceed with retrieving users.
            let email = user.email.unwrap_or("empty email".to_string());
            let users_result = db.get_recipes_by_email(email.as_str()).await;
            match users_result {
                Ok(users) => HttpResponse::Ok().json(users),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(_) => {
            // Authentication failed, return 403 Forbidden.
            HttpResponse::Forbidden().json(Response { message: "Missing JWT Token".to_string() })
        }
    }
}

