fn main() {
    let mut x: i32  = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const SECOND_IN_A_DAY: u32 = 24 * 60 * 60;

    println!("There is {SECOND_IN_A_DAY} in a day.")
}
