// Rust’s naming convention for constants is to use all uppercase 
// with underscores between words

//Constants are valid for the entire time a program runs, 
// within the scope in which they were declared.
//They can only be declared in the Global Scope

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//Shadowing can be done to constants . Where the redefinition is in a different scope

//The above is a constant it is different from a default immutable
// variable. For constants it is important to tell what type it is
// In this case it is an unsigned 32 bit integer

//Constants can be declared in 1. Global scope (outside any function) 2. Inside functions (but not inside inner blocks or if statements)
//  These are useful values that many parts of code need to know about.

//SHADOWING
//By using let, we can perform a few transformations on a value but have
//  the variable be immutable after those transformations have been
//  completed.

let mut spaces = "   ";
spaces = spaces.len(); //.len() is like .length() from javascript. 
// it will give a number
//The error says we’re not allowed to mutate a variable’s type:

//BUT
let spaces = "  ";
let spaces = spaces.len();
//Shadowing allows us to reassign even a different type 


fn main() {
    let x = 5;
    const THREE_HOURS_IN_SECONDS: u32 = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
        //Prints 12 because this is a different scope where x was undefined
    }

    println!("The value of x is: {x}");
    //The value printed will be 6 and not 5
    //The new X has shadowed the previous X. 
    // Remember This is different from changing the value of X
    //Prints 6
}

println!("The value of x is: {x}"); 
//Note the {} tells computer that inside it is a variable or a constant
