fn main() {
    if_expressions();
    labeled_loop_break();
    loop_return();
    while_example();
    do_while_exemple();
    for_example();
    for_range_example();
}

fn labeled_loop_break() {
    // an example of how to use labels on loops
    let mut count = 0;
    'counting_up: loop {
        println!("Count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn if_expressions() {
    // if in let statement
    let condition = true;
    let number = if condition { 4 } else { 6 };

    println!("The value of number is: {}", number);

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2")
    }
}

fn loop_return(){
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 10{
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}

fn while_example(){
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn do_while_exemple() {
    // Rust has no wo-while loop with syntax sugar.
    // This is an alternative implementation
    let mut number = 3;
    while{
        println!("{}", number);
        number -= 1;
        number != 0
    } {};
    println!("LIFTOFF!!!");
}

fn for_example(){
    let a = [10, 20, 30, 40, 50];

    for element in a{
        println!("The value is: {}", element);
    }
}

fn for_range_example(){
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}