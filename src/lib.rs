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

// max limit tends to be 8KB (Firefox), 4KB (Opera), or 2KB (IE, Safari)
const MAX_REQUEST_LENGTH: usize = 8192;
const DEBUG: bool = false;

pub fn handle_request<S: io::Stream>(stream: S) -> () {
    let mut stream_box: Box<S> = Box::new(stream);
    let request: String = read_request(&mut* stream_box);

    // request can have multiple lines (request line + any headers, e.g. Host)
    // only concerned with the first line (for now), so lets just retrieve it.
    let request_line: &str = request.as_slice().lines_any().nth(0).unwrap();

    server::respond(&mut *stream_box, request_line);
}

pub fn read_request<S: io::Stream>(mut stream: &mut S) -> String {
    // read request into buffer
    let mut buf: [u8; MAX_REQUEST_LENGTH] = [0; MAX_REQUEST_LENGTH];
    let bytes_read: usize = stream.read(&mut buf).unwrap();

    if DEBUG {
        println!("Request:\n{}", std::str::from_utf8(&buf).unwrap());
    }

    // extract the request from the buffer
    let request: &[u8] = buf.slice_to(bytes_read);

    // create a string slice representation of the request from the raw utf8 bytes
    let request_str: &str = std::str::from_utf8(request).unwrap();

    request_str.to_string()
}