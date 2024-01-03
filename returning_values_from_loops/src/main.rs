
fn main() {
    println!("Hello, world! Returning Values from Loops");


    let mut counter = 0;

    let _result = loop {
        counter +=1 ;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {_result}");
}
