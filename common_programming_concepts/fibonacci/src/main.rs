use std::io;

fn main() {
    println!("Fibonacci Generator♺");

    // Ask the user for the position they want
    println!("Enter the position (n) in the Fibonacci sequence:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Convert the user's input (String) to a number
    let n: u32 = input.trim().parse().expect("Please enter a valid number!");

    // For 0 or 1 we can return the value directly
    if n == 0 {
        println!("The 0th Fibonacci number is 0");
        return;
    } else if n == 1 {
        println!("The 1st Fibonacci number is 1");
        return;
    }

    // Start with the first two Fibonacci numbers
    let mut first = 0;
    let mut second = 1;

    // Variable to store the next number in the sequence
    let mut next;

    // Loop to calculate Fibonacci number up to n
    for _ in 2..=n {
        next = first + second;
        first = second;
        second = next;
    }

    println!("The {}th Fibonacci number is {}", n, second);
}
