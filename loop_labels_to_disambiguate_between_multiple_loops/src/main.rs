fn main() {
    let mut counter = 0;

    'continue_up : loop {

        println!("Count = {counter}");
        let mut remaining = 10;

        loop {

            println!("This is remaining {remaining}");
            if remaining == 9{
                break;
            }

            if counter == 2 {

                break 'continue_up;
                
            }

            remaining -= 1;
        }

        counter += 1;
    }

    println!("End count = {counter}");

}
