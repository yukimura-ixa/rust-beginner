fn main() {
    let num = 5.36;

    // Using if-else statement to check if the number is positive, negative, or zero
    if num > 0.0 {
        println!("The number is positive.");
    } else if num < 0.0 {
        println!("The number is negative.");
    } else {
        println!("The number is zero.");
    }

    // ternary-like operation using if-else expression to determine the sign of the number
    let sign = if num > 0.0 {
        "positive"
    } else if num < 0.0 {
        "negative"
    } else {
        "zero"
    };
    println!("The number is {}.", sign);

    // ternary-like operation using if-else expression to determine if the number is odd or even
    let is_odd = if (num as i32) % 2 != 0 { true } else { false };
    println!("The number is {}.", if is_odd { "odd" } else { "even" });
}
