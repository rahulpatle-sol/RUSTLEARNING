// ===============================
// 🧬 Rust Traits — Behavior Contracts
// ===============================

fn main() {
    // 🔹 Using trait objects (dynamic dispatch)
    let dog = Dog;
    let cat = Cat;

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(dog), Box::new(cat)];

    for a in animals {
        a.speak(); // Dynamic dispatch
    }

    // 🔹 Trait bound in function
    let person = Person { name: String::from("Rahul") };
    greet(&person); // Works because Person implements Greet
}

// ============================================
// 🔸 Trait definition — like an interface
// ============================================
trait Animal {
    fn speak(&self); // Required method

    // Default method
    fn sleep(&self) {
        println!("Zzz...");
    }
}

// ============================================
// 🔸 Implementing trait for a type
// ============================================
struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Dog says: Woof!");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("Cat says: Meow!");
    }
}

// ============================================
// 🔸 Trait with method and trait bound
// ============================================
trait Greet {
    fn greet(&self) -> String;
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

// 🔸 Function with trait bound
fn greet<T: Greet>(item: &T) {
    println!("{}", item.greet());
}
