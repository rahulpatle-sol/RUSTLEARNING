//  hello  here 


pub fn  chart_types(){
     let letter = 'A';
    let emoji = '🦀';
    let heart = '💖';
    
    // String types
    let greeting = "Hello, Rust!"; // string slice (&str)
    let mut message = String::from("Learning"); // String type
    
    // TODO: Add " Rust!" to the message
    // message ???;
    message.push_str("rust  🦀");

    
    // Character operations
    let first_char = greeting.chars().next().unwrap();
    let last_char = greeting.chars().last().unwrap();
    
    println!("Letters: {}, {}, {}", letter, emoji, heart);
    println!("Greeting: {}", greeting);
    println!("Message: {}", message);
    println!("First char: {}, Last char: {}", first_char, last_char);
}