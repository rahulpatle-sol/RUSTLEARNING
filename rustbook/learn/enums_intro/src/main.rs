// ===============================
// 🎯 Rust Enums — Option & Result
// ===============================

fn main() {
    // ============================================
    // 🔹 Option<T> — for values that might be missing
    // ============================================
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;

    // Pattern matching on Option
    match some_number {
        Some(n) => println!("Got a number: {}", n),
        None => println!("No number found"),
    }

    // Using unwrap (⚠️ panics if None)
    // println!("Unwrapped: {}", no_number.unwrap()); // ❌ Will panic

    // Safe access with unwrap_or
    println!("Safe unwrap: {}", no_number.unwrap_or(0)); // ✅ OK

    // ============================================
    // 🔹 Result<T, E> — for success or error
    // Used in file I/O, parsing, etc.
// ============================================
    let parsed = parse_number("123");
    match parsed {
        Ok(n) => println!("Parsed number: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    let failed = parse_number("abc");
    println!("Fallback: {}", failed.unwrap_or(-1)); // ✅ OK

    // ============================================
    // 🔹 Custom enum example
    // ============================================
    let status = Status::Online;
    status.print();

    let error = Status::Error(String::from("Connection lost"));
    error.print();
}

// 🔸 Function returning Result
fn parse_number(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(String::from("Invalid number")),
    }
}

// 🔸 Custom enum
enum Status {
    Online,
    Offline,
    Error(String),
}

impl Status {
    fn print(&self) {
        match self {
            Status::Online => println!("Status: Online"),
            Status::Offline => println!("Status: Offline"),
            Status::Error(msg) => println!("Error: {}", msg),
        }
    }
}
