#![feature(plugin)]
#![allow(unstable)]

#[plugin]
extern crate regex_macros;
extern crate regex;

pub use subway::data::load_subway_data;

pub use subway::route::find_route;
pub use subway::route::Query;
pub use subway::route::Query::{Route, Enable, Disable};

pub mod subway;
pub mod server;
