use crate::libs::eth::read::get_last_block;
use crate::libs::eth::write::{create_checkpoint, update_checkpoint, sign_checkpoint, sign_message};
use crate::libs::responses::{ApiError, ApiSuccess, Leaderboard};
use crate::modules::passport::crud::return_all_passports;
use crate::modules::admins::routes::is_admin;
use ethers::types::U256;
use mongodb::bson::doc;
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;

// Import Checkpoints structs
use crate::modules::checkpoints::crud::{
    balance_of_id, checkpoint_exists, find_collected_checkpoints, insert_new_checkpoint,
    insert_sign, return_all_checkpoints_for_event, return_checkpoint_for_slug, return_leaderboard,
    update_txid, edit_existing_checkpoint
};
use crate::modules::checkpoints::model::{
    Challenge, CheckpointInput, CheckpointJson, CollectedSigns,
};

use super::crud::{get_unsigned_checkpoints, return_single_checkpoint};

#[get("/checkpoint/challenge/<checkpoint_id>")]
pub async fn get_challenge(checkpoint_id: &str) -> Result<Json<Challenge>, Json<ApiError>> {
    let last_block = get_last_block().await;
    if !last_block.is_ok() {
        return Err(Json(ApiError {
            message: format!("Can't get challenge"),
            reason: last_block.unwrap().to_string(),
            error: true,
        }));
    }
    let challenge = format!(
        "{}-{:?}",
        checkpoint_id.to_uppercase(),
        last_block.as_ref().unwrap().clone()
    );
    let hashed = ethers::utils::hash_message(&challenge.as_bytes());
    let signed = sign_message(&format!("{:#x}", hashed)).await;
    if signed.is_err() {
        return Err(Json(ApiError {
            message: format!("Can't get challenge"),
            reason: "INVALID_SIGNATURE".to_string(),
            error: true,
        }));
    }
    let hex_string = checkpoint_id.to_string()
        + &"100001".to_owned()
        + &signed.unwrap().as_str().replace("0x", "");
    return Ok(Json(Challenge {
        id: checkpoint_id.to_string(),
        block: last_block.unwrap().to_string(),
        challenge: hex_string,
        error: false,
    }));
}

#[get("/checkpoint/sign/<address>/<challenge>")]
pub async fn sign_to_checkpoint(
    db: &State<Database>,
    address: &str,
    challenge: &str,
) -> Result<Json<ApiSuccess>, Json<ApiError>> {
    let parts = challenge.split("100001").collect::<Vec<&str>>();
    let checkpoint_id = parts[0];
    let _signature = parts[1];
    let balance = balance_of_id(db, address.to_owned(), checkpoint_id.to_owned()).await;
    if balance.is_err() {
        return Err(Json(ApiError {
            message: format!("Can't get balance"),
            reason: "INVALID_BALANCE".to_string(),
            error: true,
        }));
    }
    let balance_string = balance.unwrap().to_string();
    println!("Balance: {}", balance_string.clone());
    if balance_string.parse::<u32>().unwrap() < 1 {
        let stored_db = insert_sign(
            db,
            address.to_owned(),
            checkpoint_id.to_string(),
            "".to_string(),
            challenge.to_owned(),
        )
        .await;
        if stored_db.is_err() || stored_db.clone().unwrap() == "ALREADY_SIGNED" {
            println!("😵 Can't store sign receipt");
            return Err(Json(ApiError {
                message: format!("Can't store sign receipt."),
                reason: stored_db.unwrap().to_string(),
                error: true,
            }));
        }
        return Ok(Json(ApiSuccess {
            message: "Checkpoint sign stored!".to_owned(),
            details: stored_db.unwrap().to_string(),
            error: false,
        }));
        // match sign_checkpoint(
        //     db,
        //     U256::from_dec_str(checkpoint_id).unwrap(),
        //     address.to_owned(),
        // )
        // .await
        // {
        //     Ok(transaction_hash) => {

        //     }
        //     Err(_error) => {
        //         println!("{:?}", _error);
        //         return Err(Json(ApiError {
        //             message: format!("Can't sign, retry!"),
        //             reason: format!("{:?}", _error),
        //             error: true,
        //         }));
        //     }
        // }
    }
    return Ok(Json(ApiSuccess {
        message: format!("You already signed to this checkpoint!"),
        details: "".to_string(),
        error: true,
    }));
}

#[get("/checkpoint/collected/<address>")]
pub async fn collected_checkpoints(
    db: &State<Database>,
    address: &str,
) -> Result<Json<CollectedSigns>, Json<ApiError>> {
    let collected_checkpoints = find_collected_checkpoints(&db, address.to_owned())
        .await
        .unwrap();

    return Ok(Json(CollectedSigns {
        signs: collected_checkpoints,
        error: false,
    }));
}

