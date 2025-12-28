use std::io;

#[derive(Debug)]
struct Task {
    title: String,
    done: bool,
}

impl Task {
    fn new(title: &str) -> Self {
        Self {
            title: String::from(title),
            done: false,
        }
    }

    fn mark_done(&mut self) {
        self.done = true;
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\n📋 Todo Menu:");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Mark Task Done");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read title");
                tasks.push(Task::new(title.trim()));
                println!("✅ Task added.");
            }
            "2" => {
                println!("\n📝 Your Tasks:");
                for (i, task) in tasks.iter().enumerate() {
                    let status = if task.done { "✅" } else { "❌" };
                    println!("{}: {} [{}]", i + 1, task.title, status);
                }
            }
            "3" => {
                println!("Enter task number to mark done:");
                let mut num = String::new();
                io::stdin().read_line(&mut num).expect("Failed to read number");
                if let Ok(index) = num.trim().parse::<usize>() {
                    if index > 0 && index <= tasks.len() {
                        tasks[index - 1].mark_done();
                        println!("🎯 Task marked as done.");
                    } else {
                        println!("❌ Invalid task number.");
                    }
                } else {
                    println!("❌ Please enter a valid number.");
                }
            }
            "4" => {
                println!("👋 Exiting Todo App. See you!");
                break;
            }
            _ => println!("❌ Invalid choice. Try again."),
        }
    }
}