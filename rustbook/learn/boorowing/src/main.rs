// ===============================
// 🔗 Rust Borrowing & References — Full Concept
// ===============================

fn main() {
    // ============================================
    // 🔹 Immutable Borrowing — read-only access
    // ============================================
    let name = String::from("Rahul");
    print_name(&name); // Borrowed, not moved
    println!("Still valid: {}", name); // ✅ OK

    // ============================================
    // 🔹 Mutable Borrowing — exclusive write access
    // ============================================
    let mut role = String::from("Dev");
    update_role(&mut role); // Mutably borrowed
    println!("Updated role: {}", role); // ✅ OK

    // ============================================
    // 🔹 Reference Rules
    // 1. You can have multiple immutable references
    // 2. You can have only one mutable reference
    // 3. You can't mix mutable and immutable at the same time
    // ============================================
    let title = String::from("Founder");
    let a = &title;
    let b = &title;
    println!("a: {}, b: {}", a, b); // ✅ OK

    // let c = &mut title; // ❌ Error: can't borrow mutably while a/b are active

    // ============================================
    // 🔹 Referencing & Dereferencing
    // ============================================
    let x = 10;
    let r = &x; // r is a reference to x
    println!("Reference: {}", r); // ✅ OK

    let y = *r; // Dereferencing — gets the value behind the reference
    println!("Dereferenced: {}", y);

    // ============================================
    // 🔹 Auto Dereferencing
    // Rust automatically dereferences when calling methods
    // ============================================
    let s = String::from("Rust");
    let len = s.len(); // s is a reference to heap data — auto deref to call len()
    println!("Length: {}", len);

    // ============================================
    // 🔹 Dangling Reference — Rust prevents this
    // ============================================
    // fn dangle() -> &String {
    //     let s = String::from("Oops");
    //     &s // ❌ Error: s will be dropped, reference becomes invalid
    // }

    // ✅ Correct version
    fn no_dangle() -> String {
        let s = String::from("Safe");
        s // Ownership is returned
    }

    let safe = no_dangle();
    println!("No dangle: {}", safe);
}

// 🔸 Function with immutable borrow
fn print_name(n: &String) {
    println!("Name: {}", n);
}

// 🔸 Function with mutable borrow
fn update_role(r: &mut String) {
    r.push_str(" Lead");
}
