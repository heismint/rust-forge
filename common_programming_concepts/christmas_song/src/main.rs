// 🎅🎶 The Twelve Days of Christmas Program.
// This Rust program prints the lyrics of the Christmas carol
// “The Twelve Days of Christmas.” Each day adds a new gift to the list,
// and all previous gifts are repeated in reverse order (as in the song).
// It uses two lists: one for the days (first, second, etc.) and another for the gifts.
// Loops are used to go through each day and print the appropriate gifts.

fn main() {
    println!("🎄 The Twelve Days of Christmas 🎶\n");

    // List of all gifts for each day
    let gifts = [
        "a partridge in a pear tree.",
        "two turtle doves,",
        "three French hens,",
        "four calling birds,",
        "five golden rings,",
        "six geese a-laying,",
        "seven swans a-swimming,",
        "eight maids a-milking,",
        "nine ladies dancing,",
        "ten lords a-leaping,",
        "eleven pipers piping,",
        "twelve drummers drumming,",
    ];

    // List of the ordinal numbers for each day
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    // Loop through all 12 days
    for day in 0..12 {
        println!("On the {} day of christmas,", days[day]);
        println!("my true love sent to me:");

        // Loop backwards through the gifts(to print them in the correct order)
        for gift in (0..=day).rev() {
            if day > 0 && gift == 0 {
                println!("and {}", gifts[gift]); // Add "and" from second day onward 
            } else {
                println!("{}", gifts[gift]);
            }
        }

        println!(); // Blank line between verses
    }
}
