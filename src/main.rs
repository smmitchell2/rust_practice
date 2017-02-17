use std::io;

fn add(a : u32, b : u32) -> u32 {
    return a + b;
}

fn main() {
    let a = 10;
    let b = 20;
    let c = add(a,b);
    println!("10+20 = {}",c );
}
