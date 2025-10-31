// this module contains

pub fn int_interface(){
    //  we have 2 types of integer type 
    // singed //unsinged
     let small: i8 = 127;
    let large: i64 = 9_223_372_036_854_775_807;
    let unsigned: u32 = 4_294_967_295;
    
    // TODO: Create variables for:
    // - A counter that can't be negative
    // - A very large positive number
    // - A number that represents bytes (0-255)
    
    // Operations with different types
    let x: i32 = 100;
    let y: i64 = 1000;
    // let sum = x + y; // Fix this
    
    println!("Small: {}, Large: {}, Unsigned: {}", small, large, unsigned);

}