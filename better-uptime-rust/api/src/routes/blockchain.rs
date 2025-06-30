use crate::models::blockchain::BalanceResponse;
use poem::{
    handler,
    web::{Json, Path},
};

use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

// RPC URL =https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a

pub async fn get_balance(address: &str) -> Result<u64, String> {
    let client = RpcClient::new("https://api.devnet.solana.com");

    let pubkey = Pubkey::from_str(address).map_err(|e| e.to_string())?;

    client.get_balance(&pubkey).map_err(|e| e.to_string())
}

#[handler]
pub async fn balance(Path(addr): Path<String>) -> Json<BalanceResponse> {
    let bal = get_balance(&addr).await.unwrap_or(0);
    Json(BalanceResponse {
        address: addr,
        balance: ((bal as f64) / 1000000000.0),
    })
}
