//Slices let you reference a contiguous sequence of elements in a collection rather than
//the whole collection. A slice is a kind of reference, so it does not have ownership.

//Here’s a small programming problem: write a function that takes a string of words separated 
//by spaces and returns the first word it finds in that string. If the function doesn’t find a space in the string,
//the whole string must be one word, so the entire string should be returned.

//Syntax of Slices: &[T]  and &str where T is the data type of the collection


fn main() {
    let mut s: String = String::from("Hello World!");
    let len = s.len();

    let arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    let slice_arr: &[char] = &arr[1..=3];
    //The slice has the type &[char]

    println!("{:?}", slice_arr);


    let slice_string: &str = &s[0..5];
    let hello: &str = &s[..5]; //same as Hello
    let world: &str = &s[6..];
    let hello_world: &str = &s[..len]; //Same as Hello World!

    let all: &str = &s[..];

    println!("{}", hello_world); 
    println!("{}", slice_string);
    println!("{}", world);
    println!("{}", all);

    let bytes = b"Hello";   //the b just before the string converts each character into a number
    //Overall this bytes variable return an array of numbers. Where the numbers represent the
    //character 
    println!("{:?}", bytes);

    //The as_bytes() method in Rust is used to convert a string slice (&str) into a byte array (&[u8]
    //arr.iter() is same as &arr

    for val in bytes.iter() {
        println!("{val}");
    }  //.iter() gives value in an array by iterating

    //why is the function without slices risky??
    let index = first_word_without_slices(&s); // index = 5
    s.clear(); // Clears the string (makes it empty)

    // Now, index is 5, but the string is empty!
    println!("{index}"); // ❌ Still prints 5!
    // / the index itself being 5 is not inherently wrong—it correctly represents 
    // where the first space was at the time the function was called. The risk comes 
    // when you try to use that index later on a modified string, which can lead to 
    // unexpected behavior or crashes.
}

//Let us get the first word of a string without slices
fn first_word_without_slices(txt: &String) -> usize{
    let mut s: String = String::from("Hello World!");
    let len = s.len();
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i; // Returns index of first space
        }
    }

    s.len()
    //This function takes a reference to a String and returns a usize,
    //  which represents the index of the first space (' ') in the string.
    //  If no space is found, it returns the length of the string.

    let s = "Hello, world!"; // a string literal
    //It is immutable

    //&String is a refrence to a string
    //&str is a refrence to a slice of string
    
    //Knowing that you can take slices of literals and String values
    //  leads us to one more improvement on first_word, and that’s its signature:

    //A more experienced Rustacean would write the signature
    //fn first_word(s: &str) -> &str {
    // because it allows us to use the same function on both &String values and &str values.
    //If we have a string slice, we can pass that directly. If we have a String, 
    // we can pass a slice of the String or a reference to the String
}

//Metadata is extra info that
//  describes the data — like its length, capacity, or type.

