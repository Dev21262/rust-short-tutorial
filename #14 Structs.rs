//A struct, or structure, is a custom data 
// type that lets you package together and name multiple 
// related values that make up a meaningful group

//Comparable to an object if you are familiar with OOP
//Each struct you define is its own type
#[derive(Debug)]

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
//A struct’s name should describe the significance of the pieces
//  of data being grouped together.


//To use a struct after we’ve defined it, we create an instance
//  of that struct by specifying concrete values for each of the fields.
fn main() {
    //Remember let cannnot be used for global variables because global variables are stored in static memory
    //By using let we let it be mutable and mutables are stored in stack or heap
        
    //Note that the entire instance must be mutable;
    //  Rust doesn’t allow us to mark only certain fields as mutable
    
    let user1 = User { //user1 is an instance of User
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    
    
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // Spreads the properties of user2 that were not defined here
    };
    println!("{:?}", user1.email);  // is still active since it was not moved

    // The above is a way of writing the below
//    let user2 = User {
//        active: user1.active,
//       username: user1.username, //User1's username value was moved because it is a string (therefore not copied)
//       email: String::from("another@example.com"),
//        sign_in_count: user1.sign_in_count, //Value copied because an integer is stored on stack
//    };
    
    let user4 = user2;
    // println!("{:?}", user2); - an error
    // println!("{:?}", user2.sign_in_count);  - an error

    //instances of structs in Rust are moved unless all  of the properties implement the Copy trait. 
    //Only if the entire instance has properties with Copy trait. Else Once all the instances are moved,
    //  none of its properties are accessible — even the ones that were stored entirely on the stack and have Copy

    tuple_structs();
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }

    //Just like javascript. We can skip the part of writing variables with same name but different context twice
}

fn tuple_structs() {
    //Rust also supports structs that look similar to tuples, called tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Point(xa, ya, za) = origin;
    println!("Origin at ({}, {}, {})", xa, ya, za);

    //a function that takes a parameter of type Color cannot
    //  take a Point as an argument, even though both types are made up of three i32 values
    //fn lol (c: Color)
    // let xd = point(0,0,0);
    //cannot call lol(xd)

    //An empty tuple in rust is called Unit Tuple
    //An empty struct in rust is called unit-like structs
    struct AlwaysEqual; //Come back to this after you have learned traits
    let subject = AlwaysEqual;
    println!("{:?}", subject); //{:?} tells Rust that this is not a primitive type that has the display trait. And print it in debug mode

    //Unit-like structs can be useful when you need to implement a trait on some type but don’t have any 
    // data that you want to store in the type itself
}

//It’s also possible for structs to store references to data owned 
// by something else, but to do so requires the use of lifetimes, a Rust feature that we’ll discuss later


//#[derive(Debug)]
//Rust does include functionality to print out debugging information, but we have to explicitly opt in to 
// make that functionality available for our struct.

//Only simple data types like integers and strings have the Display Traits others will require some additional information
//to display on the CLI using the println! statement