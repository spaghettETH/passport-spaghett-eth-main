// Importing dependencies
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::FindOptions;
use mongodb::Database;

// Import Checkpoints structs
use crate::modules::passport::model::UserDocument;

pub async fn return_all_passports(
    db: &Database
) -> mongodb::error::Result<Vec<UserDocument>> {
    let collection = db.collection::<Document>("users");
    let find_options = FindOptions::builder().build();
    let mut cursor = collection.find(doc! {}, find_options).await?;
    let mut users: Vec<UserDocument> = vec![];
    while let Some(result) = cursor.try_next().await? {
        let user_document = UserDocument {
            name: result.get_str("name").unwrap().to_string(),
            description: result.get_str("description").unwrap().to_string(),
            image: result.get_str("image").unwrap().to_string(),
            updated_at: result.get_i64("updated_at").unwrap(),
            address: result.get_str("address").unwrap().to_string(),
        };
        users.push(user_document);
    }

    Ok(users)
}
