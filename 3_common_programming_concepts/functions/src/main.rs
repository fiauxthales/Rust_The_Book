fn main() {
    another_function(5);
    print_labeled_measurement(5, 'm');
    expression_example();
    println!("Add result: {}", adding_function(63.5, 96.5));
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn expression_example() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}

fn adding_function(x: f64, y: f64) -> f64 {
    x + y
}
