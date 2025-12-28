// this module contains

pub fn int_interface(){
    //  we have 2 types of integer type 
    // singed //unsinged
     let small: i8 = 127;
    let large: i64 = 807;
    let unsigned: u32 = 95;
    
    // TODO: Create variables for:
    // - A counter that can't be negative
    // - A very large positive number
    // - A number that represents bytes (0-255)
    
    // Operations with different types
    let x: i32 = -100;
    let y: i64 = 1000;
    //  let _sum = x as i64+y;
     let z:u32=123;
     let k:i32=569;
     let _sum=z as i32+k;

     // Fix this
    
    println!("Small: {}, Large: {}, Unsigned: {},sum{}", small, large, unsigned,_sum);

}