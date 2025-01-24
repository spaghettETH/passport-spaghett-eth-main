use crate::modules::passport::model::UserDocument;
use ethers::types::U256;
use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiSuccess {
    pub message: String,
    pub details: String,
    pub error: bool,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub message: String,
    pub reason: String,
    pub error: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionCreated {
    pub message: String,
    pub session: String,
    pub error: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OwnedTickets {
    pub message: String,
    pub tickets: Vec<U256>,
    pub tiers: Vec<String>,
    pub error: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Leaderboard {
    pub message: String,
    pub leaderboard: HashMap<String, u128>,
    pub users: Vec<UserDocument>,
    pub error: bool,
}
