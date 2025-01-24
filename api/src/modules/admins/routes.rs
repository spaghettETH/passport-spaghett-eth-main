use crate::libs::eth::read::verify_signature;
use crate::libs::responses::{ApiError, ApiSuccess, SessionCreated};
use crate::modules::admins::model::{CreateSessionInput, SessionJson, VerifySessionInput};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use rocket::serde::json::Json;
use serde_json::json;
use std::env;

#[post("/session/create", data = "<input>")]
pub async fn create_user_session(
    input: Json<CreateSessionInput>,
) -> Result<Json<SessionCreated>, Json<ApiError>> {
    let verified = verify_signature(&input.challenge, &input.signature, &input.address).await;
    if &verified.unwrap().to_ascii_lowercase() == &input.address.to_ascii_lowercase() {
        let secret = &env::var("SECRET_KEY");
        let hashed = ethers::utils::hash_message(&secret.as_ref().unwrap().as_bytes());
        let key = Key::<Aes256Gcm>::from_slice(&hashed.as_bytes());
        let cipher = Aes256Gcm::new(&key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let admin = &env::var("ADMIN_ADDRESS");
        if &input.address.to_ascii_lowercase() != &admin.to_owned().unwrap().to_ascii_lowercase() {
            return Err(Json(ApiError {
                message: "Not the admin".to_owned(),
                reason: "INVALID_ADDRESS".to_owned(),
                error: true,
            }));
        }
        let session = SessionJson {
            address: input.address.clone(),
            signature: input.signature.clone(),
            created_at: chrono::Utc::now().timestamp(),
        };
        let session_string = json!(session).to_string();

        match cipher.encrypt(&nonce, session_string.to_owned().as_bytes()) {
            Ok(ciphertext) => {
                let encoded_nonce = hex::encode(nonce.clone());
                let encoded = hex::encode(ciphertext.clone());
                let session = format!("0x{}{}", encoded_nonce, encoded);
                // Decrypt the message, using the same key and nonce
                let decoded = hex::decode(session.clone().replace("0x", "")).unwrap();
                let decoded_nonce = Nonce::from_slice(&decoded[0..96 / 8]);
                let decoded_ciphertext = &decoded[96 / 8..];
                match cipher.decrypt(&decoded_nonce, decoded_ciphertext.as_ref()) {
                    Ok(plaintext) => {
                        let original_content = String::from_utf8(plaintext).unwrap();
                        let original_session: SessionJson =
                            serde_json::from_str(original_content.as_str()).unwrap();
                        if &original_session.address != &input.address {
                            return Err(Json(ApiError {
                                message: "Can't create session".to_owned(),
                                reason: "DECRYPTION_FAILS".to_owned(),
                                error: true,
                            }));
                        }
                        return Ok(Json(SessionCreated {
                            message: "Session created correctly".to_owned(),
                            session: session,
                            error: false,
                        }));
                    }
                    Err(_) => {
                        return Err(Json(ApiError {
                            message: "Can't create session".to_owned(),
                            reason: "DECRYPTION_ERROR".to_owned(),
                            error: true,
                        }));
                    }
                }
            }
            Err(_) => {
                return Err(Json(ApiError {
                    message: "Can't create session".to_owned(),
                    reason: "ENCRYPTION_ERROR".to_owned(),
                    error: true,
                }));
            }
        }
    } else {
        return Err(Json(ApiError {
            message: "Invalid signature".to_owned(),
            reason: "INVALID_SIGNATURE".to_owned(),
            error: true,
        }));
    }
}

#[post("/session/verify", data = "<input>")]
pub async fn verify_user_session(
    input: Json<VerifySessionInput>,
) -> Result<Json<ApiSuccess>, Json<ApiError>> {
    match is_admin(&input.session).await {
        true => {
            return Ok(Json(ApiSuccess {
                message: "Session verified".to_owned(),
                details: "".to_owned(),
                error: false,
            }));
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

pub async fn is_admin(session: &str) -> bool {
    let secret = &env::var("SECRET_KEY");
    let hashed = ethers::utils::hash_message(&secret.as_ref().unwrap().as_bytes());
    let key = Key::<Aes256Gcm>::from_slice(&hashed.as_bytes());
    let cipher = Aes256Gcm::new(&key);
    let decoded = hex::decode(&session.replace("0x", "")).unwrap();
    let decoded_nonce = Nonce::from_slice(&decoded[0..96 / 8]);
    let decoded_ciphertext = &decoded[96 / 8..];
    match cipher.decrypt(&decoded_nonce, decoded_ciphertext.as_ref()) {
        Ok(plaintext) => {
            let original_content = String::from_utf8(plaintext).unwrap();
            let original_session: SessionJson =
                serde_json::from_str(original_content.as_str()).unwrap();
            let admin = &env::var("ADMIN_ADDRESS");
            if &original_session.address.to_ascii_lowercase()
                != &admin.to_owned().unwrap().to_ascii_lowercase()
            {
                return false;
            } else {
                return true;
            }
        }
        Err(_) => {
            return false;
        }
    }
}
