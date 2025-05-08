//In order to perform expressions that the type of a variant allow. We must write code for each type of variant
//This is done using Match It is a powerful Control Flow Statement

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1 //A return expression as it doesn't end with a semicolon
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

//Match checks for the variant of Coin and then returns a value. For penny it is 1
//Inside the match there are arms. An arm has two parts pattern and some code. Use Curly brackets if there is a lot of code


// When the match expression executes, it compares the resultant value against the pattern of each arm, in order. If a
// pattern matches the value, the code associated with that pattern is executed. If that pattern doesn’t match the value,
// execution continues to the next arm

fn main() {

}