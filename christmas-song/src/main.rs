fn main() {
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let days = [
        "First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth", "Ninth",
        "Tenth", "Eleventh", "Twelveth",
    ];

    let mut count = 1;

    for day in days {
        println!("On the {day} day of Christmas my true love sent to me");

        for index in (1..count).rev() {
            println!("{},", gifts[index]);
        }
        if count > 1 {
            println!("And {}.", gifts[0].to_lowercase())
        } else {
            println!("{}.", gifts[0])
        }
        count += 1;
        println!("\n\n");
    }
}
