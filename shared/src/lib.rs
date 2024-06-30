use once_cell::sync::Lazy;

pub mod config;
pub mod errors;
pub mod parse;

pub const CONFIG: Lazy<config::Config> = Lazy::new(|| config::Config::init());