#[post("/checkpoint/create", data = "<input>")]
pub async fn create_new_checkpoint(
    db: &State<Database>,
    input: Json<CheckpointInput>,
) -> Result<Json<ApiSuccess>, Json<ApiError>> {
    match is_admin(&input.session).await {
        true => {
            let exists = checkpoint_exists(db, input.checkpoint_slug.clone()).await;
            if exists.is_err() {
                return Err(Json(ApiError {
                    message: format!("Can't check metadata!"),
                    reason: "INVALID_METADATA".to_string(),
                    error: true,
                }));
            }
            if exists.unwrap() {
                return Err(Json(ApiError {
                    message: format!("Metadata already exists!"),
                    reason: "METADATA_EXISTS".to_string(),
                    error: true,
                }));
            }
            match create_checkpoint(
                db,
                input.name.clone(),
                input.description.clone(),
                input.image.clone(),
                input.checkpoint_slug.clone(),
                U256::from_dec_str(&input.timestamp_start).unwrap(),
                U256::from_dec_str(&input.timestamp_end).unwrap(),
            )
            .await
            {
                Ok(checkpoint_id) => {
                    let inserted: Result<String, mongodb::error::Error> = insert_new_checkpoint(
                        db,
                        input.name.clone(),
                        input.description.clone(),
                        input.image.clone(),
                        input.checkpoint_slug.clone(),
                        checkpoint_id.clone(),
                        input.timestamp_start.clone(),
                        input.timestamp_end.clone(),
                    )
                    .await;
                    if inserted.is_err() {
                        return Err(Json(ApiError {
                            message: format!("Can't insert metadata!"),
                            reason: "INVALID_METADATA".to_string(),
                            error: true,
                        }));
                    }
                    if inserted.unwrap() == "ERROR" {
                        return Err(Json(ApiError {
                            message: format!("Can't insert metadata!"),
                            reason: "METADATA_EXISTS".to_string(),
                            error: true,
                        }));
                    }
                    return Ok(Json(ApiSuccess {
                        message: "Metadata created correctly".to_owned(),
                        details: checkpoint_id.to_string(),
                        error: false,
                    }));
                }
                Err(_error) => {
                    println!("{:?}", _error);
                    return Err(Json(ApiError {
                        message: format!("Can't create metadata, retry!"),
                        reason: format!("{:?}", _error),
                        error: true,
                    }));
                }
            }
        }
        false => {
            return Err(Json(ApiError {
                message: "Invalid session".to_owned(),
                reason: "INVALID_SESSION".to_owned(),
                error: true,
            }));
        }
    }
}

#[post("/checkpoint/update", data = "<input>")]
pub async fn update_existing_checkpoint(
    db: &State<Database>,
    input: Json<CheckpointInput>,
) -> Result<Json<ApiSuccess>, Json<ApiError>> {
    match is_admin(&input.session).await {
        true => {
            let exists = checkpoint_exists(db, input.checkpoint_slug.clone()).await;
            if exists.is_err() {
                return Err(Json(ApiError {
                    message: format!("Can't check metadata!"),
                    reason: "INVALID_METADATA".to_string(),
                    error: true,
                }));
            }
            if !exists.unwrap() {
                return Err(Json(ApiError {
                    message: format!("Metadata doesn't exists!"),
                    reason: "METADATA_NOT_FOUND".to_string(),
                    error: true,
                }));
            }
            match update_checkpoint(
                db,
                input.name.clone(),
                input.description.clone(),
                input.image.clone(),
                input.checkpoint_slug.clone(),
                U256::from_dec_str(&input.timestamp_start).unwrap(),
                U256::from_dec_str(&input.timestamp_end).unwrap(),
            )
            .await
            {
                Ok(checkpoint_id) => {
                    let updated: Result<String, mongodb::error::Error> = edit_existing_checkpoint(
                        db,
                        input.name.clone(),
                        input.description.clone(),
                        input.image.clone(),
                        input.checkpoint_slug.clone(),
                        checkpoint_id.clone(),
                        input.timestamp_start.clone(),
                        input.timestamp_end.clone(),
                    )
                    .await;
                    if updated.is_err() {
                        return Err(Json(ApiError {
                            message: format!("Can't update metadata!"),
                            reason: "INVALID_METADATA".to_string(),
                            error: true,
                        }));
                    }
                    if updated.unwrap() == "ERROR" {
                        return Err(Json(ApiError {
                            message: format!("Can't update metadata!"),
                            reason: "METADATA_NOT_FOUND".to_string(),
                            error: true,
                        }));
                    }
                    return Ok(Json(ApiSuccess {
                        message: "Metadata updated correctly".to_owned(),
                        details: checkpoint_id.to_string(),
                        error: false,
                    }));
                }
                Err(_error) => {
                    println!("{:?}", _error);
                    return Err(Json(ApiError {
                        message: format!("Can't create metadata, retry!"),
                        reason: format!("{:?}", _error),
                        error: true,
                    }));
                }
            }
        }
        false => {
            return Err(Json(ApiError {
                message: "Invalid session".to_owned(),
                reason: "INVALID_SESSION".to_owned(),
                error: true,
            }));
        }
    }
}



