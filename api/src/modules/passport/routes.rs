use crate::libs::eth::read::{balance_of, verify_signature, Profile};
use crate::libs::eth::write::mint_passport;
use crate::libs::responses::{ApiError, ApiSuccess};
use crate::modules::passport::model::{SetProfileInput, UserDocument};
use mongodb::bson::doc;
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;

#[get("/passport/check/<address>")]
pub async fn get_passport(address: String) -> Result<Json<ApiSuccess>, Json<ApiError>> {
    let balance_of = balance_of(address.clone()).await;
    if balance_of.is_err() {
        return Err(Json(ApiError {
            message: format!("Can't read balance, retry!"),
            reason: format!("{:?}", balance_of.unwrap_err()),
            error: true,
        }));
    }
    if balance_of.unwrap() > 0 {
        return Err(Json(ApiError {
            message: format!("Passport already minted"),
            reason: format!("PASSPORT_ALREADY_MINTED"),
            error: true,
        }));
    }
    return Ok(Json(ApiSuccess {
        message: "Passport can be minted".to_string(),
        details: "".to_string(),
        error: false,
    }));
}

#[get("/passport/mint/<address>")]
pub async fn req_mint_passport(
    db: &State<Database>,
    address: String,
) -> Result<Json<ApiSuccess>, Json<ApiError>> {
    let balance_of = balance_of(address.clone()).await;
    if balance_of.is_err() {
        return Err(Json(ApiError {
            message: format!("Can't read balance, retry!"),
            reason: format!("{:?}", balance_of.unwrap_err()),
            error: true,
        }));
    }
    if balance_of.unwrap() > 0 {
        return Err(Json(ApiError {
            message: format!("Passport already minted"),
            reason: format!("PASSPORT_ALREADY_MINTED"),
            error: true,
        }));
    }
    match mint_passport(db, address).await {
        Ok(transaction_hash) => {
            return Ok(Json(ApiSuccess {
                message: "Passport minted successfully".to_string(),
                details: transaction_hash,
                error: false,
            }));
        }
        Err(_error) => {
            println!("{:?}", _error);
            return Err(Json(ApiError {
                message: format!("Can't mint passport, retry!"),
                reason: format!("{:?}", _error),
                error: true,
            }));
        }
    }
}

#[get("/passport/profile/<address>")]
pub async fn get_profile(address: String) -> Result<Json<Profile>, Json<ApiError>> {
    let profile = crate::libs::eth::read::get_passport_profile(address.clone()).await;
    if profile.is_err() {
        return Err(Json(ApiError {
            message: format!("Can't read profile, retry!"),
            reason: format!("{:?}", profile.unwrap_err()),
            error: true,
        }));
    }
    return Ok(Json(profile.unwrap()));
}

#[post("/profile", data = "<input>")]
pub async fn set_user_profile(
    db: &State<Database>,
    input: Json<SetProfileInput>,
) -> Result<Json<ApiSuccess>, Json<ApiError>> {
    let verified = verify_signature("SET PASSPORT PROFILE", &input.signature, &input.address).await;
    if &verified.unwrap().to_ascii_lowercase() == &input.address.to_ascii_lowercase() {
        let tx = crate::libs::eth::write::set_profile(
            db,
            &input.name,
            &input.image,
            &input.description,
            &input.address,
        )
        .await;
        if tx.is_err() {
            return Err(Json(ApiError {
                message: "Can't set profile, retry!".to_owned(),
                reason: format!("{:?}", tx.unwrap_err()),
                error: true,
            }));
        }
        Ok(Json(ApiSuccess {
            message: "Profile set correctly".to_owned(),
            details: tx.unwrap(),
            error: false,
        }))
    } else {
        Err(Json(ApiError {
            message: "Invalid signature".to_owned(),
            reason: "INVALID_SIGNATURE".to_owned(),
            error: true,
        }))
    }
}

#[get("/passport/sync")]
pub async fn sync_passports(db: &State<Database>) -> Result<Json<ApiSuccess>, Json<ApiError>> {
    let events = crate::libs::eth::events::get_passport_set_events().await;
    if events.is_err() {
        return Err(Json(ApiError {
            message: "Can't get events, retry!".to_owned(),
            reason: format!("{:?}", events.unwrap_err()),
            error: true,
        }));
    }
    for event in events.unwrap().iter() {
        let block_number = event.0;
        let user_address = format!("{:#x}", event.1);
        let user_name = event.2.to_string();
        let user_description = event.3.to_string();
        let user_image = event.4.to_string();
        let collection = db.collection::<UserDocument>("users");
        let check_user = collection
            .find_one(doc! { "address": user_address.clone() }, None)
            .await;
        if check_user.is_err() {
            return Err(Json(ApiError {
                message: "Can't check user, retry!".to_owned(),
                reason: format!("{:?}", check_user.unwrap_err()),
                error: true,
            }));
        }
        if check_user.clone().unwrap().is_none() {
            println!("User {} is not synced, syncing...", user_address);
            let user = UserDocument {
                address: user_address.clone(),
                name: user_name.clone(),
                description: user_description.clone(),
                image: user_image.clone(),
                updated_at: block_number as i64,
            };
            let _ = collection.insert_one(user, None).await;
        } else if check_user.clone().unwrap().unwrap().updated_at < block_number as i64 {
            println!("User {} is outdated, syncing...", user_address);
            let _ = collection
                .update_one(
                    doc! { "address": user_address.clone() },
                    doc! { "$set": { "updated_at": block_number as i64, "name": user_name, "description": user_description, "image": user_image } },
                    None,
                )
            .await;
        } else {
            println!("User {} already synced at block {}", user_address, check_user.unwrap().unwrap().updated_at);
        }
    }
    Ok(Json(ApiSuccess {
        message: "Passports synced correctly".to_owned(),
        details: "".to_owned(),
        error: false,
    }))
}
