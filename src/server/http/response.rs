#![allow(unstable)]

use std::io;
use std::io::File;

use std::string;
use std::io::IoErrorKind::{PermissionDenied,FileNotFound};

const DEBUG: bool = false;
const SERVER_NAME: &'static str = "IBATs_web_server";

#[derive(Copy)]
pub enum Status {
    Ok,
    BadRequest,
    Forbidden,
    NotFound,
}

#[derive(Copy)]
pub enum ContentType {
    Plain,
    Html,
}

pub struct Response<'a> {
    status: Status,
    content_type: ContentType,
    content_length: u64,
    body: &'a str,
}

impl<'a> Response<'a> {
    pub fn ok(content_type: ContentType, content_len: u64, body: &'a str) -> Response<'a> {
        Response {
            status: Status::Ok,
            content_type: content_type,
            content_length: content_len,
            body: body,
        }
    }
    pub fn bad(response_status: Status) -> Response<'a> {
        Response {
            status: response_status,
            content_type: ContentType::Plain,
            content_length: 0,
            body: ""
        }
    }
}

impl<'a> string::ToString for Response<'a> {
    fn to_string(&self) -> String {
        match self.status {
            Status::BadRequest => "HTTP/1.0 400 Bad Request\r\n".to_string(),
            Status::Forbidden  => "HTTP/1.0 403 Forbidden\r\n".to_string(),
            Status::NotFound   => "HTTP/1.0 404 Not Found\r\n".to_string(),
            Status::Ok => {
                let status_line: &str = "HTTP/1.0 200 OK";
                let content_type: &str = match self.content_type {
                    ContentType::Plain => "Content-type: text/plain",
                    ContentType::Html => "Content-type: text/html",
                };
                let content_length: String = format!("Content-Length: {}", self.content_length);
                [status_line,
                 SERVER_NAME,
                 content_type,
                 content_length.as_slice(),
                 "\r\n",
                 self.body].connect("\r\n")
            }
        }
    }
}

pub fn bad_request<S: io::Stream>(mut stream: &mut S) -> () {
    let r: String = Response::bad(Status::BadRequest).to_string();
    if DEBUG { println!("Response:\n{}", r); }
    stream.write_str(r.as_slice()).unwrap();
}

pub fn not_found<S: io::Stream>(mut stream: &mut S) -> () {
    let r: String = Response::bad(Status::NotFound).to_string();
    if DEBUG { println!("Response:\n{}", r); }
    stream.write_str(r.as_slice()).unwrap();
}

pub fn response<S: io::Stream>(mut stream: &mut S, path: &Path) -> () {
    let r: String = build_response(path);
    if DEBUG { println!("Response:\n{}", r); }
    return stream.write_str(r.as_slice()).unwrap();
}

fn build_response(path: &Path) -> String {
    match File::open(path) {
        Ok(mut f) => {
            let content_type: ContentType =
                if path.extension() == Some(b"html") { ContentType::Html }
                                                else { ContentType::Plain };

            let content_length: u64 = match f.stat() {
                Ok(info) => info.size,
                Err(_)   => 0,
            };

            let body: String = f.read_to_string().unwrap_or(String::new());

            let r: Response = Response::ok(content_type, content_length, body.as_slice());
            return r.to_string();

        },
        Err(e) => {
            let error_response = match e.kind {
                PermissionDenied => Response::bad(Status::Forbidden),
                FileNotFound => Response::bad(Status::NotFound),
                _ => Response::bad(Status::BadRequest),
            };    
            return error_response.to_string();
        }
    }
}
