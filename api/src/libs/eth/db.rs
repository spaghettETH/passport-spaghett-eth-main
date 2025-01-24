// Importing dependencies
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::H256;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, DateTime, Document};
use mongodb::Database;
use serde::{Deserialize, Serialize};
extern crate dotenv;
use dotenv::dotenv;
use std::env;
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxDocument {
    pub _id: ObjectId,
    pub from: String,
    pub receiver: String,
    pub txid: String,
    pub nonce: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tx {
    pub from: String,
    pub receiver: String,
    pub txid: String,
    pub nonce: String,
    pub created_at: String,
}

pub async fn is_nonce_valid(
    db: &Database,
    sender: String,
    nonce: String,
) -> mongodb::error::Result<bool> {
    dotenv().ok();
    let collection = db.collection::<TxDocument>("transactions");
    let check = collection
        .find_one(
            doc! { "from": sender.to_string(), "nonce": nonce.to_string() },
            None,
        )
        .await?;
    if !check.is_none() {
        println!("💿 Checking nonce with blockchain..");
        // Check if txid is valid
        let provider_url = &env::var("ETH_PROVIDER_URL").unwrap();
        let provider = Provider::<Http>::try_from(provider_url.as_str());
        if provider.is_err() {
            println!("💀 Can't create provider: {:?}", provider.err());
            Ok(false)
        } else {
            let transaction_hash = check.unwrap().txid.parse::<H256>().unwrap();
            let transaction = provider.unwrap().get_transaction(transaction_hash).await;
            if transaction.is_err() || transaction.unwrap().is_none() {
                println!("👀 Tx with this nonce is not valid, can be replaced!");
                Ok(true)
            } else {
                Ok(false)
            }
        }
    } else {
        Ok(true)
    }
}

pub async fn insert_tx(
    db: &Database,
    sender: String,
    nonce: String,
    txid: String,
    receiver: String,
) -> mongodb::error::Result<String> {
    let collection = db.collection::<Document>("transactions");
    let created_at: DateTime = DateTime::now();

    let check = collection
        .find_one(
            doc! { "from": sender.to_string(), "nonce": nonce.to_string() },
            None,
        )
        .await?;
    if check.is_none() {
        let insert_one_result = collection
            .insert_one(
                doc! {
                    "from": sender.clone(),
                    "nonce": nonce.clone(),
                    "txid": txid.clone(),
                    "receiver": receiver.clone(),
                    "created_at": created_at.to_string()
                },
                None,
            )
            .await?;

        Ok(insert_one_result.inserted_id.to_string())
    } else {
        Ok("ERROR".to_string())
    }
}
