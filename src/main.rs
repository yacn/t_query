#![allow(unstable)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![feature(plugin)]

#[plugin]
extern crate regex_macros;
extern crate regex;

extern crate t_query;

use t_query::subway::{Subway, SubwayGraph};
use t_query::load_subway_data;
use t_query::find_route;

use std::io;
use std::os;

use std::io::{
    TcpListener,
    TcpStream,
    BufferedStream,
    File,
    Listener,
    Acceptor,
    IoError,
    IoErrorKind,
    FileStat
};
use std::thread::Thread;
use std::io::net::tcp::TcpAcceptor;

const PROMPT: &'static str = "===>>>";

const BIND_ADDR: &'static str = "127.0.0.1:12345";

fn mk_reader(s: &str) -> io::BufferedReader<io::MemReader> {
    let b = s.to_string().into_bytes();
    io::BufferedReader::new(io::MemReader::new(b))
}

fn main() {
    /*
    let listener: TcpListener = TcpListener::bind(BIND_ADDR).unwrap();
    let mut acceptor: TcpAcceptor = listener.listen().unwrap();
    for stream in acceptor.incoming() {
        match stream {
            Err(e) => {println!("error: {}", e) }
            Ok(stream) => {println!("reading")
                //let stream_buff: BufferedStream<TcpStream> = BufferedStream::new(stream);
                //Thread::spawn(move || {
                    handle_request(stream_buff)
                //});
            }
        }
    }
    drop(acceptor);*/
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
            load_subway_data(&mut subway, file_buf, subway_line);
        } else {
            println!("Error getting filename from: {:?}", path);
            continue;
        }
    }

    let route_re = regex!(r"^from (?P<from>[A-Za-z. ]+) to (?P<to>[A-Za-z. ]+)$");

    io::stdio::print(PROMPT);
    for line in io::stdin().lock().lines() {
        let input_line: String = line.unwrap();
        let maybe_caps = route_re.captures(input_line.trim());
        if maybe_caps.is_none() {
            println!("No path: {}", input_line);
            io::stdio::print(PROMPT);
            continue;
        }
        let caps = maybe_caps.unwrap();
        let from = caps.name("from").unwrap();
        let from = caps.name("from").unwrap();
        let to = caps.name("to").unwrap();
        let start_id = *(subway.get_station_id(from).unwrap());
        let end_id = *(subway.get_station_id(to).unwrap());
        let maybe_path = find_route(&subway, start_id, end_id);
        match maybe_path {
            Some(p) => println!("path from {} to {}:\n{}", from, to, p),
            None    => {
                println!("No path from {} to {}", from, to);
                io::stdio::print(PROMPT);
                continue;
            }
        }
        io::stdio::print(PROMPT);
    }

}

//fn handle_request<S: io::Stream>(stream: S) -> () {

//}
