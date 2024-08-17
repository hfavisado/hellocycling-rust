use std::env;
use std::io;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Determine the number of hours
    let hours = if args.len() > 1 {
        // Try to parse the first command-line argument as hours
        match args[1].trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid input in command-line argument. Please enter a valid number.");
                return;
            }
        }
    } else {
        // Prompt the user for input
        println!("Enter the number of hours:");

        // Create a mutable String to store user input
        let mut input = String::new();

        // Read input from the standard input
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Convert input to a floating point number
        match input.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid input. Please enter a valid number.");
                return;
            }
        }
    };

    // Convert hours to minutes
    let minutes = hours * 60.0;

    // Calculate the cost based on the number of minutes
    let cost = if minutes <= 30.0 {
        130.0
    } else {
        let additional_minutes = minutes - 30.0;
        let intervals = (additional_minutes / 15.0).ceil(); // Number of 15-minute intervals
        130.0 + 100.0 * intervals
    };

    // Print the result
    println!("The cost is: {:.2}", cost);
}

