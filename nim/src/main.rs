use std::io;

fn menu(){
        println!("##############" );
        println!("#### NIM #####" );
        println!("##############" );
        println!("Welcome to the game of nim. The point of the game");
        println!("is to make sure you are not the one to pick up the" );
        println!("last stick." );
}

fn() -> i32{
    println!("How many sticks should there be?" );
    let mut stickNumber = String::new();
    io::stdin().read_line(&mut stickNumber).unwrap();

    return stickNumber;
}

fn main() {
    menu();
    let mut stickNumber =

}
