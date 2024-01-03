fn main() {
    println!("This is the main function");
    another_func();

}


fn another_func (){
    println!("This is the another function");

    let a = 32;

    addition(a);

    name("shantanu dey anik");

}

fn addition(x: i32){
    let result = x + 5;
    println!("The result is {result}") ;
}

fn name(name: &str ){

    println!("The name of the programmer is {name}");

}