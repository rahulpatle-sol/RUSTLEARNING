use std::collections::HashMap;

fn main() {
    /* -------------------------------------------------
       1. Create a HashMap
    -------------------------------------------------- */
    let mut scores: HashMap<String, i32> = HashMap::new();

    /* -------------------------------------------------
       2. Insert key-value pairs
    -------------------------------------------------- */
    scores.insert("Alice".to_string(), 50);
    scores.insert("Bob".to_string(), 40);
    scores.insert("Charlie".to_string(), 60);

    println!("Initial map: {:?}", scores);

    /* -------------------------------------------------
       3. Access values using get()
    -------------------------------------------------- */
    match scores.get("Alice") {
        Some(score) => println!("Alice's score: {}", score),
        None => println!("Alice not found"),
    }

    /* -------------------------------------------------
       4. Check if a key exists
    -------------------------------------------------- */
    if scores.contains_key("Bob") {
        println!("Bob exists in the map");
    }

    /* -------------------------------------------------
       5. Update value using get_mut()
    -------------------------------------------------- */
    if let Some(score) = scores.get_mut("Bob") {
        *score += 10;
    }

    println!("After updating Bob: {:?}", scores);

    /* -------------------------------------------------
       6. Entry API (insert if missing)
    -------------------------------------------------- */
    scores.entry("David".to_string()).or_insert(45);

    /* -------------------------------------------------
       7. Entry API (modify or insert)
    -------------------------------------------------- */
    scores
        .entry("Alice".to_string())
        .and_modify(|v| *v += 5)
        .or_insert(50);

    println!("After entry API usage: {:?}", scores);

    /* -------------------------------------------------
       8. Iterate over key-value pairs
    -------------------------------------------------- */
    println!("Iterating over map:");
    for (name, score) in &scores {
        println!("{} => {}", name, score);
    }

    /* -------------------------------------------------
       9. Iterate and modify values
    -------------------------------------------------- */
    for score in scores.values_mut() {
        *score += 1;
    }

    println!("After incrementing all scores: {:?}", scores);

    /* -------------------------------------------------
       10. Remove a key
    -------------------------------------------------- */
    let removed = scores.remove("Charlie");
    println!("Removed Charlie: {:?}", removed);

    /* -------------------------------------------------
       11. Keys and values separately
    -------------------------------------------------- */
    println!("Keys:");
    for key in scores.keys() {
        println!("{}", key);
    }

    println!("Values:");
    for value in scores.values() {
        println!("{}", value);
    }

    /* -------------------------------------------------
       12. Clear the HashMap
    -------------------------------------------------- */
    scores.clear();
    println!("After clearing map: {:?}", scores);
}
