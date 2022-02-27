use std::io::stdin;

fn main() {
    println!("Generate the nth Fibonacci numbers");
    loop{
        let mut n = String::new();
        stdin().read_line(&mut n).expect("Failed read line");
        let n: u64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        fibonacci(n, 0, 0, 0);
    }
}

fn fibonacci(x: u64, n: u64, fm1: u128, fm2: u128 ) {
    let mut n = n;
    let mut fm1 = fm1;
    let mut fm2 = fm2;
    if n>2 {
        println!("n({}) = {}",n, fm1 + fm2);
    } else if n==0{
        println!("n(0) = 0");
        fm1 = 0;
        fm2 = 0;

    } else {
        println!("n(1) = 1");
        fm1 = 0;
        fm2 = 1;
    }
    n += 1;
    if n <= x{
        fibonacci(x, n, fm2, fm1+fm2);
    };
}