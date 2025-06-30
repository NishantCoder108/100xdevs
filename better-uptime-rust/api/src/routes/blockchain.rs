use crate::models::blockchain::BalanceResponse;
use poem::{
    IntoResponse, handler,
    web::{Json, Path},
};

async fn get_balance(addr: &str) -> Result<String, String> {
    // Simulate an asynchronous operation to fetch balance
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    if addr.is_empty() {
        Err("Address cannot be empty".to_string())
    } else {
        Ok("1000 SOL".to_string())
    }
}

// #[handler]
// pub async fn balance(Path(addr): Path<String>) -> impl IntoResponse {
//     match get_balance(&addr).await {
//         Ok(bal) => format!("Balance for {addr}: {bal}"),
//         Err(err) => format!("Error: {err}"),
//     }
// }

#[handler]
pub async fn balance(Path(addr): Path<String>) -> Json<BalanceResponse> {
    let bal = get_balance(&addr).await.unwrap_or("0".into());
    Json(BalanceResponse {
        address: addr,
        balance: bal,
    })
}
