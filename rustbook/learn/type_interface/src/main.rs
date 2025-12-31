// ===============================
// 🧠 Rust Type Inference — Let the Compiler Decide
// ===============================

fn main() {
    // ============================================
    // 🔹 Basic Inference — Compiler figures out the type
    // ============================================
    let x = 42; // inferred as i32
    let pi = 3.14; // inferred as f64
    let name = "Rahul"; // inferred as &str
    let is_active = true; // inferred as bool

    println!("x: {}, pi: {}, name: {}, active: {}", x, pi, name, is_active);

    // ============================================
    // 🔹 Inference with collections
    // ============================================
    let scores = vec![95, 88, 76]; // inferred as Vec<i32>
    println!("Scores: {:?}", scores);

    // ============================================
    // 🔹 Inference with function return
    // ============================================
    let result = add(5, 7); // inferred as i32
    println!("Sum: {}", result);

    // ============================================
    // 🔹 Explicit typing — when needed
    // Useful for clarity or when inference fails
    // ============================================
    let count: u32 = 100;
    let ratio: f32 = 0.75;
    println!("Count: {}, Ratio: {}", count, ratio);

    // ============================================
    // 🔹 Inference fails — must specify type
    // ============================================
    // let empty = Vec::new(); // ❌ Error: compiler can't infer type
    let empty: Vec<String> = Vec::new(); // ✅ OK
    println!("Empty vector: {:?}", empty);
}

// 🔸 Function with inferred return type
fn add(a: i32, b: i32) -> i32 {
    a + b
}
