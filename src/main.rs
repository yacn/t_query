#![allow(unstable)]
#![allow(unused_mut)]
#![allow(unused_imports)]

extern crate t_query;

use std::io;
use std::os;

use t_query::subway::Subway;
use t_query::load_subway_data;
use t_query::find_route;

use std::sync::{Arc, Mutex};

const BIND_ADDR: &'static str = "127.0.0.1:12345";

fn main() {
    let args: Vec<String> = os::args();
    let args: &[String] = args.tail();

    if args.len() == 0 {
        println!("ERROR: Must provide at least one subway data file!");
        return;
    }

    let mut subway: Subway = Subway::new();

    for arg in args.iter() {
        let path: Path = Path::new(arg);
        if let Some(subway_line) = path.filestem_str() {
            let file = io::File::open(&path);
            let file_buf = io::BufferedReader::new(file);
            load_subway_data(&mut subway, file_buf, subway_line).unwrap_or_else(|s| panic!(s));
        } else {
            println!("Error getting filename from: {:?}", path);
            continue;
        }
    }

    let shared_subway = Arc::new(Mutex::new(subway));
    t_query::server::start(BIND_ADDR, shared_subway.clone());
}
