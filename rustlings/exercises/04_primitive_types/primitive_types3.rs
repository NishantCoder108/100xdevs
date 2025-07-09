fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a = [0; 110];
    let v: Vec<u32> = (1..=100).collect(); // Vec with 1 to 100
    println!("{:?}", v);
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
