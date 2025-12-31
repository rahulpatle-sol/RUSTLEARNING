// ===============================
// 🧮 Rust Arrays — Fixed-size Collections
// ===============================

fn main() {
    // ===============================
    // 🔹 Declaring an array
    // Fixed-size, same type elements
    // ===============================
    let numbers: [i32; 5] = [10, 20, 30, 40, 50];
    println!("First number: {}", numbers[0]); // Access by index

    // ===============================
    // 🔹 Type inference
    // Rust infers [i32; 3]
    // ===============================
    let scores = [95, 88, 76];
    println!("Second score: {}", scores[1]);

    // ===============================
    // 🔹 Repeating values
    // [value; count] syntax
    // ===============================
    let zeros = [0; 4]; // [0, 0, 0, 0]
    println!("Zeros: {:?}", zeros); // {:?} prints entire array

    // ===============================
    // 🔹 Iterating over an array
    // ===============================
    for num in numbers.iter() {
        println!("Number: {}", num);
    }

    // ===============================
    // 🔹 Mutable array
    // You can change values if array is mutable
    // ===============================
    let mut flags = [true, false, false];
    flags[1] = true;
    println!("Flags: {:?}", flags);

    // ===============================
    // 🔹 Destructuring arrays
    // ===============================
    let [a, b, c, d, e] = numbers;
    println!("Destructured: {}, {}", a, e);

    // ===============================
    // 🔹 Out-of-bounds panic
    // Uncommenting below will crash at runtime
    // ===============================
    // println!("{}", numbers[10]); // Index out of bounds
}
