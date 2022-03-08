fn main() {
    let gifts = [
        "a Patridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six geese a Laying",
        "Seven Swans a Swinmming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "Twelve drummers drumming"
    ];
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];
    for i in 0..12 {
        println!("For the {} day of Christmas my true love sent to me", days[i]);

        for j in (0..=i).rev() {
            if j==0 && i!=0{
                print!("and ");
            }
            println!("{}", gifts[j]);
        }
        println!();
    }
}
