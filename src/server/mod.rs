#![allow(unstable)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![feature(plugin)]

#[plugin]
extern crate regex_macros;
extern crate regex;

use std::os;
use std::io;

use std::io::fs::PathExtensions;
use std::sync::{Arc, Mutex};

use std::sync::mpsc::{sync_channel, channel, Sender};

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

use self::http::RequestLine;

use super::find_route;
use super::subway::{Subway, SubwayGraph, Query};
use super::subway::Query::{Route, Enable, Disable};

// chosen arbitrarily
const MAX_QUERY_LENGTH: usize = 1024;

pub mod http;

pub fn start(bind_addr: &str, shared_subway: Arc<Mutex<Subway>>) {
    type Message = (Result<Query, String>, Sender<String>);
    // create rendezvous channel for queries and results
    let (queue_back, queue_front) = sync_channel::<Message>(0);

    // query handler
    {
        let shared_subway = shared_subway.clone();
        Thread::spawn(move||{
            loop {
                let (maybe_query, results_chan) = queue_front.recv().unwrap();
                match maybe_query {
                    Ok(q) => match q {
                        Query::Route(from, to) => {
                            println!("route qry from {} to {}", from, to);
                            let subway = shared_subway.lock().unwrap();
                            match find_route(&*subway, from.as_slice(), to.as_slice()) {
                                Ok(p) => {
                                    println!("found path from {} to {}", from, to);
                                    results_chan.send(p).unwrap();
                                },
                                Err(e) => {
                                    println!("err findng path: {}", e);
                                    results_chan.send(e).unwrap();
                                },
                            }
                        },
                        Query::Enable(stn) => {
                            let mut subway = shared_subway.lock().unwrap();
                            println!("enabling {}", stn);
                            subway.enable_station(stn);
                            results_chan.send("done".to_string()).unwrap();
                        },
                        Query::Disable(stn) => {
                            println!("disabling {}", stn);
                            let mut subway = shared_subway.lock().unwrap();
                            subway.disable_station(stn);
                            results_chan.send("done".to_string()).unwrap();
                        }
                    },
                    Err(e) => {
                        println!("err in qry: {}", e);
                        results_chan.send(e).unwrap();
                    },
                }
            }
        });
    }

    let listener: TcpListener = TcpListener::bind(bind_addr).unwrap();
    let mut acceptor: TcpAcceptor = listener.listen().unwrap();
    for stream in acceptor.incoming() {
        match stream {
            Err(e) => { println!("error: {}", e) }
            Ok(stream) => {
                let queue_back = queue_back.clone();
                let mut streambuf: BufferedStream<TcpStream> = BufferedStream::new(stream);
                let shared_subway = shared_subway.clone();
                Thread::spawn(move || {
                    let mut buf: [u8; MAX_QUERY_LENGTH] = [0; MAX_QUERY_LENGTH];
                    let bytes_read: usize = streambuf.read(&mut buf).unwrap();

                    println!("qry recvd: {}\n", ::std::str::from_utf8(&buf).unwrap());

                    let query_bytes: &[u8] = buf.slice_to(bytes_read);
                    let query_str: &str = ::std::str::from_utf8(query_bytes).unwrap();

                    let (done_send, done_recv) = channel::<String>();

                    let subway = shared_subway.lock().unwrap();
                    let query = Query::new(&*subway, query_str.trim());
                    drop(subway);

                    queue_back.send((query, done_send)).unwrap();

                    let results: String = done_recv.recv().unwrap();
                    println!("got results: {}\n", results);
                    streambuf.write_str(results.as_slice());
                });
            }
        }
    }
}



pub fn respond<S: io::Stream>(mut stream: &mut S, request_line: &str) -> () {
    let maybe_request_line: Option<RequestLine> = RequestLine::from_str(request_line);

    if maybe_request_line.is_none() { return http::bad_request(stream) }

    let rline: RequestLine = maybe_request_line.unwrap();

    let cwd = os::getcwd().unwrap();
    
    // remove the initial `/' before joining so path doesn't think file is absolute path
    let path = cwd.join(rline.path.slice_from(1));

    let check_for: [&str; 4] = [path.as_str().unwrap_or(""), "index.html",
                                "index.shtml", "index.txt"];

    let maybe_path: Option<Path> = check_for.iter()
                                            .map(|&: s| path.join(*s))
                                            .filter(|&: p| p.exists() &&
                                                          !p.is_dir()).next();
    match maybe_path {
        Some(p) => { return http::response(stream, &p); },
        None    => { return http::not_found(stream); },
    }
}
