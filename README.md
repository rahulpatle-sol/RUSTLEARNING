# 🦀 Rust Learning Journey

This repository documents my complete journey learning the Rust programming language.  
It includes **projects from the Rust Book** and **exercises from Rustlings**, organized for clarity and practice.

---

## 📚 Curriculum Overview

Rust is a systems programming language focused on **safety, speed, and concurrency**.  
Below is the chapter-by-chapter breakdown of the Rust Book, with notes and exercises aligned to each concept.

| Chapter | Topic | What I Learned |
|---------|-------|----------------|
| 01 | Getting Started | How to install Rust, use `cargo`, and write a simple "Hello, world!" program. |
| 02 | Guessing Game *(Project)* | Built a CLI number guessing game using `rand`, loops, and input handling. |
| 03 | Common Programming Concepts | Variables, mutability, data types, functions, control flow (`if`, `loop`, `match`). |
| 04 | Ownership | Core Rust concept: how values are owned, borrowed, and moved. Learned about references and slices. |
| 05 | Structs | Defined custom data types with `struct`, implemented methods with `impl`. |
| 06 | Enums & Pattern Matching | Created enums, used `match` for control flow, and handled multiple states cleanly. |
| 07 | Packages, Crates, Modules | Organized code into modules, understood visibility (`pub`), and managed dependencies. |
| 08 | Collections | Practiced with `Vec`, `String`, and `HashMap`. Learned about borrowing rules with collections. |
| 09 | Error Handling | Used `panic!`, `Result`, and error propagation with `?`. |
| 10 | Generics, Traits, Lifetimes | Wrote generic functions, implemented traits, and understood lifetimes for references. |
| 11 | Testing | Created unit tests and integration tests with `cargo test`. |
| 12 | Minigrep *(Project)* | Built a CLI text search tool, handled command-line arguments, and added case-insensitive search. |
| 13 | Iterators & Closures | Learned functional style with iterators, closures, and lazy evaluation. |
| 14 | Cargo & Crates.io | Explored workspaces, publishing crates, and release profiles. |
| 15 | Smart Pointers | Used `Box`, `Rc`, `RefCell`, and understood `Deref` and `Drop`. |
| 16 | Concurrency | Spawned threads, used channels for message passing, and handled shared state safely. |
| 17 | Object-Oriented Rust | Implemented trait objects, dynamic dispatch, and OOP-like patterns. |
| 18 | Patterns & Matching | Advanced pattern matching with destructuring, guards, and wildcards. |
| 19 | Advanced Features | Explored `unsafe`, advanced traits, and macros. |
| 20 | Multithreaded Web Server *(Final Project)* | Built a basic HTTP server with concurrency and graceful shutdown. |

---

## 🛠 Projects Included

- **Guessing Game** (Chapter 2)  
- **Minigrep** (Chapter 12)  
- **Multithreaded Web Server** (Chapter 20)  

Each project is inside `rust-book-projects/` with commented code and notes.

---

## 🦀 Rustlings Exercises

Rustlings is a set of small exercises that reinforce syntax and concepts.  
Topics covered:

- Variables, Functions, Control Flow
- Ownership, Borrowing, References
- Structs, Enums, Strings, Vectors, HashMaps
- Error Handling (`Option`, `Result`)
- Traits, Generics, Lifetimes
- Testing
- Iterators and Closures
- Modules and Packages

All exercises are completed and included as a **submodule** in this repo.

---

## 🚀 How to Run

Clone the repo:

```bash
git clone https://github.com/rahulpatle-sol/rust-learning.git
cd rust-learning
