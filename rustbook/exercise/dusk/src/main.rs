fn main() {
    println!("Hello, world!");
    let input = String::from("Hello");
    let output = str_return(&input);
    println!("{}", output);
    
    
}
fn  str_return(input:&String)->&str{
    input.as_str()
    
}
//  2
// immutable and mutable borrow


fn mut_borrow(input:&mut String){
    
    //  imutable borrow
     input.push_str(" World");
}


fn mut_borrow_fix(input:&mut String){
    
    //  imutable borrow
     input.push_str(" World");
}


// - Ek function likho jo &String le aur &str return kare.
// - Aisa code likho jisme mutable + immutable borrow aa jaye (error), fir fix karo.
// - Do struct ka lifetime link kar ke safe code banao.
// 
//  im[ple ment 2 struct ]
// 
struct User{
    name:string,
    age:u8
}
struct job{
    position:String,
    payment:u32
}


// lifetime link  pending
// 
// 
// 