//A literal in programming is a fixed value written directly in the code.

//Two types of a memory in a computer's RAM are
//Stack: Fast, small memory used for fixed-size data.
//Heap: Slower, larger memory used for dynamically sized data.

fn main() {
    //There are broadly two datatype catgeories in Rust. (Acutally there is a third)
    
    //Rust is a statically typed language which means type of the variables
    //are defined at compiletime for this user....
    
    //JS is a dynamically typed language meaning data type of the variables are
    //figured out during running the code (runtime)
    
    let _guess: u32 = "42".parse().expect("Not a number!");
    
    //.parse() is used to convert a string into any another type
    
    //If we don’t add the : u32 type annotation shown in the preceding code, 
    // Rust will display an error, which means the compiler needs 
    // more information from us to know which type we want to use:
    
    
    //Scalar
        //A scalar type represents a single value. Rust has four primary scalar
        //types: integers, floating-point numbers, Booleans, and characters
    
        //Integer type includes 
    
            // Unsigned (Meaning only positive numbers): u8, u16, u32, u64, u128, uSize
            // 0 to ((2^N) - 1)) For any uN we have these many digits allowed
            //For eg u8's range = 0 to 2^8-1 = 255
            let whole: u32 = 150;
            println!("{}", whole);
            let whole: u64 = 21262; //Shadowing here
            println!("{}", whole);
    
            // Signed (Meaning it can store both + and - numbers): i8, i16, ....
            // -(2^(N-1)) to (2^(N-1) - 1) the formula for range of an iN
            //for eg i8's range = -128 to 127
            let integer: i32 = -150;
            println!("{}", integer);
            let integer: i64 = -21262; //Shadowing here
            println!("{}", integer);
    
            // hexadecimal is useful for writing large numbers compactly
            
            //21_262 is another way of saying 21,262
            //The underscore (_) is just a visual separator for readability in large numbers.
            
            //usize and isize allow number of digits based on a system's bits

            // Integer Overflow - When setting value of a uN variable to a value greater
            // than the possible max value it is called integer overflow
            // When you’re compiling in release mode with the --release flag, 
            // ie - cargo build --release
            // Rust does not include checks for integer overflow that cause panics
            // Instead The value 256 becomes 0, the value 257 becomes 1, and so on.

        //Default Integer Type is i32. By default RUST doesn't assume a numerical value to be unsigned but i32
        //  Floating-point types
            // f32 → ~7 digits (Faster, less memory, but less precise)
            // f64 → ~15-16 digits (Default in Rust, more precise)
            let decimal: f32 = 212.623142719;
            println!("The decimal is {decimal}");
            let decimal = 212.623142719; //By default it is f64
            println!("The decimal is {decimal}");

        //Boolean
            // There are two possible values true and false
            let t: bool = !false;
            println!("The boolean is : {t}");

        //Character
            // Rust’s char type is the language’s most primitive alphabetic type.
            // The character literals are stored in single quotes rather than double (used for sentences)

            let emoji: char = '🤣';
            let lol = 'H';
            println!("{lol}ahaha {emoji}");
        
    //Compound
    //Compound types can group multiple values into one type

        //Tuple
            // Tuples have a fixed length: once declared, they cannot grow or shrink in size.

            let a_tup: (char, char , char, i8) = ('✅', '🎁', '😎', 100);
            let (tick, gift, cool, num) = a_tup;
            // let (tick, gift, cool) = aTup; - an error because only 3 of them took the value

            println!("{tick} {gift} {cool} and {num}");
            let tick = a_tup.0;
            let gift = a_tup.1;
            let cool = a_tup.2;
            let num = a_tup.3;
            println!("The new ones are {tick} {gift} {cool} and {num}");

            //The tuple without any values has a special name, unit. This value and its corresponding type are both written ()
        
        //Array
            // Unlike arrays in some other languages, arrays in Rust have a fixed length.
            // All the elements in a Rust's array must have the same data type
            // Arrays are more useful when you know the number of elements will not need to change

            let months = ["January", "February", "March", "April", "May", "June", "July", 
            "August", "September", "October", "November", "December"];
            //Another way is given below
            let numbers: [i8; 5] = [1, 2, 3, 4, 5]; //[i8, 5] the 5 tells there are 5 elements in the array

            let a = [21262; 3]; // Another way of writing let a = [21262, 21262, 21262];

            let the_num_i_want_to_print = numbers[1];
            println!("This is {the_num_i_want_to_print}");

            //Unlike JS a tuple or array must be marked with mut just to change its
            //inner elements. With mut you cannot change inner elements.

            let mut arr = [1, 2, 3];
            arr = [7, 8, 9]; // valid
}