use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};


#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheckpointDocument {
    pub _id: ObjectId,
    pub checkpoint_id: String,
    pub signature: String,
    pub receiver: String,
    pub txid: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Checkpoint {
    pub checkpoint_id: String,
    pub signature: String,
    pub txid: String,
    pub receiver: String,
    pub metadata: CheckpointJson,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Challenge {
    pub id: String,
    pub block: String,
    pub challenge: String,
    pub error: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CollectedSigns {
    pub signs: Vec<Checkpoint>,
    pub error: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheckpointJson {
    pub name: String,
    pub description: String,
    pub image: String,
    pub timestamp_start: String,
    pub timestamp_end: String,
    pub checkpoint_slug: String,
    pub checkpoint_id: String,
    pub hash: String
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheckpointInput {
    pub description : String,
    pub name: String,
    pub image: String,
    pub timestamp_start: String,
    pub timestamp_end: String,
    pub checkpoint_slug: String,
    pub session: String
}