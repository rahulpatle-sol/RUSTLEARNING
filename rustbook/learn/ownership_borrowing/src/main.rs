fn main() {
    // 🔹 Ownership transfer (move)
    let s1 = String::from("Rahul");
    let s2 = s1; // s1 is moved to s2 — s1 is invalid now
    // println!("{}", s1); // ❌ Error: s1 no longer owns the value
    println!("s2 owns: {}", s2); // ✅ OK

    // 🔹 Borrowing (immutable reference)
    let name = String::from("Founder");
    print_name(&name); // Borrowed — no ownership transfer
    println!("Still valid: {}", name); // ✅ OK

    // 🔹 Borrowing (mutable reference)
    let mut role = String::from("Dev");
    update_role(&mut role); // Mutable borrow — allows change
    println!("Updated role: {}", role); // ✅ OK

    // 🔹 Multiple immutable borrows — allowed
    let a = &name;
    let b = &name;
    println!("a: {}, b: {}", a, b); // ✅ OK

    // 🔹 Mutable borrow must be exclusive
    // let c = &mut name;
    // println!("a: {}", a); // ❌ Error: can't use a while c is active
}

// 🔸 Function that borrows immutably
fn print_name(n: &String) {
    println!("Name: {}", n);
}

// 🔸 Function that borrows mutably
fn update_role(r: &mut String) {
    r.push_str(" Lead");
}
