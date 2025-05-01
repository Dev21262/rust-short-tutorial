struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let mut rect1 = Rectangle {
        width: 90,
        height: 179
    };

    rect1.height += 1; //Since the instance (rect1) of the Rectangle Struct is mutable
    println!("{}", rect1.height);

    //Each struct is allowed to have multiple impl blocks
    impl Rectangle {
        //The implemenation block. functions put here become a part of the struct
        fn area(&self) -> u32 {    //self is Comparable to this in javascript. 
        // Use self if you want to take ownership of the properties. Use &mut self if you want to take a mutable refrence to the instance's properties
            self.width * self.height
        }   
        //You can implement multiple methods
        fn width(&self) -> bool {
            self.width > 0
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            //We can define more argument for a method just like we do for a regular function
            //The other parameter borrows a Rectangle (Yes a struct name is its own type) type immutably
            self.width > other.width && self.height > other.height
        }
        //These three are methods of the Struct
        
        //An associated function however doesn't take self as an argument and therefore is a function that doesn't need an instance to work on
        fn createSquare(length: u32) -> Self { //Self is an alias of Rectangle In this case.
            Self {
                width: length,
                height: length
            }
        }
        //To call this associated function, we use the :: syntax with the struct name

    }

    println!("{:?}", rect1.area()); //Prints the area
    println!("{:?}", rect1.width()); //Prints true/false

    //When we follow rect1.width with parentheses, Rust knows we mean the method width. 
    // When we don’t use parentheses, Rust knows we mean the field width.
    
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let square: Rectangle = Rectangle::createSquare(21262);
    //Since square is a rectangle after all :D

}

//Often, but not always, when we give a method the same name as a field we want it to only return
// the value in the field and do nothing else. Methods like this are called getters, and Rust does not 
//implement them automatically for struct fields as some other languages do. Getters are useful because you can make the field private but the method public