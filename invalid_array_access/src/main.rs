use std::io;

fn main() {

    let a = [1,2,3,4,5,6,7,8,9];
    let mut index = String::new();

    println!("Enter a array index number to fetch for the array");

    io::stdin()
        .read_line(&mut index)
        .expect("can read the input");

    let index: usize = index.trim().parse().expect("Index you enter is not a number");

    let element = a[index];

    println!("The value of the index number {index} is {element}");

    


}
