// ===============================
// 🔀 Rust Control Flow — if, else, match
// ===============================

fn main() {
    // ===============================
    // 🔹 if / else if / else
    // ===============================
    let score = 85;

    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else {
        println!("Grade: F");
    }

    // ===============================
    // 🔹 if as an expression
    // ===============================
    let is_pass = if score >= 50 { true } else { false };
    println!("Passed? {}", is_pass);

    // ===============================
    // 🔹 match — powerful pattern matching
    // ===============================
    let day = 3;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 | 7 => println!("Weekend"), // multiple patterns
        _ => println!("Invalid day"), // catch-all
    }

    // ===============================
    // 🔹 match with return value
    // ===============================
    let grade = match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        0..=69 => "F",
        _ => "Invalid",
    };
    println!("Match Grade: {}", grade);

    // ===============================
    // 🔹 match with tuple destructuring
    // ===============================
    let point = (0, 5);

    match point {
        (0, y) => println!("On Y axis at {}", y),
        (x, 0) => println!("On X axis at {}", x),
        (x, y) => println!("Point at ({}, {})", x, y),
    }

    // ===============================
    // 🔹 if let — concise pattern match
    // ===============================
    let some_number = Some(7);

    if let Some(n) = some_number {
        println!("Number is: {}", n);
    }

    // ===============================
    // 🔹 while let — loop with pattern match
    // ===============================
    let mut optional = Some(3);

    while let Some(n) = optional {
        println!("Looping with: {}", n);
        optional = if n > 0 { Some(n - 1) } else { None };
    }
}
