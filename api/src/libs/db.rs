use mongodb::options::ClientOptions;
use mongodb::{Client, Database};
use rocket::fairing::AdHoc;
use std::env;

pub fn init() -> AdHoc {
    AdHoc::on_ignite("💽 Connecting to MongoDB", |rocket| async {
        match connect().await {
            Ok(database) => {
                println!("🔥 MongoDB Connected!");
                rocket.manage(database)
            }
            Err(error) => {
                panic!("Cannot connect to instance:: {:?}", error)
            }
        }
    })
}

pub async fn connect() -> mongodb::error::Result<Database> {
    let mongo_uri = env::var("MONGODB_CONNECTION").expect("MONGODB_CONNECTION is not found.");
    let mongo_db_name = env::var("MONGODB_DB").expect("MONGODB_DB is not found.");

    let client_options = ClientOptions::parse(mongo_uri).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database(mongo_db_name.as_str());

    Ok(database)
}
