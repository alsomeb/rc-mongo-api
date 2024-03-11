use std::env;
use std::io::{Error, ErrorKind};

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;

use crate::api::health_check::health_check;
use crate::api::recipe_api::{get_recipes_by_email, insert_recipe};
use crate::models::app_data::AppData;

mod models;
mod repository;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // So we can access db + firebase auth throughout the app in a shared state
    // We need to convert it to an io::ErrorKind since the main function's error type is this type
    let app_data = AppData::new().await.map_err(|err| Error::new(ErrorKind::Other, err))?;
    let db = Data::new(app_data.db);
    let firebase_auth = Data::new(app_data.firebase_auth);

    env::set_var("RUST_LOG", "actix_web=info"); // Configure the logging level
    env_logger::init(); // Initialize the logger

    // The move keyword attached to the closure gives it, HttpServer, ownership of the MongoDB configuration.
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%r %U %a - %s")) // Add the Logger middleware
            .app_data(db.clone())
            .app_data(firebase_auth.clone())
            .service(insert_recipe)
            .service(get_recipes_by_email)
            .service(health_check)
    })
        .bind(("127.0.0.1", 8080))?
        //.bind(("0.0.0.0", 8080))? for docker network
        .run()
        .await
}
