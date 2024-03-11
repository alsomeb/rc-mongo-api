extern crate dotenv;

use std::env;

use futures::TryStreamExt;
use mongodb::{Client, Collection, Database};
use mongodb::bson::doc;
use mongodb::error::Error;

use crate::models::recipe_model::Recipe;

// https://dev.to/hackmamba/create-a-graphql-powered-project-management-endpoint-in-rust-and-mongodb-actix-web-version-3j1
// Impl multiple Collections for MongoDB

pub struct MongoRepo {
    db: Database,
}

pub enum CollectionName {
    Recipes,
}

impl MongoRepo {
    pub async fn init() -> Self {
        let uri = env::var("MONGO_URI").expect("MONGO_URI environment variable not set");
        let client = Client::with_uri_str(uri).await.expect("Failed to connect to MongoDB with provided URI");
        let db = client.database("alsomeb");
        MongoRepo {db}
    }

    pub async fn collection_switch<T>(data_source: &Self, col_name: CollectionName) -> Collection<T> {
        match col_name {
            CollectionName::Recipes => data_source.db.collection("Recipes"),
        }
    }


    pub async fn insert_recipe(&self, new_recipe: Recipe) -> Result<String, Error> {
        let col = MongoRepo::collection_switch::<Recipe>(&self, CollectionName::Recipes).await;

        let recipe_result = col
            .insert_one(new_recipe, None)
            .await?;

        Ok(recipe_result.inserted_id.to_string())
    }

    pub async fn get_recipes_by_email(&self, email: &str) -> Result<Vec<Recipe>, Error> {
        let col = MongoRepo::collection_switch::<Recipe>(&self, CollectionName::Recipes).await;

        let mut cursors = col
            .find(doc! {"email": email}, None)
            .await?;

        let mut recipes: Vec<Recipe> = Vec::new();

        // This is a loop that will continue to run as long as the pattern matching succeeds.
        // In this case, the pattern is Some(recipe), which matches the Option type returned by the try_next() method call.

        // Here, Some(recipe) means that if the try_next() method returns a Some variant (indicating that there is a next item in the stream),
        // then the recipe variable inside the Some will be bound to that item.

        // try_next(): This method is called on cursors, which is an asynchronous stream of documents retrieved from a MongoDB collection.
        // The try_next() method attempts to fetch the next item from the stream.
        // It returns a Result<Option<Recipe>, Error>, where Ok(Some(recipe)) indicates a successfully retrieved recipe, Ok(None) indicates the end of the stream (no more items)

        // The loop body ({ recipes.push(recipe) }): For each recipe successfully matched by Some(recipe),
        // the loop body executes. In this case, it adds the recipe to the recipes vector using the push method.
        while let Some(recipe) = cursors
            .try_next()
            .await?
        {
            recipes.push(recipe)
        }

        Ok(recipes)
    }
/*
    pub async fn update_user_by_id(&self, id: &String, new_user: User) -> Option<User> {
        let col = MongoRepo::collection_switch::<User>(&self, CollectionName::User).await;
        let obj_id = ObjectId::parse_str(id).unwrap_or_default();
        let filter = doc! {"_id": obj_id};

        let updated_doc = col
            .find_one_and_replace(
                filter,
                new_user,
                FindOneAndReplaceOptions::builder() // Vi vill ha Dokumentet EFTER med nya uppdateringen, använder "FindOneAndReplaceOptions::Builder()"
                    .return_document(ReturnDocument::After)
                    .build())
            .await
            .ok()?; // ok() method to convert from Result<T, E> to Option<T>, which is a valid approach when you want to discard the error and work with an Option


        updated_doc
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let col = MongoRepo::collection_switch::<User>(&self, CollectionName::User).await;

        let mut cursors = col
            .find(None, None) // without any options & filters to match all documents
            .await?;

        let mut users: Vec<User> = Vec::new();

        while let Some(user) = cursors
            .try_next()
            .await?
        {
            users.push(user)
        }

        Ok(users)
    }

    // Denna är förbättrad och kommer ej PANIC vid error, samt Return Option<User> istället
    pub async fn get_user_by_id(&self, id: &String) -> Option<User> {
        let col = MongoRepo::collection_switch::<User>(&self, CollectionName::User).await;
        let obj_id = ObjectId::parse_str(id).unwrap_or_default();
        let filter = doc!{"_id": obj_id};

        let result = col
            .find_one(filter, None)
            .await;

        /*
             In this code, ok() is used to convert Result<Option<User>> to Option<Option<User>>, and then and_then is used to flatten it. Finally, the inner Option<User> is extracted
        */
        let user_option = result.ok().and_then(|user_result| user_result); // Some languages call this operation flatmap
        user_option
    }

    pub async fn delete_user_by_id(&self, id: &str) -> Option<User> {
        let col = MongoRepo::collection_switch::<User>(&self, CollectionName::User).await;
        let obj_id = ObjectId::parse_str(id).unwrap_or_default();
        let filter = doc!{"_id": obj_id};

        let result = col
            .find_one_and_delete(filter, None)
            .await
            .ok()?; // ok() method to convert from Result<T, E> to Option<T>, which is a valid approach when you want to discard the error and work with an Option

        result
    }

 */

}
