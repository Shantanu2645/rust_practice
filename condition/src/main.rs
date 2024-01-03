use std::io;

fn main() {
    println!("Hello, world!");
    println!("I hope the program of check the knowledge of if conditon will work very fine");

    println!("Please input a number please");
    let mut input: String = String::new();
       io::stdin().read_line(&mut input).expect("I cant read the input sorry");

    let input: i32 = match input.trim().parse() {
    Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    println!("Your input is {input}");

    let result_var: &str =  verify(input);

    println!("{result_var}")
}

fn verify(input: i32) -> &'static str {

    if input < 1 {

        "This is less than 1"
    }

    else if input < 2 {
        "This is less than 2"
        
    }
    else {
        "This is bigger than 2"
    }

}