//To do what match does but in less code (verbose) way.
//We can use if, let and else combined with it

//Consider the code given below
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    _ => (),
}


//If the value is Some, we print out the value in the Some variant by binding
//  the value to the variable max in the pattern. We don’t want to do anything 
// with the None value. To satisfy the match expression, we have to add _ => ()
//  after processing just one variant, which is annoying boilerplate code to add.


//You can think of if let as a simple of writing match that is concerned only with one case and for all the other
//case it returns a unit tuple / unit type


let config_max: Option<u8> = Some(8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}

//One more example
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    _ => count += 1,
}

let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}!");
} else {
    count += 1;
}
println!("{state:?}!");// Not valid state was only valid inside the curly braces ❌

fn main() {

}

//However thrugh the if let method the exhaustive matching provided by match in Rust is lost
//So it is upto you to decided which method suits the best for the current case

//Let Else in Rust

let Coin::Quarter(state) = coin else {
    println!("It was not a quarter")
    return;
}
//Here for if case we just attatched the value of coin to the state 
//After this line the state can be used as any other variable
println!(state); //✅

//Note: The else block is mandatory and must diverge (e.g., return, panic!, break, continue) 
//to ensure the variables are always initialized for the subsequent code in the outer scope.

//Summary
//Use match if you want full exhaustive matching over all variants.
//Use if let for quick pattern checks when you don’t care about all other cases.
//Use let…else when you want a binding after the pattern and to bail early if it doesn’t match.
