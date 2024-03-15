use actix_web::{delete, get, HttpResponse, post, put};
use actix_web::web::{Data, Json, Path};
use firebase_auth::FirebaseUser;
use mongodb::bson::oid::ObjectId;

use crate::api::util::{Response, unauthorized_response};
use crate::models::recipe_model::Recipe;
use crate::repository::mongo_repo::MongoRepo;

/*
    The Result<FirebaseUser, actix_web::Error> type in our handler function's parameters is a pattern in Actix-web that allows your handler to work with extractors that might fail.
    In this context, FirebaseUser is an extractor provided by the firebase_auth crate that attempts to extract and validate a Firebase user from the request,
    typically by looking at the Authorization header for a Firebase JWT token.

    Using Result<FirebaseUser, actix_web::Error> in our handler allows us to explicitly handle authentication failures.
    This is useful for customizing the response in case of errors, such as providing a specific error message or status code, as we've done with the unauthorized_response() function.

    However it's not strictly necessary to use Result<FirebaseUser, actix_web::Error> if our application's logic does not require custom error handling for authentication failures
    but we want to provide 403 response which is not guaranteed without this implementation
 */

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

#[put("/recipes/{id}")]
pub async fn update_recipe_by_id(db: Data<MongoRepo>, id: Path<String>, new_recipe: Json<Recipe>, firebase_user: Result<FirebaseUser, actix_web::Error>) -> HttpResponse {
    if let Err(_) = firebase_user {
        return unauthorized_response();
    }

    // Shadowing variable, overwriting
    let id = id.into_inner();
    let object_id = ObjectId::parse_str(&id).ok();

    // Take ownership of the inner `Recipe` to avoid cloning
    let new_recipe = new_recipe.into_inner();
    let data = Recipe {
        id: object_id,
        title: new_recipe.title,
        description: new_recipe.description,
        steps: new_recipe.steps,
        ingredients: new_recipe.ingredients,
        email: new_recipe.email,
    };

    match db.update_recipe_by_id(id.as_str(), data).await {
        Some(recipe) => HttpResponse::Ok().json(recipe),
        None => HttpResponse::BadRequest().json(Response {message: "No ID Match".to_string()}),
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

#[delete("/recipes/{id}")]
pub async fn delete_recipe_by_id(db: Data<MongoRepo>, id: Path<String>, firebase_user: Result<FirebaseUser, actix_web::Error>) -> HttpResponse {
    if let Err(_) = firebase_user {
        return unauthorized_response();
    }

    let id = id.into_inner();

    match db.delete_recipe_by_id(id.as_str()).await {
        Some(_) => HttpResponse::Ok().json(Response { message: format!("Recipe with ID: {} deleted", id)}),
        None => HttpResponse::BadRequest().json(Response { message: format!("No recipe with ID: {} found", id) })
    }
}

#[get("/recipes/{id}")]
pub async fn get_recipe_by_id(db: Data<MongoRepo>, id: Path<String>, firebase_user: Result<FirebaseUser, actix_web::Error>) -> HttpResponse {
    if let Err(_) = firebase_user {
        return unauthorized_response();
    }

    let id = id.into_inner();

    match db.get_recipe_by_id(id.as_str()).await {
        Some(recipe) => HttpResponse::Ok().json(recipe),
        None => HttpResponse::BadRequest().json(Response { message: format!("No recipe with ID: {} found", id) })
    }
}



