


//  this is  proj  filr 

pub fn borrow_checker(){
    let mut input = String::from("Hello");
    mut_borrow(&mut input);
    println!("{}", input);
}

//  2 proj
// mutable immutable borrow checker
// 

pub fn immutable_check(){
    let input = String::from("Hello");
    let output = str_return(&input);
    println!("{}", output);
}
// 3 proj 