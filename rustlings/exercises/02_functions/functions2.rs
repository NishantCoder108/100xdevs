// TODO: Add the missing type of the argument `num` after the colon `:`.
fn call_me(num: i32) -> u8 {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }

    3
}

fn main() {
    call_me(3);
}
