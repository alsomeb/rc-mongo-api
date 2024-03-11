use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

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
