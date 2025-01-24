use std::collections::HashMap;

// Importing dependencies
use crate::modules::checkpoints::model::CheckpointJson;
use futures::stream::TryStreamExt;
use futures::StreamExt;
use mongodb::bson::{doc, DateTime, Document};
use mongodb::options::FindOptions;
use mongodb::Database;

// Import Checkpoints structs
use crate::modules::checkpoints::model::{Checkpoint, CheckpointDocument};

pub async fn insert_sign(
    db: &Database,
    receiver: String,
    id: String,
    txid: String,
    signature: String,
) -> mongodb::error::Result<String> {
    let collection = db.collection::<Document>("signs");
    let created_at: DateTime = DateTime::now();
    let check = collection
        .find_one(doc! {"checkpoint_id": &id, "receiver": &receiver }, None)
        .await?;
    if check.is_none() {
        let insert_one_result = collection
            .insert_one(
                doc! {
                    "checkpoint_id": id.clone(),
                    "signature": signature.to_string(),
                    "receiver": receiver,
                    "txid": txid,
                    "created_at": created_at.to_string()
                },
                None,
            )
            .await?;

        Ok(insert_one_result.inserted_id.to_string())
    } else {
        Ok("ALREADY_SIGNED".to_string())
    }
}

pub async fn update_txid(
    db: &Database,
    checkpoint_id: String,
    receiver: String,
    txid: String,
) -> mongodb::error::Result<String> {
    let collection = db.collection::<CheckpointDocument>("signs");
    collection
        .update_one(
            doc! {"checkpoint_id": checkpoint_id, "receiver": receiver},
            doc! {"$set": {"txid": txid}},
            None,
        )
        .await?;
    Ok("OK".to_string())
}

pub async fn find_collected_checkpoints(
    db: &Database,
    address: String,
) -> mongodb::error::Result<Vec<Checkpoint>> {
    let collection = db.collection::<CheckpointDocument>("signs");
    let find_options = FindOptions::builder().build();
    let mut cursor = collection
        .find(doc! { "receiver": address.to_string() }, find_options)
        .await?;
    let mut signs: Vec<Checkpoint> = vec![];
    while let Some(result) = cursor.try_next().await? {
        let metadata = return_single_checkpoint(db, result.checkpoint_id.clone()).await;
        if metadata.is_err() {
            continue;
        }
        let checkpoint_json = Checkpoint {
            checkpoint_id: result.checkpoint_id,
            signature: result.signature,
            receiver: result.receiver,
            metadata: metadata.unwrap(),
            txid: result.txid,
        };
        signs.push(checkpoint_json);
    }

    Ok(signs)
}

pub async fn balance_of_id(
    db: &Database,
    address: String,
    id: String,
) -> mongodb::error::Result<i32> {
    let collection = db.collection::<CheckpointDocument>("signs");
    let find_options = FindOptions::builder().build();
    let cursor = collection
        .find(
            doc! { "receiver": address.to_string(), "name": id },
            find_options,
        )
        .await?;
    let count = cursor.count().await;
    Ok(count as i32)
}

pub async fn checkpoint_exists(db: &Database, name: String) -> mongodb::error::Result<bool> {
    let collection = db.collection::<Document>("metadata");
    let check = collection
        .find_one(doc! {"checkpoint_slug": name }, None)
        .await?;
    if check.is_none() {
        Ok(false)
    } else {
        Ok(true)
    }
}

pub async fn insert_new_checkpoint(
    db: &Database,
    name: String,
    description: String,
    image: String,
    slug: String,
    checkpoint_id: String,
    timestamp_start: String,
    timestamp_end: String,
) -> mongodb::error::Result<String> {
    let collection = db.collection::<Document>("metadata");
    let hashed = ethers::utils::hash_message(&slug.as_bytes());
    let check = collection
        .find_one(doc! {"checkpoint_slug": &slug }, None)
        .await?;
    if check.is_none() {
        let insert_one_result = collection
            .insert_one(
                doc! {
                    "name": name.clone(),
                    "description": description.clone(),
                    "checkpoint_id": checkpoint_id,
                    "timestamp_start": timestamp_start,
                    "timestamp_end": timestamp_end,
                    "checkpoint_slug": &slug,
                    "image": image.clone(),
                    "hash": format!("{:#x}", hashed)
                },
                None,
            )
            .await?;
        Ok(insert_one_result.inserted_id.to_string())
    } else {
        Ok("ERROR".to_string())
    }
}

pub async fn edit_existing_checkpoint(
    db: &Database,
    name: String,
    description: String,
    image: String,
    slug: String,
    checkpoint_id: String,
    timestamp_start: String,
    timestamp_end: String,
) -> mongodb::error::Result<String> {
    let collection = db.collection::<Document>("metadata");
    let hashed = ethers::utils::hash_message(&slug.as_bytes());
    let filter = doc! {"checkpoint_slug": &slug};
    let update = doc! {
        "$set": {
            "name": name.clone(),
            "description": description.clone(),
            "checkpoint_id": checkpoint_id,
            "timestamp_start": timestamp_start,
            "timestamp_end": timestamp_end,
            "image": image.clone(),
            "hash": format!("{:#x}", hashed)
        }
    };
    let options = mongodb::options::UpdateOptions::builder()
        .upsert(false)
        .build();

    let result = collection.update_one(filter, update, options).await?;

    if result.modified_count == 1 {
        Ok("Updated successfully".to_string())
    } else {
        Ok("ERROR".to_string())
    }
}

