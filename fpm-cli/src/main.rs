#![warn(clippy::unwrap_used, clippy::expect_used, clippy::pedantic)]

mod cli;
mod commands;
mod utils;

use eyre::Result;

fn main() -> Result<()> {
    cli::parse()?;

    Ok(())
}
