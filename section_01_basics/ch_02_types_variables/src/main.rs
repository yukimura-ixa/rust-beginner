fn main() {
    // Static typing
    let name: &str = "Noon"; // &str is a string slice
    let age: u32 = 20; // u32 is an unsigned 32-bit

    println!("Name: {}, Age: {}", name, age);

    // floating-point numbers
    let a: f64 = 100.0/3.0; // f64 is a 64-bit floating-point number
    println!("A: {}", a);

    //boolean
    let is_rust_fun: bool = true; // bool is a boolean type
    println!("Is Rust fun? {}", is_rust_fun);

    // characters
    let c: char = 'A'; // char is a Unicode scalar value
    let emoji: char = '😊'; // char can also represent emojis
    println!("Character: {}", c);
    println!("Emoji: {}", emoji);

    // character type string
    let greeting: String = "Hello, Rust!".to_string(); // String is a growable, heap-allocated data structure
    println!("Greeting: {}", greeting);

    // Variables are immutable by default
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 10; // This would cause a compile-time error

    // To make a variable mutable, use the mut keyword
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 10; // Now this is allowed
    println!("The new value of y is: {}", y);
    y+= 5; // y is now 15
    println!("The final value of y is: {}", y);

    // Constants are always immutable and must have a type annotation
    const MAX_POINTS: u32 = 100_000; // Constants are declared using the const keyword
    println!("The maximum points are: {}", MAX_POINTS);

    // Shadowing allows you to declare a new variable with the same name as a previous variable
    let z = 5;
    let z = z + 1; // This shadows the previous z
    let z = z * 2; // This shadows the previous z again
    println!("The value of z is: {}", z); // This will print 12
    let z: &str = "ZZZ"; // This shadows the previous z again
    println!("The value of z is: {}", z); // This will print "ZZZ"

    //Scope and blocks
    let a = 10;
    {
        let b = 20;
        println!("Inside the block: a = {}, b = {}", a, b); // This will print a = 10, b = 20
    }
    // println!("Outside the block: a = {}, b = {}", a, b); // This will cause a compile-time error because b is not in scope here

    // Compiler Directives
    #[allow(unused_variables)] // This directive tells the compiler to allow unused variables in the following block of code
    {
        let unused_variable = 42; // This variable is not used, but the compiler will not warn about it
    }   

    #[allow(dead_code)] // This directive tells the compiler to allow dead code in the following block of code
    fn unused_function() {
        // This function is not used, but the compiler will not warn about it
    }

    // Type Aliases
    type Kilometers = i32; // This creates a type alias for i32 called Kilometers
    let distance: Kilometers = 5; // Now we can use Kilometers as a type
    println!("Distance: {} kilometers", distance);

    // Rust Error Handling
    // Rust does not have exceptions. Instead, it has a type called Result<T, E
    // where T is the type of the value that will be returned on success, and E is the type of the error that will be returned on failure.
    let result: Result<i32, &str> = Ok(42); // This is a successful result
    match result {
        Ok(value) => println!("The value is: {}", value),
        Err(error) => println!("An error occurred: {}", error),
    }
    let result: Result<i32, &str> = Err("Something went wrong"); // This is a failed result
    match result {
        Ok(value) => println!("The value is: {}", value),
        Err(error) => println!("An error occurred: {}", error),
    }

}
