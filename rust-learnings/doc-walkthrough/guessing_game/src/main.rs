// rust has a set of items defined in the stdlib that is available
// by default in the scope of each program, this set is called the prelude

// the std io is not available in the program scope by default so import it using the use statement
use rand::Rng;
use std::io; // standard libary, same as #include <std> // the Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use these methods

use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // in rust variables are immutable my default, so use the mut keyword
        // new() is an associated (member) function of String type
        let mut guess = String::new(); // intialize mutable string

        // thread_rng function will give us the random number generator
        // that is local to the current thread of execution and one that
        // is seeded by the operating system
        // gen_range takes a range expression (1..=10) as an argument
        // and generates a random number in the range
        // value defaults to i32 unless specified

        // we could also write this as std::io::stdin (same as std::cout)
        // pass by reference using the ampersand operator (&mut)
        // readline also returns a Result value which is an enum
        // Result 's variants are Ok (operation successful) and Err (operation failed)
        // if the instance of Result is an Err, then the expect() method causes the program to crash with the message shown in the command line
        // compiler will throw you a warning if you don't operate on the Result value
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // shadowing - reuse the `guess` variable name rather than
        // creating a new variable - will resurface in chapter 3
        // parse method will only work on characters that can be logically
        // converted into numbers and so can easily cause errors
        // so it also returns a Result enum which can be evaluated with an expect()

        // pattern match the Result of parse(), return num if it is parsed correctly, else continue with the loop
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // ignore error information by using _
        };

        // no need to define data type for print, macro handles it I guess?
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // exit the loop when the answer is correct
            }
        }
    }
    // when printing the value of the variable, place it inside curly braces
    // if you're printing the value of an expression, use empty braces in format string as shown above
    // let x = 5;
    // let y = 10;
    // println!("x = {x} and y + 2 = {}", y + 2);
}
