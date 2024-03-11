use actix_web::{get, HttpResponse, post};
use actix_web::web::{Data, Json};
use firebase_auth::FirebaseUser;

use crate::api::util::{Response, unauthorized_response};
use crate::models::recipe_model::Recipe;
use crate::repository::mongo_repo::MongoRepo;

#[post("/recipes")]
pub async fn insert_recipe(db: Data<MongoRepo>, new_recipe: Json<Recipe>, firebase_user: Result<FirebaseUser, actix_web::Error>) -> HttpResponse {
    // Util function checking if we have a valid token in Auth Header
    if let Err(_) = firebase_user {
        return unauthorized_response();
    }

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

    match db.insert_recipe(data).await {
        Ok(recipe_id) => HttpResponse::Created().json(Response { message: format!("Recipe added with ID: {}", recipe_id) }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()), // Bara om de blir Error i Servern
    }
}


#[get("/recipes/user")]
pub async fn get_recipes_by_email(db: Data<MongoRepo>, firebase_user: Result<FirebaseUser, actix_web::Error>) -> HttpResponse {

    // Check if user is authenticated, return unauthorized response if not
    if let Err(_) = firebase_user {
        return unauthorized_response();
    }

    // Authentication succeeded, extract the email from the FirebaseUser
    let user = firebase_user.unwrap(); // Safe due to the check above
    let email = user.email.unwrap_or("empty email".to_string());

    match db.get_recipes_by_email(email.as_str()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

