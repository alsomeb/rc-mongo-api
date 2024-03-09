use actix_web::{get, HttpResponse};
use crate::api::recipe_api::Response;

#[get("/health")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(Response {message: "UP".to_string()})
}
