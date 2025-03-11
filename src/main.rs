fn main() {
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh",
        "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    for (i, day) in days.iter().enumerate() {
        println!("\nOn the {} day of Christmas, my true love gave to me:", day);
        for j in (0..=i).rev() {
            if i == 0 {
                println!("{}.", gifts[j]);
            } else if j == 0 {
                println!("And {}.", gifts[j].to_lowercase());
            } else {
                println!("{},", gifts[j]);
            }
        }
    }
}
