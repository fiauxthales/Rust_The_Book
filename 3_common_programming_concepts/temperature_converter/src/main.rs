use std::io::stdin;

fn main() {
    println!("Temperature converter");
    'menu: loop {
        println!("1 - Celsius to Fahrenheit");
        println!("2 - Fahrenheit to Celsius");
        println!("3 - Exit");
        let mut operation = String::new();

        stdin()
            .read_line(&mut operation)
            .expect("Failed to read line");
        let op: u8 = match operation.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match op {
            1 => cel_to_far(),
            2 => far_to_cel(),
            3 => break 'menu,
            _ => continue,
        }
    }
}

fn cel_to_far() {
    let x = read_temperature();
    let y = (x * (9.0 / 5.0)) + 32.0;
    println!("{:.2}° Celsius is {:.2}° Fahrenheit", x, y);
}

fn far_to_cel() {
    let x = read_temperature();
    let y = (x - 32.0) * (5.0 / 9.0);
    println!("{:.2}° Fahrenheit is {:.2}° Celsius", x, y);
}

fn read_temperature() -> f64 {
    loop {
        println!("Insert temperature");
        let mut temp = String::new();
        stdin().read_line(&mut temp).expect("Failed to read line");
        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return temp;
    }
}
