use actix_web::{delete, get, HttpResponse, patch, post, put};
use actix_web::web::{Data, Json, Path, Query};
use firebase_auth::FirebaseUser;
use mongodb::bson::oid::ObjectId;

use crate::api::util::{map_input_dto, PaginationParams, RecipeStatus, Response, unauthorized_response};
use crate::models::recipe_model::{ImageUrlChangeRequest, RecipeDTO};
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
pub async fn insert_recipe(db: Data<MongoRepo>, new_recipe: Json<RecipeDTO>, firebase_user: Result<FirebaseUser, actix_web::Error>) -> HttpResponse {
    // Util function checking if we have a valid token in Auth Header
    if let Err(_) = firebase_user {
        return unauthorized_response();
    }

    // Take ownership of the inner `Recipe` to avoid cloning
    let new_recipe_dto = new_recipe.into_inner();
    let recipe_entity = map_input_dto(new_recipe_dto, None, RecipeStatus::Created);

    match db.insert_recipe(recipe_entity).await {
        Ok(recipe_id) => HttpResponse::Created().json(Response { message: format!("Recipe added with ID: {}", recipe_id) }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()), // Bara om de blir Error i Servern
    }
}

#[put("/recipes/{id}")]
pub async fn update_recipe_by_id(db: Data<MongoRepo>, id: Path<String>, new_recipe: Json<RecipeDTO>, firebase_user: Result<FirebaseUser, actix_web::Error>) -> HttpResponse {
    if let Err(_) = firebase_user {
        return unauthorized_response();
    }

    // Shadowing variable, overwriting
    let id = id.into_inner();
    let object_id = ObjectId::parse_str(&id).ok();

    // Take ownership of the inner `Recipe` to avoid cloning
    let new_recipe_dto = new_recipe.into_inner();
    let recipe_entity = map_input_dto(new_recipe_dto, object_id, RecipeStatus::Updated);

    match db.update_recipe_by_id(id.as_str(), recipe_entity).await {
        Some(recipe) => HttpResponse::Ok().json(recipe),
        None => HttpResponse::BadRequest().json(Response {message: "No ID Match".to_string()}),
    }
}

#[patch("/recipes/{id}/imgurl")]
pub async fn update_image_url_by_recipe_id(db: Data<MongoRepo>, id: Path<String>, image_url: Json<ImageUrlChangeRequest>, firebase_user: Result<FirebaseUser, actix_web::Error>) -> HttpResponse {
    if let Err(_) = firebase_user {
        return unauthorized_response();
    }

    let id = id.into_inner();
    let new_url = image_url.photo_url.to_owned();

    match db.update_recipe_img_url(id.as_str(), new_url.as_str()).await {
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
        Ok(recipes) => HttpResponse::Ok().json(recipes),
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

#[get("/recipes/{id}/imgurl")]
pub async fn get_recipe_img_url_by_id(db: Data<MongoRepo>, id: Path<String>, firebase_user: Result<FirebaseUser, actix_web::Error>) -> HttpResponse {
    if let Err(_) = firebase_user {
        return unauthorized_response();
    }

    let id = id.into_inner();

    match db.get_recipe_img_url_by_id(id.as_str()).await {
        Some(img_url) => HttpResponse::Ok().body(img_url),
        None => HttpResponse::BadRequest().json(Response { message: format!("No recipe with ID: {} found", id) })
    }
}

// This setup allows the /recipes endpoint to accept page and per_page query parameters for
// ex ../recipes?page=1&per_page=20 -> Ger Page 1 och 20 Resultat
#[get("/recipes")]
pub async fn get_all_recipes_pagination(db: Data<MongoRepo>, firebase_user: Result<FirebaseUser, actix_web::Error>, params: Query<PaginationParams>) -> HttpResponse {
    if let Err(_) = firebase_user {
        return unauthorized_response();
    }

    // Details of pagination & Defaults
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(5);

    match db.get_all_recipes_pageable(page, per_page).await {
        Ok(recipes) => HttpResponse::Ok().json(recipes),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}



