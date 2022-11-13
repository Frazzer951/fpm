#![warn(clippy::unwrap_used, clippy::expect_used, clippy::pedantic)]

fn main() {
    println!("Hello, world!");

    let result = fpm::add(1, 1);
    println!("1 + 1 = {result}");
}
