use crate::models::blockchain::{BalanceResponse, TxHistory, TxSignature};
use poem::{
    Result, handler,
    http::StatusCode,
    web::{Json, Path},
};
use solana_client::{
    nonblocking::rpc_client::RpcClient, rpc_client::GetConfirmedSignaturesForAddress2Config,
    rpc_response::RpcConfirmedTransactionStatusWithSignature,
};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey};
use std::str::FromStr;

fn get_solana_client() -> RpcClient {
    // RPC URL =https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a
    RpcClient::new_with_commitment(
        String::from("https://api.devnet.solana.com"),
        CommitmentConfig::confirmed(),
    )
}

pub async fn get_balance(address: &str) -> Result<u64, String> {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());

    let pubkey = Pubkey::from_str(address).map_err(|e| e.to_string())?;

    client.get_balance(&pubkey).await.map_err(|e| e.to_string())
}

#[handler]
pub async fn balance(Path(addr): Path<String>) -> Json<BalanceResponse> {
    let bal = get_balance(&addr).await.unwrap_or(0);
    Json(BalanceResponse {
        address: addr,
        balance: ((bal as f64) / 1000000000.0),
    })
}

// Get Transaction history for wallet address

#[handler]
pub async fn tx_history(
    Path(addr): Path<String>,
) -> Result<Json<Vec<RpcConfirmedTransactionStatusWithSignature>>, poem::Error> {
    let client = get_solana_client();

    let pubkey = match Pubkey::from_str(&addr) {
        Ok(p) => p,
        Err(e) => {
            return Err(poem::Error::from_string(
                "NOT_VALID_ADDRESS",
                StatusCode::NOT_FOUND,
            ));
        }
    };

    let signatures_for_config = GetConfirmedSignaturesForAddress2Config {
        before: None,
        until: None,
        limit: Some(10),
        commitment: CommitmentConfig::finalized().into(),
    };

    let signatures = client
        .get_signatures_for_address_with_config(&pubkey, signatures_for_config)
        .await;

    println!("{:#?}", signatures);

    // let signature = signatures
    //     .map_err(|e| poem::Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(signatures.unwrap_or_else(|_| vec![])))
}
