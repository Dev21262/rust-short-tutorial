// (We call the libraries that your code needs dependencies.)

/**Cargo is Rust’s build system and package manager. Most Rustaceans 
use this tool to manage their Rust projects because Cargo handles
a lot of tasks for you, such as building your code, downloading
the libraries your code depends on, and building those libraries**/

/**A build system in programming is a set of tools and processes 
that automate the creation of executable programs from source code**/

$ cargo new hello_cargo //In the terminal this will 
// create a new rust project intgerated with cargo inside the directory 
// named hello_cargo. It will generate a folder of src and config file 
// of cargo named Cargo.toml. All coded files are put inside src

$ cargo init //It will create a .toml file automatically for you

$ cargo build //It creates and exe program inside target/debug
//A debug build is a version of a program that is compiled with extra 
// information to help developers test and debug the software
//Build compiles the code into machine code

$ cargo check //It partially compiles the code
//  without producing an exe program. Sometimes 
// Devs just need to check for compilation errors(the do this by cargo check)
// When they are ready for producing an exe they run cargo build
//Skips final step of producing machine code

$ cargo run //We just built a project with cargo build and ran it
//  with ./target/debug/hello_cargo, but we can also use cargo run 
// to compile the code and then run the resultant executable all in one 
// command:

//Cargo is the package manager and build system of Rust
