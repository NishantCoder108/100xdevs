use poem::{handler, web::Path};

#[handler]
fn your_name(Path(name): Path<String>) -> String {
    format!("Your name is : {name}")
}
