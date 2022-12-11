#![warn(clippy::unwrap_used, clippy::expect_used, clippy::pedantic)]

mod cli;

fn main() {
    cli::parse();
}
