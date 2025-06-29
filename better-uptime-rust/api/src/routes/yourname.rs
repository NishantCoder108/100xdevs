use poem::{handler, web::Path};

#[handler]
pub fn your_name(Path(name): Path<String>) -> String {
    format!("Your name is : {name}")
}
