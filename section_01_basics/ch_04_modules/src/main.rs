mod greeting;
mod greetings;

// use greetings::morning;
// use greetings::evening;
use greetings::{morning, evening};

fn main() {
    greeting::hello();
    greeting::bye();
    morning::good_morning();
    evening::good_evening();
}
