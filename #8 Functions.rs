//Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.
// Any function called in main can be defined after or before main

//The values passed in the parameters are the concrete values called arguments

//Statements are instructions that perform some action and do not return a value.
//Expressions evaluate to a resultant value. Let’s look at some examples.

//When we define a function say fn main() {} this is called a function definition it is also a statement
//When we call a function or a macro or create a new scope it is an expression. An expression returns a value

//Invalid Code: let x = (let y = 6); This will not work be cause let y = 6 is a statement

//This is different from what happens in other languages, such as C and Ruby, where the assignment returns the value 
// of the assignment. In those languages, you can write x = y = 6 and have both x and y have the value 6;
//  that is not the case in Rust.

fn another_function(x: u8) {
    //In function signatures, you must declare the type of each parameter. 
    println!("Another function. Hey the number is {x}");
    const LOL: &str = "SSS";
    let y = {
        let x = 3;
        x + 1

        //Note that the x + 1 line doesn’t have a semicolon at the end, which is unlike most of the lines 
        // you’ve seen so far. Expressions do not include ending semicolons.
        //  If you add a semicolon to the end of an expression, you turn it into a statement
    };

    //The block {
    // } here creates a scope (Which is also an expression) where you can write code to 
    // define y's value x+1 is an expression that return value to y 
    println!("Haha {y}");
}


fn main() {
    //When you run a Rust program, execution starts from main. All other functions must be put in main() for execution
    println!("The main function ran");
    another_function(255);
    let summation = {
        sum(900, 2900) + 21262
    };
    println!("{summation}");
}


//We are specifying the datatype of the returned value by our function
fn sum(a: i64, b: i64) -> i64 {
    a + b
}

// A perfectly valid function is Rust. There is no need of return keyword in Rust. Just don't use a semicolon and it gets the
//job done
fn five() -> i32 {
    5
}