use poem::{handler, web::Path};

#[handler]
pub fn health() -> &'static str {
    "Health of application is good."
}

#[handler]
pub fn home() -> &'static str {
    "Welcome to my application!"
}
