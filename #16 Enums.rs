//Enum is a datatype that is used when you want
// a variable to have a value out of some possible choices

//"An enum variable can only hold a value that matches the
//  data type defined for one of its variants

//Let us look at an example of IP Address
//In real life Any IP address can be either a version four or a version six address
// but not both at the same time.

enum IpAddrKind {
    V4,
    V6,
}

//We can create instances of each of the two variants of IpAddrKind like this:


let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

//We can use struct to bundle the kind of IP address along with actual data 

struct IP {
    kind: IpAddrKind,
    address: String
}

let home = IP {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IP {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};

//Two structs with data and the kind
//Now a particular variant is associated with a value

//Below is a better way to do this

enum IpAddrs {
    V4(String),
    V6(String),
    error
}

//Here error is a variant of IpAddrs that has no no extra value no data type.

// The name of each enum variant we define becomes a function.
// It takes in a value 

let home = IpAddrs::V4(String::from("127.0.0.1"));
let loopback = IpAddrs::V6(String::from("::1")); // It returns an instance of IpAddrs

//There’s another advantage to using an enum rather than a struct:
//  each variant can have different types and amounts of associated data. 
// Version four IP addresses will always have four numeric components that 
// will have values between 0 and 255. If we wanted to store V4 addresses as
//  four u8 values but still express V6 addresses as one String value, 
// we wouldn’t be able to do that with a struct.

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));


//In rust's standard library IpAddr is a reserved word which takes a particular variant
//And that variant passed must be a struct which contain bundle of multiple related
//data types

//The code inside std::net looks like this
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

//Note that even though the standard library contains a definition 
// for IpAddr, we can still create and use our own definition without
//  conflict because we haven’t brought the standard library’s definition 
// into our scope

//Using the IpAddr Enum by passing in a struct
struct Ipv4Addr {
    octets: [u8; 4],  // A simple representation for IPv4 address (4 bytes)
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

let v4 = IpAddr::V4(
    Ipv4Addr {
        octets: [192, 168, 1, 1],
    }
);

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// This enum has four variants with different types:
// Quit has no data associated with it at all. - Message
// Move has named fields, like a struct does. - Struct
// Write includes a single String. The heap allocated dynamically sized - String type
// ChangeColor includes three i32 values. A tuple

// There is one more similarity between enums and structs: just as
//  we’re able to define methods on structs using impl, we’re also 
//  able to define methods on enums

impl Message {
    fn call(&self) { //borrows Message immutably
        self
    }
}

let m = Message::Write(String::from("hello"));
m.call();

fn main() {

}