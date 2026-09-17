fn main() {
    another_function();
    let sum = add(5, 10);
    println!("The sum is: {}", sum);

    // Block expression
    let difference = {
        let x = { 2 * 19 };
        let y = 5;
        x - y
    };
    println!("The difference is: {}", difference);
}

// Implicit return type
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
// Explicit return type
fn subtract(x: i32, y: i32) -> i32 {
    return x - y;
}

fn another_function() {
    println!("This is another function.");
}
