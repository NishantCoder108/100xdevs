use poem::{handler, web::Path};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {name}")
}
