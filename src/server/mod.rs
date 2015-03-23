#![allow(unstable)]

use std::os;
use std::io;

use std::io::fs::PathExtensions;

use self::http::RequestLine;

pub mod http;

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
