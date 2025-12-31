// ===============================
// 🔁 Rust Loops — for, while, loop
// ===============================

fn main() {
    // ===============================
    // 🔹 loop — Infinite loop (until break)
    // ===============================
    let mut count = 0;
    loop {
        println!("loop count: {}", count);
        count += 1;

        // Exit condition
        if count == 3 {
            break; // Stops the loop
        }
    }

    // ===============================
    // 🔹 while — Conditional loop
    // ===============================
    let mut num = 0;
    while num < 3 {
        println!("while num: {}", num);
        num += 1;
    }

    // ===============================
    // 🔹 for — Iterator-based loop
    // ===============================
    let scores = [85, 90, 78];
    for score in scores.iter() {
        println!("Score: {}", score);
    }

    // ===============================
    // 🔹 for with range
    // ===============================
    for i in 1..4 {
        // Range is exclusive of the upper bound (4)
        println!("Range value: {}", i);
    }

    // ===============================
    // 🔹 for with reverse
    // ===============================
    for i in (1..4).rev() {
        println!("Reverse range: {}", i);
    }

    // ===============================
    // 🔹 Nested loops
    // ===============================
    for x in 1..3 {
        for y in 1..3 {
            println!("x: {}, y: {}", x, y);
        }
    }

    // ===============================
    // 🔹 Loop with continue
    // ===============================
    for i in 0..5 {
        if i == 2 {
            continue; // Skip this iteration
        }
        println!("Continue loop: {}", i);
    }
}
