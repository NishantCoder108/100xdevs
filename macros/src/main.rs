#[derive(Debug)]
struct LRect<'a, 'b> {
    width: u32,
    height: u32,
    name: &'a String,

    last_name: &'b String,
}

#[derive(Debug)]
struct Rect {
    width: u128,
    height: u128,
}
//Lifetime is a method in rust that provide ownership rule , if life time is not , then it will unncessary or might not compile the code.
//Here , rect1 and rect2 are both have lifetime ('a and 'b)
struct Shape<'a, 'b> {
    rect1: LRect<'a, 'b>,
    rect2: LRect<'a, 'b>,
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

//Life times
