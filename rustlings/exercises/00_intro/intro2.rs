fn main() {
    // TODO: Fix the code to print "Hello world!".
    println!("Hello world!");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("{}", spaces);

    let mut spaces = "     ";
    let spaces = spaces.len(); // here if we remove let , it will give error. shadowing work with same variable name but different types

    println!("{}", spaces);
}
