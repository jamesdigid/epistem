pub mod catalog;
pub mod cli;
pub mod error;
pub mod learn;
pub mod manifest;
pub mod models;
pub mod provider;
pub mod registry;
pub mod reasoning;
pub mod resolver;
pub mod search;
pub mod runtime;
pub mod storage;
pub mod verification;
pub mod utils;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
