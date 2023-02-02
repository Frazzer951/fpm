#![warn(clippy::unwrap_used, clippy::expect_used)]

// Must be included first to not cause compile error
pub mod project;

pub mod config;
pub mod database;
pub mod error;

mod utils;
