use std::io;

// Simple temperature converter
// Convert Celsius to Fahrenheit or Fahrenheit to Celsius
//
// Cooking Analogy:
// Think of temperature like adjusting your oven/gas settings
// for different recipes. You must know whether you're converting from
// "how hot you pot feels" (Celsius) or "how hot the oven feels" (Fahrenheit).

fn main() {
    println!("Temperature converter - Celsius <-> Fahrenheit");
    println!("Type 'C' to convert Celsius to Fahrenheit, or 'F' for Fahrenheit to Celsius");

    // Ask the user which conversion they want
    let mode = loop {
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice = choice.trim().to_uppercase();

        if choice == "C" || choice == "F" {
            break choice;
        } else {
            println!("Please type only 'C' or 'F'. Example: C");
        }
    };

    // Ask for temperature value
    println!("Enter the temperature (Example: 37.5):");
    let temp: f64 = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        match input.parse::<f64>() {
            Ok(val) => break val,
            Err(_) => {
                println!("Please enter a valid number like 36.6 or 100");
                continue;
            }
        }
    };

    // Convert depending on the mode
    if mode == "C" {
        // Celcius -> Fahrenheit: F = (C * 9/5) + 32
        let f = (temp * 9.0 / 5.0) + 32.0;
        println!("{:.2} °C = {:.2} °F", temp, f);
    } else {
        // Fahrenheit -> Celsius: C = (F - 32) * 5/9
        let c = (temp - 32.0) * 5.0 / 9.0;
        println!("{:.2} °F = {:.2} °C", temp, c);
    }
}
