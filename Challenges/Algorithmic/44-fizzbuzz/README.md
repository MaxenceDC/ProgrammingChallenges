# N°44 - FizzBuzz

What better way to start than with the classic programming challenge, FizzBuzz!
I will be writing the code with Rust, as I am currently learning it. It should
be pretty simple, because I already coded a FizzBuzz program before in some
other languages before.

## Starting the challenge

First, I create a new directory that I name `44-fizzbuzz` inside the
`Challenges/Algorithmic` directory. To create a new Rust project, I use the
`cargo new` command and I give it the name of my project, which is `fizzbuzz`.
I can start writing code in the `main.rs` file, located inside the `src`
directory.

I then document myself on what a FizzBuzz program is, and how it works.
Furthermore, I've already programmed a FizzBuzz program before, but it's always
good to have a description of what the program should do.

According to [this](https://wikipedia.org/wiki/Fizz_Buzz) Wikipedia article,
FizzBuzz is a game for children to learn about counting. The rules are simple:
you start by counting up indefinitely, and if you see a number that is divisible
by 3, you say "Fizz!" instead, when it's divisible by 5, you say "Buzz!",
and when it's divisible by both, say "FizzBuzz!".

For an example, if I count from 1 to 15, I should get:

```text
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
```

The article also states that writing an implementation of this game is widely
asked during programming interviews, to analyze the coding style and habits of
the interviewee. So, let's start with the code!

## Solving the challenge

I start by creating a function named `fizzbuzz` that takes a positive number as
an argument and returns nothing.

```rs
fn fizzbuzz(n: u32) {}
```

Then, I write a simple for loop inside this function, which loops over a range
from 1 to n, included. To specify that I want the n to be included, I use the
`..=` syntax.

```rs
for i in 1..=n {
    // ...
}
```

Inside this for loop, I first check if the current number is not a multiple of
3 or 5. To do that, I use the `%` operator, which returns the remainder of a
division. If the remainder is 0, then the number is a multiple of the number I'm
checking, else, it's not. For example, `9 % 3` returns `0`, so `9` is a multiple
of `3`. In my code, if the number is not a multiple of 3 or 5, it just prints
the said number and continues over to the next iteration, without doing anything
else.

```rs
for i in 1..=n {
    if i % 3 != 0 && i % 5 != 0 {
        println!("{}", i);
        continue;
    }
    // ...
}
```

If the number does not pass the first check, it means that it is a multiple of
3 or 5, if not both. I start by checking if it's a multiple of 3, and if it is,
it prints "Fizz", without adding a new line, using the `print!` macro. If it's
a multiple of 5, it prints "Buzz", also with the `print!` macro. The reason I
use the `print!` macro instead of the `println!` macro is that if the number
is a multiple of both 3 and 5, I want to print "FizzBuzz!", and I don't want to
add an unnecessary if statement which would check if the number is divisible by
both 3 and 5. Finally, it prints an exclamation mark followed by new line.

```rs
for i in 1..=n {
    // ...
    if i % 3 == 0 {
        print!("Fizz");
    }
    if i % 5 == 0 {
        print!("Buzz");
    }
    println!("!");
}
```

And there we go! We have a fully functional FizzBuzz function! We just need to
call it from the main function, and we're done! To run the code, simply execute
the following command: `cargo run`.

### Final code

```rs
fn main() {
    fizzbuzz(100);
}

fn fizzbuzz(n: u32) {
    for i in 1..=n {
        if i % 3 != 0 && i % 5 != 0 {
            println!("{}", i);
            continue;
        }
        if i % 3 == 0 {
            print!("Fizz");
        }
        if i % 5 == 0 {
            print!("Buzz");
        }
        println!("!");
    }
}
```

You can see a commented version of this code inside `main.rs` file located in
the `src` folder.

## Going further : Taking user input

To take user input in Rust, the most basic way is to use the `std::io` module.
So the first line of my code is `use std::io;`, which imports the standard
input/output.

```rs
use std::io;
```

Then, I create a new infinite loop inside the `main` function that I label as
`take_input`. This loop will be used to take user input, until the said input is
a positive integer.

```rs
fn main() {
    'take_input: loop {}
}
```

Inside this loop, I start by creating a mutable variable named `input` that
holds a new empty string. I then ask the user to enter a number and I use the
`io::stdin` function to start taking user input and call the `read_line` method
on it. This method takes a mutable reference to a string, and binds the content
of what the user typed to the given mutable variable. In case there is an error
while reading from the standard input, it calls the except method and exits the
program.

```rs
let mut input = String::new();
println!("Please enter a number: ");
io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line from the stdin");
```

Then, to remove the new line character from the string, I use the `trim` method
which removes any whitespace characters from the beginning and end of the string
and returns a **`&str`**, not a String. Because it returns a `&str`, I need to
use the `let` keyword to
[shadow](https://en.wikipedia.org/wiki/Variable_shadowing) the variable `input`
and assign it to the result of the `trim()` method.

```rs
let input = input.trim();
```

Then, I check if the input is a good number. To do that, I match the result of
the `parse` method on the `input` string. If it's a `Ok`, then it's a good
number, and I can call the `fizzbuzz` function with the number and break the
loop. If it's a `Err`, then it means that the input is invalid and the programs
tells the user to try again.

```rust
match input.parse::<u32>() {
    Ok(i) => {
        fizzbuzz(i);
        break 'take_input;
    }
    Err(..) => println!("Please enter a valid number!"),
};
```

And it's done! The program will now start by asking the user to enter a number,
and will then execute the `fizzbuzz` function with the number!

### Final improved code

```rs
use std::io;

fn main() {
    'take_input: loop {
        let mut input = String::new();
        println!("Please enter a number: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line from the stdin");
        let input = input.trim();
        match input.parse::<u32>() {
            Ok(i) => {
                fizzbuzz(i);
                break 'take_input;
            }
            Err(..) => println!("Please enter a valid number!"),
        };
    }
}

fn fizzbuzz(n: u32) {
    for i in 1..=n {
        if i % 3 != 0 && i % 5 != 0 {
            println!("{}", i);
            continue;
        }
        if i % 3 == 0 {
            print!("Fizz");
        }
        if i % 5 == 0 {
            print!("Buzz");
        }
        println!("!");
    }
}
```
