#![allow(unstable)]
#![feature(plugin)]

#[plugin]
extern crate regex_macros;
extern crate regex;

use std::io;

pub use subway::load_subway_data;
pub use subway::find_route;

pub use subway::Query;
pub use subway::Query::{Route, Enable, Disable};

pub mod subway;
pub mod server;

const DEBUG: bool = false;