// ===============================
// 🧱 Rust Structs + Ownership + impl
// ===============================

fn main() {
    // 🔹 Creating a struct instance
    let user1 = User::new("rahul_dev", "rahul@vbizgro.com");
    user1.print(); // Method call via &self

    // 🔹 Ownership transfer
    let user2 = user1; // Ownership of user1 moves to user2
    // user1.print(); // ❌ Error: user1 is moved
    user2.print(); // ✅ OK

    // 🔹 Mutable struct and method
    let mut user3 = User::new("rahul_ops", "ops@vbizgro.com");
    user3.update_email("new@vbizgro.com"); // &mut self method
    user3.print();

    // 🔹 Associated function — no self
    let default_user = User::default();
    default_user.print();
}

// ============================================
// 🔸 Struct definition
// ============================================
struct User {
    username: String,
    email: String,
    active: bool,
}

// ============================================
// 🔸 impl block — methods and associated functions
// ============================================
impl User {
    // 🔹 Associated function — no self
    fn new(username: &str, email: &str) -> Self {
        Self {
            username: String::from(username),
            email: String::from(email),
            active: true,
        }
    }

    // 🔹 Method — takes &self
    fn print(&self) {
        println!("User: {} | Email: {} | Active: {}", self.username, self.email, self.active);
    }

    // 🔹 Method — takes &mut self
    fn update_email(&mut self, new_email: &str) {
        self.email = String::from(new_email);
    }

    // 🔹 Associated function — returns default user
    fn default() -> Self {
        Self {
            username: String::from("guest"),
            email: String::from("guest@example.com"),
            active: false,
        }
    }
}
