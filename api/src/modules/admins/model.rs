use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateSessionInput {
    pub address: String,
    pub signature: String,
    pub challenge: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionJson {
    pub address: String,
    pub signature: String,
    pub created_at: i64,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifySessionInput {
    pub session: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDocument {
    pub address: Option<String>,
    pub name: Option<String>,
    // ... other fields ...
}
