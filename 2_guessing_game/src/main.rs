use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("Guess the number!");
    let range: u32;
    loop {
        println!("select difficulty:");
        println!("1. Easy");
        println!("2. Medium");
        println!("3. Hard");

        let mut diff = String::new();
        stdin().read_line(&mut diff).expect("Failed to read line");
        let diff: u8 = match diff.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match diff {
            1 => {
                range = 10;
                break;
            }
            2 => {
                range = 30;
                break;
            }
            3 => {
                range = 50;
                break;
            }
            _ => continue,
        }
    }

    let secret_number = rand::thread_rng().gen_range(1..=range);
    loop {
        println!("Please input your guess:");
        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
