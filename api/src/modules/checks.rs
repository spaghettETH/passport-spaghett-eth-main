use crate::libs::responses::ApiSuccess;
use dotenv::dotenv;
use mongodb::bson::doc;
use mongodb::{options::ClientOptions, Client};
use rocket::response::Debug;
use rocket::serde::json::Json;
use rocket::tokio::task;
use std::env;

#[get("/checks/version")]
pub fn version() -> Json<ApiSuccess> {
    dotenv().ok();
    Json(ApiSuccess {
        message: "API ".to_owned()
            + &env::var("API_VERSION").unwrap().to_owned().to_string(),
        details: "".to_string(),
        error: false,
    })
}

#[get("/checks/database")]
pub async fn status() -> Result<Json<ApiSuccess>, Debug<task::JoinError>> {
    dotenv().ok();
    let mongodb_connection =
        std::env::var("MONGODB_CONNECTION").expect("failed to find mongodb connection");
    let database_name = std::env::var("MONGODB_DB").expect("failed to find mongodb database");
    let mut client_options = ClientOptions::parse(mongodb_connection)
        .await
        .expect("Can't create client options");
    client_options.app_name = Some("Control Room".to_string());
    let client = Client::with_options(client_options).expect("Can't create client");
    let databases = client
        .list_database_names(None, None)
        .await
        .expect("Can't list databases");
    let mut found = false;
    databases.iter().for_each(|database| {
        if database == &database_name {
            found = true;
        }
    });
    if found {
        Ok(Json(ApiSuccess {
            message: "DATABASE OK".to_string(),
            details: "".to_string(),
            error: false,
        }))
    } else {
        Ok(Json(ApiSuccess {
            message: "DATABASE NOT OK".to_string(),
            details: "".to_string(),
            error: true,
        }))
    }
}
