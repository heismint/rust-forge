// Import the standard input/output library so we can take input from the user
use std::io;

// Import the Rng trait from the rand crate needed to generate random numbers
use rand::Rng;

// Import the Ordering enum, helps compare values (less than, greater than, equal)
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // Generate a random secret number between 1 and 100
    // rand::thread_rng() gives us a random number generator
    // gen_range(1, 101) means "generate a number from 1 up to but not including 101"
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Uncomment the line below to see the secret number, good for debugging or testing
    //println!("The secret number is: {}", secret_number);

    // Start an infinite loop so the user can keep guessing until they get it right
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // Take input from the user through the terminal
        // read_line stores what the user types into the 'guess' variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // Handle errors in case input fails

        // Convert the user input (which is text) into a number (u32)
        // trim() removes spaces/newlines, parse() converts string to number
        // match handles the case where user doesn’t type a valid number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // If conversion works, store the number
            Err(_) => continue, // If not, skip this loop and ask again
        };

        println!("You guessed: {}", guess);

        // Compare the user’s guess with the secret number using cmp (compare)
        // cmp returns an Ordering (Less, Greater, or Equal)
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // If guess is less, tell user
            Ordering::Greater => println!("Too Big!"), // If guess is greater, tell user
            Ordering::Equal => {
                println!("You win!"); // If equal, celebrate 🎉
                break; // Break out of the loop. Game over!
            }
        }
    }
}
