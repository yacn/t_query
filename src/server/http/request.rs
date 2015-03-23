#![allow(unstable)]

use std::ascii::AsciiExt;
use std::str::Split;
use std::string::ToString;

pub struct RequestLine<'a> {
    pub verb: &'a str,
    pub path: &'a str,
    pub version: &'a str,
}

impl<'a> RequestLine<'a> {

    pub fn new(m: &'a str, fp: &'a str, v: &'a str) -> RequestLine<'a> {
        RequestLine { verb: m.trim(), path: fp.trim(), version: v.trim(), }
    }

    pub fn from_str(s: &'a str) -> Option<RequestLine<'a>> {
        let mut splits = s.split(' ');
        // request line should be: <VERB> <PATH> HTTP/1.0, otherwise malformed
        let malformed_request_line = |&: s: Split<char>| { s.count() != 3 };
        if malformed_request_line(splits.clone()) { return None; }

        let method: &str = splits.next().unwrap();
        let file: &str = splits.next().unwrap();
        let http_version: &str = splits.next().unwrap();

        let request_line: RequestLine = RequestLine::new(method, file, http_version);

        if request_line.valid() { Some(request_line) } else { None }
    }

    pub fn valid(&self) -> bool {
        fn valid_method(m: &str) -> bool {
            match m.to_ascii_uppercase().as_slice() {
                "GET" => true,
                _     => false,
            }
        }

        let valid_path: bool = self.path.starts_with("/");

        fn valid_version(v: &str) -> bool {
            match v {
                "HTTP/0.9" |
                "HTTP/1.0" |
                "HTTP/1.1" | "HTTP" => true,
                _                   => false,
            }
        }

        valid_method(self.verb) && valid_path && valid_version(self.version)

    }
}

impl<'a> ToString for RequestLine<'a> {
    fn to_string(&self) -> String {
        let s: String = format!("RequestLine[{} {} {}]", self.verb, self.path, self.version);
        s
    }
}
