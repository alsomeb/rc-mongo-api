# rc-mongo-api

`rc-mongo-api`, Recipe Companion, is a simple CRUD (Create, Read, Update, Delete) API for managing recipes, built using Rust with asynchronous support. The API uses MongoDB for data storage and Firebase for authentication.
This API is one of the services part of my exam work for Nackademin.

## Features

- Create new recipes
- Update existing recipes
- Delete recipes
- Retrieve recipes by ID or user email
- Update recipe image URLs
- Paginated retrieval of all recipes


## Dependencies

The project relies on several Rust crates to function:

- `actix-web`: Web framework for building HTTP servers
- `serde`: Serialization and deserialization library
- `serde_json`: JSON support for `serde`
- `dotenv`: Library to load environment variables from a `.env` file
- `futures`: Asynchronous programming support
- `env_logger`: Logging support
- `log`: Logging facade
- `firebase-auth`: Firebase authentication integration
- `actix-cors`: Cross-Origin Resource Sharing (CORS) support
- `http`: HTTP library
- `mongodb`: MongoDB driver for Rust


## Running the Project

- Create a `.env` file with your MongoDB and Firebase Credentials
- `cargo run` to run the project
