use poem::{
    EndpointExt, Route, Server, get, handler, listener::TcpListener, middleware::Tracing, web::Path,
};

// pub mod routes;
#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {name}")
}

#[handler]
fn your_name(Path(name): Path<String>) -> String {
    format!("Your name is : {name}")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // if std::env::var_os("RUST_LOG").is_none() {
    //     std::env::set_var("RUST_LOG", "poem=debug");
    // }
    // tracing_subscriber::fmt::init();

    let app = Route::new()
        .at("/hello/:name", get(hello))
        .at("/yourname/:name", get(your_name))
        .with(Tracing);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
