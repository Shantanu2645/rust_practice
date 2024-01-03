fn main(){
    let x = 5;
    let x = x + 10;

    {
        let x = x * 2;
        println!("The value of the x is {x}");
    }
    println!("The value of the x is {}", &x);
}