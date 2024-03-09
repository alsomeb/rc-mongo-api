use std::env;
use dotenv::dotenv;
use firebase_auth::FirebaseAuth;

use crate::repository::mongo_repo::MongoRepo;

pub struct AppData {
    pub db: MongoRepo,
    pub firebase_auth: FirebaseAuth
}

pub type AsyncError = Box<dyn std::error::Error + Send + Sync>; // Send + Sync För att det är async runtime

impl AppData {
    /// Initializes `AppData` with database and FirebaseAuth configurations.
    /// Returns `AppData` on success or a thread-safe error on failure, compatible with async environments.
    /// The error is boxed to allow for multiple error types to be returned
    pub async fn new() -> Result<Self, AsyncError> {
        dotenv().ok(); // Load .env file if it exists

        // Initialize MongoDB connection
        let db = MongoRepo::init().await;

        // Retrieve Firebase ID from environment variable
        let firebase_id = env::var("FIREBASE_ID").expect("FIREBASE_ID environment variable not set");

        // Initialize FirebaseAuth with the Firebase ID
        let firebase_auth = FirebaseAuth::new(&firebase_id).await;

        Ok(Self { db, firebase_auth })
    }
}
