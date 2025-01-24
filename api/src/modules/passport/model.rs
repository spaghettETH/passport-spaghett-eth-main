use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetProfileInput {
    pub address: String,
    pub name: String,
    pub description: String,
    pub image: String,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDocument {
    pub address: String,
    pub name: String,
    pub description: String,
    pub image: String,
    pub updated_at: i64,
}