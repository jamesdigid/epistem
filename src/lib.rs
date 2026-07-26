pub mod catalog;
pub mod cli;
pub mod error;
pub mod manifest;
pub mod models;
pub mod registry;
pub mod resolver;
pub mod search;
pub mod storage;
pub mod utils;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