#[get("/checkpoints/<name>")]
pub async fn get_checkpoints_for_event(
    db: &State<Database>,
    name: &str,
) -> Result<Json<Vec<CheckpointJson>>, Json<ApiError>> {
    match return_all_checkpoints_for_event(db, name).await {
        Ok(metadata) => {
            return Ok(Json(metadata));
        }
        Err(_error) => {
            return Err(Json(ApiError {
                message: format!("Can't get metadata!"),
                reason: format!("{:?}", _error),
                error: true,
            }));
        }
    }
}

#[get("/checkpoint/id/<slug>")]
pub async fn get_checkpoint_id(
    db: &State<Database>,
    slug: &str,
) -> Result<Json<CheckpointJson>, Json<ApiError>> {
    match return_checkpoint_for_slug(db, slug).await {
        Ok(metadata) => {
            return Ok(Json(metadata));
        }
        Err(_error) => {
            return Err(Json(ApiError {
                message: format!("Can't get metadata!"),
                reason: format!("{:?}", _error),
                error: true,
            }));
        }
    }
}

#[get("/checkpoint/metadata/<checkpoint_id>")]
pub async fn get_checkpoint_metadata(
    db: &State<Database>,
    checkpoint_id: &str,
) -> Result<Json<CheckpointJson>, Json<ApiError>> {
    match return_single_checkpoint(db, checkpoint_id.to_owned()).await {
        Ok(metadata) => {
            return Ok(Json(metadata));
        }
        Err(_error) => {
            return Err(Json(ApiError {
                message: format!("Can't get metadata!"),
                reason: format!("{:?}", _error),
                error: true,
            }));
        }
    }
}

#[get("/leaderboard")]
pub async fn get_leaderboard(db: &State<Database>) -> Result<Json<Leaderboard>, Json<ApiError>> {
    match return_leaderboard(db).await {
        Ok(leaderboard) => match return_all_passports(db).await {
            Ok(users) => {
                return Ok(Json(Leaderboard {
                    message: "Leaderboard fetched successfully".to_string(),
                    leaderboard,
                    users,
                    error: false,
                }));
            }
            Err(_error) => {
                return Err(Json(ApiError {
                    message: format!("Can't get leaderboard!"),
                    reason: format!("{:?}", _error),
                    error: true,
                }));
            }
        },
        Err(_error) => {
            return Err(Json(ApiError {
                message: format!("Can't get leaderboard!"),
                reason: format!("{:?}", _error),
                error: true,
            }));
        }
    }
}

#[get("/process-unsigned-checkpoints")]
pub async fn process_unsigned_checkpoints(
    db: &State<Database>,
) -> Result<Json<ApiSuccess>, Json<ApiError>> {
    let unsigned_checkpoints = get_unsigned_checkpoints(db).await;
    if unsigned_checkpoints.is_err() {
        return Err(Json(ApiError {
            message: format!("Can't get unsigned checkpoints!"),
            reason: format!("{:?}", unsigned_checkpoints.unwrap()),
            error: true,
        }));
    }
    let unsigned_checkpoints = unsigned_checkpoints.unwrap();
    for checkpoint in unsigned_checkpoints.clone() {
        println!("Checkpoint: {:?}", checkpoint);
        match sign_checkpoint(
            db,
            U256::from_dec_str(&checkpoint.checkpoint_id).unwrap(),
            checkpoint.receiver.clone(),
        )
        .await
        {
            Ok(tx) => {
                println!("Transaction sent: {:?}", tx);
                match update_txid(
                    db,
                    checkpoint.checkpoint_id.clone(),
                    checkpoint.receiver.clone(),
                    tx.clone(),
                )
                .await
                {
                    Ok(_) => {
                        println!("Transaction updated: {:?}", tx);
                    }
                    Err(_error) => {
                        println!("Can't update transaction: {:?}", _error);
                    }
                }
            }
            Err(_error) => {
                println!("Can't sign checkpoint: {:?}", _error);
                continue;
            }
        };
    }
    Ok(Json(ApiSuccess {
        message: "Unsigned checkpoints processed!".to_owned(),
        details: unsigned_checkpoints.len().to_string(),
        error: false,
    }))
}