pub async fn return_all_checkpoints_for_event(
    db: &Database,
    name: &str,
) -> mongodb::error::Result<Vec<CheckpointJson>> {
    let collection = db.collection::<Document>("metadata");
    let find_options = FindOptions::builder().build();
    let mut cursor = collection.find(doc! {"name": name}, find_options).await?;
    let mut checkpoints: Vec<CheckpointJson> = vec![];
    while let Some(result) = cursor.try_next().await? {
        let checkpoint_json = CheckpointJson {
            name: result.get_str("name").unwrap().to_string(),
            description: result.get_str("description").unwrap().to_string(),
            image: result.get_str("image").unwrap().to_string(),
            timestamp_start: result.get_str("timestamp_start").unwrap().to_string(),
            timestamp_end: result.get_str("timestamp_end").unwrap().to_string(),
            checkpoint_slug: result.get_str("checkpoint_slug").unwrap().to_string(),
            checkpoint_id: result.get_str("checkpoint_id").unwrap().to_string(),
            hash: result.get_str("hash").unwrap().to_string(),
        };
        checkpoints.push(checkpoint_json);
    }

    Ok(checkpoints)
}

pub async fn return_checkpoint_for_slug(
    db: &Database,
    slug: &str,
) -> mongodb::error::Result<CheckpointJson> {
    let collection = db.collection::<Document>("metadata");
    let find_options = FindOptions::builder().build();
    let mut cursor = collection
        .find(doc! {"checkpoint_slug": slug}, find_options)
        .await?;
    let result = cursor.next().await.unwrap()?;
    let checkpoint_json = CheckpointJson {
        name: result.get_str("name").unwrap().to_string(),
        description: result.get_str("description").unwrap().to_string(),
        image: result.get_str("image").unwrap().to_string(),
        timestamp_start: result.get_str("timestamp_start").unwrap().to_string(),
        timestamp_end: result.get_str("timestamp_end").unwrap().to_string(),
        checkpoint_slug: result.get_str("checkpoint_slug").unwrap().to_string(),
        checkpoint_id: result.get_str("checkpoint_id").unwrap().to_string(),
        hash: result.get_str("hash").unwrap().to_string(),
    };

    Ok(checkpoint_json)
}

pub async fn return_single_checkpoint(
    db: &Database,
    id: String,
) -> mongodb::error::Result<CheckpointJson> {
    let collection = db.collection::<Document>("metadata");
    let check = collection
        .find_one(doc! {"checkpoint_id": id }, None)
        .await?;
    if !check.is_none() {
        let result = check.unwrap();
        let metadata_json = CheckpointJson {
            name: result.get_str("name").unwrap().to_string(),
            description: result.get_str("description").unwrap().to_string(),
            image: result.get_str("image").unwrap().to_string(),
            timestamp_start: result.get_str("timestamp_start").unwrap().to_string(),
            timestamp_end: result.get_str("timestamp_end").unwrap().to_string(),
            checkpoint_id: result.get_str("checkpoint_id").unwrap().to_string(),
            checkpoint_slug: result.get_str("checkpoint_slug").unwrap().to_string(),
            hash: result.get_str("hash").unwrap().to_string(),
        };
        Ok(metadata_json)
    } else {
        Err(mongodb::error::Error::from(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Not found",
        )))
    }
}

pub async fn return_leaderboard(db: &Database) -> mongodb::error::Result<HashMap<String, u128>> {
    let collection = db.collection::<CheckpointDocument>("signs");
    let find_options = FindOptions::builder().build();
    let mut cursor = collection.find(doc! {}, find_options).await?;
    let mut leaderboard: HashMap<String, u128> = HashMap::new();

    while let Some(result) = cursor.try_next().await? {
        if !leaderboard.contains_key(&result.receiver) {
            leaderboard.insert(result.receiver.clone(), 1);
        } else {
            let count = leaderboard.get(&result.receiver).unwrap();
            leaderboard.insert(result.receiver.clone(), count + 1);
        }
    }

    Ok(leaderboard)
}

pub async fn get_unsigned_checkpoints(db: &Database) -> mongodb::error::Result<Vec<Checkpoint>> {
    let collection = db.collection::<Document>("signs");
    let find_options = FindOptions::builder().build();
    let mut cursor = collection.find(doc! {"txid": ""}, find_options).await?;
    let mut checkpoints: Vec<Checkpoint> = vec![];
    while let Some(result) = cursor.try_next().await? {
        let checkpoint = Checkpoint {
            checkpoint_id: result.get_str("checkpoint_id").unwrap().to_string(),
            signature: result.get_str("signature").unwrap().to_string(),
            txid: result.get_str("txid").unwrap().to_string(),
            receiver: result.get_str("receiver").unwrap().to_string(),
            metadata: return_single_checkpoint(
                db,
                result.get_str("checkpoint_id").unwrap().to_string(),
            )
            .await
            .unwrap(),
        };
        checkpoints.push(checkpoint);
    }
    Ok(checkpoints)
}
