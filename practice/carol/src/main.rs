fn main() {
    let sequence = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let lylic_block = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings, badam-pam-pam",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for (day, day_name) in sequence.iter().enumerate() { // enumerate로 (인덱스, 값)
        println!("On the {day_name} day of Christmas");
        println!("My true love gave to me");

        for n in (0..=day).rev() {
            if day != 0 && n == 0 {
                print!("And ");
            }
            println!("{}", lylic_block[n]);
        }
        println!("");
    }
}