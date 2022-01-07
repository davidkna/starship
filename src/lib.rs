#![warn(clippy::disallowed_method)]

#[macro_use]
extern crate shadow_rs;

shadow!(shadow);

// Lib is present to allow for benchmarking
pub mod bug_report;
pub mod cache;
pub mod config;
pub mod configs;
pub mod configure;
pub mod context;
pub mod error;
pub mod formatter;
pub mod init;
pub mod logger;
pub mod module;
mod modules;
pub mod print;
mod segment;
mod utils;

pub use error::Error;

#[cfg(test)]
mod test;
