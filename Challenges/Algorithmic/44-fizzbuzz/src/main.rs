use std::io;

fn main() {
  // Creates a loop labeled take_input, which will run until the input is a
  // valid integer
  'take_input: loop {
    // Declares the new variable input and prompts the user for an input
    println!("Please enter a number: ");
    let mut input = String::new();
    // Reads the input and stores it in the mutable input variable
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line from the stdin");
    // Redeclares the input variable as a str and trims it
    let input = input.trim();
    // Matches the input as a positive integer.
    // If the input is valid, the fizzbuzz function is called and the loop
    // labeled take_input breaks
    // Otherwise, it will print an error message and continue the loop to prompt
    // the user for a new input
    match input.parse::<u32>() {
      Ok(i) => {
        fizzbuzz(i);
        break 'take_input;
      }
      Err(..) => println!("Please enter a valid number!"),
    };
  }
}

/// Prints out all the numbers from 1 to n (inclusive)
///
/// If the number is divisible by 3, prints "Fizz" instead
///
/// If the number is divisible by 5, prints "Buzz" instead
///  
/// If the number is divisible by both 3 and 5, prints "FizzBuzz".  
///
/// # Arguments
/// + `n` - The number to count up to
fn fizzbuzz(n: u32) {
  // Goes through all the numbers from 1 to n (inclusive)
  for i in 1..=n {
    // If the number is not divisible by 3 or 5, prints
    // the number and moves on to the next iteration
    if i % 3 != 0 && i % 5 != 0 {
      println!("{}", i);
      continue;
    }
    // Checks if the number is divisible by 3
    // If so, prints "Fizz" without a new line
    if i % 3 == 0 {
      print!("Fizz");
    }
    // Checks if the number is divisible by 5
    // If so, prints "Buzz" without a new line
    if i % 5 == 0 {
      print!("Buzz");
    }
    // Prints an exclamation mark followed by a new line
    println!("!");
  }
}
