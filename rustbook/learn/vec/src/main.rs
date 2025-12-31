// ===============================
// 📦 Rust Vectors — Growable Arrays
// ===============================

fn main() {
    // ===============================
    // 🔹 Declaring a vector
    // Vectors are dynamic, unlike fixed-size arrays
    // ===============================
    let numbers = vec![10, 20, 30];
    println!("First number: {}", numbers[0]);

    // ===============================
    // 🔹 Creating an empty vector
    // You can push values later
    // ===============================
    let mut scores: Vec<i32> = Vec::new();
    scores.push(95);
    scores.push(88);
    scores.push(76);
    println!("Scores: {:?}", scores);

    // ===============================
    // 🔹 Iterating over a vector
    // ===============================
    for score in &scores {
        println!("Score: {}", score);
    }

    // ===============================
    // 🔹 Mutable iteration
    // You can modify values using mutable reference
    // ===============================
    for score in &mut scores {
        *score += 5; // Dereference and mutate
    }
    println!("Boosted scores: {:?}", scores);

    // ===============================
    // 🔹 Removing elements
    // ===============================
    scores.pop(); // Removes last element
    println!("After pop: {:?}", scores);

    // ===============================
    // 🔹 Accessing safely with get()
    // Returns Option<T>
    // ===============================
    match scores.get(1) {
        Some(val) => println!("Second score: {}", val),
        None => println!("No value at index 1"),
    }

    // ===============================
    // 🔹 Vector length
    // ===============================
    println!("Length: {}", scores.len());

    // ===============================
    // 🔹 Comparing with arrays
    // Arrays: fixed size, stack-allocated
    // Vectors: dynamic size, heap-allocated
    // ===============================
    let arr = [1, 2, 3];
    let vec = vec![1, 2, 3];
    println!("Array: {:?}, Vector: {:?}", arr, vec);
}
