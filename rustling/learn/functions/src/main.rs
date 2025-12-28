// ===============================
// 🧠 Rust Functions — Basics to Advanced
// ===============================

fn main() {
    // Calling a basic function with no parameters
    greet();

    // Calling a function with parameters
    print_sum(10, 20);

    // Calling a function that returns a value
    let square = square_of(6);
    println!("Square is: {}", square);

    // Using a function with an explicit return type
    let result = multiply(4, 5);
    println!("Multiplication result: {}", result);

    // Demonstrating expression vs statement
    let doubled = double_then_add_one(7);
    println!("Double + 1: {}", doubled);
}

// ===============================
// 🔹 Basic Function — No parameters, no return
// ===============================
fn greet() {
    println!("Hello from a function!");
}

// ===============================
// 🔹 Function with parameters
// Takes two i32 values and prints their sum
// ===============================
fn print_sum(a: i32, b: i32) {
    println!("Sum is: {}", a + b);
}

// ===============================
// 🔹 Function with return value
// Returns the square of a number
// ===============================
fn square_of(x: i32) -> i32 {
    x * x // No semicolon = expression = return value
}

// ===============================
// 🔹 Function with explicit return and semicolon
// Multiplies two numbers and returns the result
// ===============================
fn multiply(a: i32, b: i32) -> i32 {
    return a * b; // `return` keyword used explicitly
}

// ===============================
// 🔹 Expression vs Statement
// Demonstrates how expressions return values
// ===============================
fn double_then_add_one(n: i32) -> i32 {
    let doubled = {
        let temp = n * 2;
        temp // This block is an expression
    };
    doubled + 1
}
