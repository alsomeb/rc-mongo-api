use actix_web::{get, Responder};
use actix_web::web::Json;

use crate::api::recipe_api::Response;

/*
    Handler functions in Actix can return a wide range of objects that implement the Responder trait. This makes it a breeze to return consistent responses from your APIs.
    Json<T> where T: Serialize (From Serde)
 */
#[get("/health")]
pub async fn health_check() -> impl Responder {
    Json(
        Response { message: "Server is UP".to_owned() }
    )
}
