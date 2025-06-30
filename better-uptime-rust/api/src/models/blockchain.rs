use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateUserInput {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct BalanceResponse {
    pub address: String,
    pub balance: f64,
}
