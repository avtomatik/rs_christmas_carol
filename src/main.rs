fn main() {
    let items_advent = [
        "A partridge in a pear tree",
        "Two turtle doves and",
        "Three french hens",
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

    // TODO: Need to Know What to Do with This Warning
    let mut suffix = "";

    for (number, item) in items_advent.iter().enumerate() {
        match number {
            0 => suffix = "st",
            1 => suffix = "nd",
            2 => suffix = "rd",
            _ => suffix = "th",
        }
        println!("On the {}{suffix} day of Christmas, my true love sent to me", number + 1);
        if number == 0 {println!("{item}.");} else {
            for index in (0..(number + 1)).rev() {
                println!("{}", items_advent[index]);
            }
        }
    }
}
