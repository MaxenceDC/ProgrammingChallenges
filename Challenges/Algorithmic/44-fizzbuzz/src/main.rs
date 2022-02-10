fn main() {
    fizzbuzz(150);
}

/// Prints out all the numbers from 1 to n (inclusive)
/// If the number is divisible by 3, prints "Fizz" instead
/// If the number is divisible by 5, prints "Buzz" instead
/// If the number is divisible by both 3 and 5, prints "FizzBuzz".
///
/// # Arguments
/// * `n` - The number to count up to
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
