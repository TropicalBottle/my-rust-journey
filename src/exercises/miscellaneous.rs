#[allow(dead_code)]
pub fn sing_christmas() {
    const DAYS_OF_CHRISTMAS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    const POSSIBLE_GIFTS: [&str; 12] = [
        "A partridge in a pear tree.",
        "Two turtle doves",
        "Three French Hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a laying,",
        "Seven swans a swimming,",
        "Eight maids a milking,",
        "Nine ladies dancing,",
        "Ten lords a leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    let mut active_gifts: Vec<&str> = Vec::new();

    for (actual_day, day_of_christmas) in DAYS_OF_CHRISTMAS.iter().enumerate() {
        println!(
            "On the {} day of Christmas\nMy true love sent to me:",
            day_of_christmas
        );
        for (actual_gift, possible_gift) in POSSIBLE_GIFTS.iter().enumerate() {
            if actual_gift <= actual_day {
                active_gifts.push(possible_gift);
            }
        }
        for active_gift in active_gifts.iter().rev() {
            println!("{}", active_gift);
        }

        active_gifts.clear();
        println!("\n");
    }
}
