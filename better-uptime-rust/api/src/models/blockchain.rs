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

#[derive(Serialize, Deserialize)]
pub struct TxHistory {
    pub signature: String,
    pub amount: String,
    pub from: String,
    pub to: String,
    pub status: String,
    pub slot: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub result: T,
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TxEntry {
    pub signature: String,
    pub slot: u64,
    pub err: Option<String>,
    pub memo: Option<String>,
    pub blockTime: Option<u64>,
    pub confirmationStatus: String,
}

pub type TxSignature = Vec<TxEntry>;
