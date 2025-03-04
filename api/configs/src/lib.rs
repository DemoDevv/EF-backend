use once_cell::sync::Lazy;

pub mod config;
pub mod parse;

pub static CONFIG: Lazy<config::Config> = Lazy::new(config::Config::init);
