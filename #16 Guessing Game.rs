//Module is a smaller unit of the library. For eg library
//Contains 10 files meaning it has 10 modules
//In JavaScript, a module is typically a single file,
//  not individual functions.


use std::io; 
//brings the (input/output)io module from Rust’s standard 
// library (std) into scope

fn main() {
    println!("Please make an input");
    let mut guess = String::new();
    // mut defines a mutable variable (Changeable) 
    // For an immutable variable you can use let apples = 5;
    //String::new is basically = ""
    //::new is an associated function.  An associated function 
    // is a function that’s implemented on a type, in this case String

    
    //However, if you ask "What is the data type of io::stdin()?", the answer 
    // is: struct
    //If we hadn’t imported the io library with use std::io; at the
    //  beginning of the program, we could still use the function by
    //  writing this function call as std::io::stdin
}

//Rust’s prelude contains commonly used types, traits, and functions
// that are automatically imported into every Rust program
