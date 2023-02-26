// import the basic types and functions for working with inputs in rust
use std::io;

// start the rust program. this is a mandatory entry point
fn main() {
    println!("Rust Lesson 1 Calculator");

    // read the first number from the console. rs version of js 'console.log()'
    println!("Enter the first number:");
    
    let mut num1 = String::new();  // declare a new mutable variable named 'num1' and initialize it with a new empty string, which is used to store the first number
    // use standard function for working with input
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");
    let num1: f64 = num1.trim().parse().expect("Invalid input");
    // `read_line()` method reads a line of text from the standard input (console) and stores it in the `num1` variable
    // `expect()` method is used to handle errors that occur while reading input
    // `trim()` method removes any leading or trailing whitespace from the input string
    // `parse()` method parses the input string as a floating-point number (`f64`)
    // `expect()` method is used to handle errors that occur from parsing


    // read the second number from the console
    // Use the same process as 'num1'. 
    println!("Enter the second number:");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");
    let num2: f64 = num2.trim().parse().expect("Invalid input");

    // read the operator from the console
    // use the same process until 'let operator:' where you replace 'parse' with 'chars' and 'next'
    println!("Enter the operator (+, -, *, /):");
    let mut operator = String::new();
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");
    let operator: char = operator.trim().chars().next().expect("Invalid input");
    // 'chars' creates an iterator over the characters in the string 
    // 'next' throws to an error is there is no characters in the operator


    // perform the operation and print the result
    // basic math functions similar to js result math
    let result = match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        _ => panic!("Invalid operator"),
    };

    // print result similar to exporting a function at the end of a js file
    println!("Result: {}", result);
}
