fn main() {
    let number = 3;

    //The if statements in Rust
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //Shadowing
    let number = 53;
    //if number {
        println!("number was three");
    //}
    
    // The if condition evaluates to a value of 3 this time, and Rust throws an error: saying it got an integer and it was expecting a boolean
    // this is unlike javascript where putting variable in the if condition simply means true

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 3243;

    //Rust only executes the block for the first true condition, and once it finds one, it doesn’t even check the rest.
    if 21262 % number == 0 {
        println!("21262 is divisible by the number");
        if number == 21262 {
            println!("In fact the nmber is 21262");
        }
    } else if number % 2 == 0 {
        println!("The number is divisble by 2");
    } else {
        println!("That is a stupid number yo got there");
    }

    //Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable
    let number2 = if number == 21262 { number - 21262 } else { number + 21262 };
    println!("{number2}");

    // let number2 = if condition { 5 } else { "six" };
    //The expression in the if block evaluates to an integer, and the expression in the else block evaluates to a
    //  string. This won’t work because variables must have a single determined type, and Rust needs to know at compile time what
    //  type the number variable is. Compiler doesn't run the code and therefore must know in advance what data type is of that variable
    //Right now it is ambigous
    println!("The value of number is: {number2}");

    // loop {
        // println!("again!");
    // }
    //When we run this program, we’ll see printed over and over continuously until we stop the program manually

    let mut numero = 0;
    loop {
        numero += 2;
        println!("{numero}");
        if numero >= 10 {
            break;
            //The above statement tells Rust to stop the loop
        }
    }

    let result = loop {
        numero -= 1;

        if numero == 3 {
            break numero * 2; //Basically return 0 to result while breaking the loop
            //continue;
            //If you use a continue statement the continuing lines in the loop are not executed and loop is run from the starting line.
            //ie - continue; statement is used within loops to skip the current iteration and move directly to the next iteration of the loop
            continue;
            //any code after a break; statement in a loop does not run because break; immediately exits the loop
        } else {
            //Whatever 
            println!("Stil not a 0");
        }

        println!("HI!"); // This will not be printed when numero == 3 even though it is not inside the else statement
    };

    println!("The result is {result}");

    let mut i = 1;
    let mut j = 1;
    'outer: loop {  // Outer loop a name labelling
        loop {  // Inner loop
            println!("The inner loop has begun");
            println!("i: {}, j: {}", i, j);
            
            i += 1;
            j += 1;
            println!("after addition i: {}, j: {}", i, j);
           
            if i >= 4 && j >= 4 {
                break;  // Break only the inner loop
            }

            //The below statement will not be printed when i or j becomes 4 or greater
            println!("This statement is after break");
         
            if j % 5 == 0 {
                break 'outer;  // Break only the outer loop
            }
        }
        println!("The inner loop has ended");

        i += 1;
        if i > 10 {
            break;  // Break outer loop if condition meets
        }
    }

    println!("Loop ended."); // Runs only after loop ends

    //It is possible to use the behaviour of for loop and using break when a condition is true by using a while loop
    let mut number = 5;
    //while loop in Rust
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    //We can use while loop for looping over an array
    let array: [i64; 5] = [17, 18, 15, 26, 29]; 
    let mut index = 0;
    while index < array.len() {
        println!("the value is: {}", array[index]);

        index += 1;
    }
    //While statements for looping over an array take a lot of memory. Therefore it is advised to use for in
    for value in array {
        println!("The value is {value}");
    } 

    //(1..=4) Inclusive range 1,2,3,4
    //(1..4) Exclusive range 1,2,3,
    for value in (1..=4).rev() {
        //.rev() is a method that reverses the range
        println!("In descending order it is {}", value);
    }
}
