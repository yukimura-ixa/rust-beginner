fn main() {
        
    // String Interpolation
    let name = "Noon";
    let age = 20;
    println!("My name is {} and I am {} years old.", name, age);

    // {0} {1} {2} are positional arguments
    println!("{0} is my name and {1} is my age.", name, age);

    // Named arguments
    println!("{name} is my name and {age} is my age.", name=name, age=age);

    // Formatting numbers
    let number = 42;
    println!("The number is: {}", number);
    println!("The number in binary is: {:b}", number);
    println!("The number in hexadecimal is: {:x}", number);
    println!("The number in octal is: {:o}", number);
    // padding with spaces
    println!("The number padded with spaces: {:>5}", number);
    // padding with zeros
    println!("The number padded with zeros: {:0>5}", number);
    // Formatting floating-point numbers
    let pi = 3.14159;
    println!("Pi is approximately: {}", pi);
    println!("Pi is approximately: {:.2}", pi);
    println!("Pi is approximately: {:.3}", pi);
}
