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

fn main() {

}