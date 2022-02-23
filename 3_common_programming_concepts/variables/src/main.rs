fn main() {
    let mut x = 5;
    // defining a mutable variable
    x = x + 1;
    let x = x + 1;
    // variable shadowing
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    //shadowing can be used with different data types
    let spaces = "    ";
    let spaces = spaces.len();
    println!("{}", spaces);

    define_consts();
    char_variables();
    boolean_variables();
    integer_variables();
    float_variables();
    basic_operations();
    tuple_variables();
    arrays_variables()
}

fn integer_variables() {
    // integer variables
    let w: u32 = 98_666;
    // unsigned 32 bits int we can use "_" as a saparator the number
    let x: i64 = 0xff;
    // signed 64 bits int
    let y: isize = 0o77;
    // signed, but the size is based on the computer archtecture
    let z: usize = 0b1111_0000;
    // we can define the number as binary, octal or hexadecimal too
    println!("{} {} {} {}", w, x, y, z);
}

fn float_variables() {
    //floating-Point variables
    let x: f32 = 3.12;
    let y: f64 = 4.045785950;
    println!("{} {}", x, y);
}

fn basic_operations() {
    // basic operations
    println!("addition: 5 + 10 = {}", 5 + 10);
    println!("subtraction: 95.4 - 12.7 = {}", 95.4 - 12.7);
    println!("product: 4 * 3.7 = {}", 4.0 * 3.7);
    println!("division(float): 3.0 / 4.0 = {}", 3.0 / 4.0);
    println!("division(int): 3/4 = {}", 3 / 4);
    println!("remainder: 43 % 5 = {}", 43 % 5);
}

fn boolean_variables() {
    //boolean varibles
    let test: bool = false;
    println!("{}", test);
    let test = true; // implicit notation
    println!("{}", test);
}

fn char_variables() {
    //character type
    let c = 'c';
    // char is defined with single quotes
    let heart_eyed_cat = '😻';
    // rust char is an Unicode Scalar Value
    println!("{} {}", c, heart_eyed_cat);
}

fn define_consts() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // constants evaluating example
    println!("{}", THREE_HOURS_IN_SECONDS);
}

fn tuple_variables() {
    // tuple
    // tuples has a fixed lenght
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    println!("{} {} {}", z, y, x);
    //we can access a specific index by using '.'
    println!("{} {} {}", tup.0, tup.1, tup.2);
}

fn arrays_variables() {
    //arrays
    //arrays have a fixed lenght and every element has the same type
    let a: [i32; 3] = [1, 2, 3];
    println!("{} {} {}", a[0], a[1], a[2]);
    //initializing the array with same value for each element
    let a = [0; 2];
    println!("{} {}", a[0], a[1]); // acessing an array element
}
