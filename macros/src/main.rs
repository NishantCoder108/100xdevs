#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    println!("Hello, world!");

    let rect = Rect {
        height: 32,
        width: 43,
    };

    println!("Rectangle width : {}", rect.width);
    println!("Rec height : {}", rect.height);

    println!("Rect: {:?}", rect);
}
