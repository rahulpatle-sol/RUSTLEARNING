// ===============================
// 📦 Rust Data Types Overview
// ===============================

fn main() {
    // ===============================
    // 🔢 Integer Types
    // ===============================

    // Signed integers (can be negative)
    let a: i8 = -128;      // 8-bit signed, range: -128 to 127
    let b: i16 = -32_000;  // 16-bit signed
    let c: i32 = -2_000_000_000; // 32-bit signed (default type for integers)
    let d: i64 = -9_000_000_000_000_000_000; // 64-bit signed
    let e: i128 = -1_000_000_000_000_000_000_000_000_000_000_000; // 128-bit signed
    let f: isize = -500;   // Signed integer, size depends on system architecture (32-bit or 64-bit)

    // Unsigned integers (only positive)
    let g: u8 = 255;       // 8-bit unsigned, range: 0 to 255
    let h: u16 = 65_000;   // 16-bit unsigned
    let i: u32 = 4_000_000_000; // 32-bit unsigned
    let j: u64 = 18_000_000_000_000_000_000; // 64-bit unsigned
    let k: u128 = 3_000_000_000_000_000_000_000_000_000_000_000; // 128-bit unsigned
    let l: usize = 1000;   // Unsigned, system-dependent size

    println!("Signed i32: {}", c);
    println!("Unsigned u8: {}", g);

    // ===============================
    // 🔤 Character Type
    // ===============================
    let ch: char = 'R'; // Single Unicode character
    println!("Character: {}", ch);

    // ===============================
    // 📄 Boolean Type
    // ===============================
    let is_active: bool = true;
    let is_admin: bool = false;
    println!("Is active? {}", is_active);

    // ===============================
    // 🧮 Floating Point Types
    // ===============================
    let x: f32 = 3.14;     // 32-bit float
    let y: f64 = 2.718281828459045; // 64-bit float (default for decimals)
    println!("Float f64: {}", y);

    // ===============================
    // 📦 Compound Types
    // ===============================

    // Tuple — fixed-size, mixed types
    let person: (&str, i32, bool) = ("Rahul", 30, true);
    println!("Tuple name: {}", person.0);

    // Array — fixed-size, same type
    let scores: [i32; 3] = [95, 88, 76];
    println!("First score: {}", scores[0]);

    // ===============================
    // 🧠 Type Inference
    // ===============================
    let inferred = 42; // Rust infers this as i32
    println!("Inferred type: {}", inferred);
}
