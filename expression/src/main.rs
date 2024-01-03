use std::io;

fn main() {
    println!("Hello, world!");

    println!("This is a code test");

    let name= get_name();

    println!("Hello dear {name}");
}

fn get_name() -> String {


    println!("Please enter your name:");

    let mut name = String::new();

    io::stdin().read_line(&mut name).expect("Unable to read your name dear!");

    name




}
