//Obviously You won't be calling the complete path of a function to call it
//To save the day we have 'use'  in Rust

--Start of a file---
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

    use crate::front_of_house::hosting;
    //This line
    //will work even if front of house is a file or 
    //if it is a module within the same file
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}



//The below code won't compile
use crate::front_of_house::hosting;
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}


mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

//An import declared in a module is only available within that module, not automatically inside its child modules
//Because use is not in same scope as the customer's insides

//FIX:
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    use crate::front_of_house::hosting;
    
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

use std::fmt::Result;
use std::io::Result;

//Mind you: Both are named Result but can have different data types


fn format_thing() -> Result {
    // formatting logic
    Ok(())
}

fn write_thing() -> Result {
    // writing logic
    Ok(())
}

//Rust doesn't know what result the code is referring to

//Solution 1:

//Instead bring them this way
use std::fmt;
use std::io;

fn format_thing() -> fmt::Result {
    // formatting logic
    Ok(())
}

fn write_thing() -> io::Result {
    // writing logic
    Ok(())
}

//Solution 2:
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult {
    // --snip--
}

//Creating a public shortcut within a module (Note this can also be done to create shortcuts at the root of the crate )
//==FILE===
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

//Using External Packages
//Members of the Rust community have made many packages available at crates.io,
// and pulling any of them into your package involves these same steps: listing them in your package’s Cargo.toml file
// and using use to bring items from their crates into scope.

//In Cargo.toml file, rand = "0.8.5"
//And then in your project file

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}

//Instead of this
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--

//You can do this
// --snip--
use std::{cmp::Ordering, io};
// --snip--

use std::io::{self, Write};
// This line brings std::io and std::io::Write into scope.

use std::collections::*;
//The * is also called glob operator Imports everything that is public within that module