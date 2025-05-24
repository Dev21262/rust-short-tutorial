//In order to perform expressions that the type of a variant allow. We must write code for each type of variant
//This is done using Match It is a powerful Control Flow Statement

//match itself is an expression, meaning it produces a value.
//  You can assign the result of a match to a variable.

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
//The code can return a value and also contain some statements


// When the match expression executes, it compares the resultant value against the pattern of each arm, in order. If a
// pattern matches the value, the code associated with that pattern is executed. If that pattern doesn’t match the value,
// execution continues to the next arm

//Note Match is not limited to only enums. Match can also be used
//With Integers, Tuples, Structs etc

let x = 3;

match x {
    1 => println!("One"),
    2 | 3 => println!("Two or Three"),
    4..=10 => println!("Between 4 and 10"),
    _ => println!("Something else"),
}
// 2 | 3 is an or operation
// _ handles all the other case. Note we must handle all cases while using the match statement.
//Exhaustive checking in Rust means the compiler forces you to cover every possible case 
// Sometimes for most of the cases you want a default action you can use the keyword 'other' or '_' for that

//We can bind values while matching for later use
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn process(msg: Message) {
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to bind.");
        }
        Message::Move { x, y } => {
            // `x` and `y` are bound to the values inside the struct variant
            println!("Moving to coordinates: ({}, {})", x, y);
        }
        Message::Write(text) => {
            // `text` is bound to the string inside the variant
            println!("Message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            // `r`, `g`, `b` are bound to the respective color values
            println!("Changing color to red={}, green={}, blue={}", r, g, b);
        }
    }
}

fn main() {
    let m = Message::Move { x: 10, y: 20 };
    process(m); //The values x, y of move struct (a variant) of the enum are moved which are binded for use
}

// The other keyword for handling all other cases. We can bind value of other to a function 
// Which will not be possible when we just _

let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}
//That above is a unit tuple or an empty tuple which often times is used to handle cases where you wish to do nothing