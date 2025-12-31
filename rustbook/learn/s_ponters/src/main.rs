use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex};
use std::thread;

/* -------------------------------------------------
   Box<T> – Heap allocation
-------------------------------------------------- */
fn box_example() {
    let boxed = Box::new(100);
    println!("Box value: {}", boxed);
}

/* -------------------------------------------------
   Rc<T> – Multiple ownership (single-thread)
-------------------------------------------------- */
fn rc_example() {
    let data = Rc::new(String::from("Rust"));

    let owner1 = Rc::clone(&data);
    let owner2 = Rc::clone(&data);

    println!("Rc value: {}", data);
    println!("Reference count: {}", Rc::strong_count(&data));

    drop(owner1);
    println!("After drop, count: {}", Rc::strong_count(&data));

    let _ = owner2;
}

/* -------------------------------------------------
   RefCell<T> – Interior mutability
-------------------------------------------------- */
fn refcell_example() {
    let value = RefCell::new(10);

    *value.borrow_mut() += 5;
    println!("RefCell value: {}", value.borrow());

    // Runtime panic if rules violated
    // let a = value.borrow();
    // let b = value.borrow_mut(); // PANIC
}

/* -------------------------------------------------
   Rc<RefCell<T>> – Shared + Mutable (VERY COMMON)
-------------------------------------------------- */
fn rc_refcell_example() {
    let shared = Rc::new(RefCell::new(0));

    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);

    *a.borrow_mut() += 1;
    *b.borrow_mut() += 2;

    println!("Rc<RefCell> value: {}", shared.borrow());
}

/* -------------------------------------------------
   Weak<T> – Prevent reference cycles
-------------------------------------------------- */
fn weak_example() {
    let strong = Rc::new(100);
    let weak: Weak<i32> = Rc::downgrade(&strong);

    println!("Strong count: {}", Rc::strong_count(&strong));
    println!("Weak count: {}", Rc::weak_count(&strong));

    if let Some(value) = weak.upgrade() {
        println!("Upgraded weak value: {}", value);
    }
}

/* -------------------------------------------------
   Arc<T> – Thread-safe shared ownership
-------------------------------------------------- */
fn arc_example() {
    let data = Arc::new(5);

    let handles: Vec<_> = (0..3)
        .map(|_| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                println!("Thread value: {}", data);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

/* -------------------------------------------------
   Arc<Mutex<T>> – Thread-safe shared mutability
-------------------------------------------------- */
fn arc_mutex_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}

fn main() {
    box_example();
    rc_example();
    refcell_example();
    rc_refcell_example();
    weak_example();
    arc_example();
    arc_mutex_example();
}
