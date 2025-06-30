pub mod models;
pub mod routes;

use poem::{
    EndpointExt, Route, Server, get, handler, listener::TcpListener, middleware::Tracing, web::Path,
};
use routes::{
    blockchain::{balance, tx_history},
    hello::hello,
    internal::{health, home},
    yourname::your_name,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/hello/:name", get(hello))
        .at("/yourname/:name", get(your_name))
        .at("/health", get(health))
        .at("/", get(home))
        // Adding a route for blockchain balance
        .at("/balance/:addr", get(balance))
        .at("/tx_history/:addr", get(tx_history));

    println!("Starting server on http://localhost:3000");
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
