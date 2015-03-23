#![allow(unstable)]
#![allow(unused_mut)]

#![allow(unused_imports)]
extern crate t_query;

use t_query::subway::{Subway, SubwayGraph};
use t_query::load_subway_data;
use t_query::find_route;

use std::io;

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

    let mut subway: Subway = Subway::new();

    let blue: &str = "--- blue 
Wonderland Station
Revere Beach Station
Beachmont Station
Suffolk Downs Station
Orient Heights Station
Wood Island Station
Airport Station
Maverick Station
Aquarium Station
State Station
Government Center Station
Bowdoin Station";
    let mut blue_reader = mk_reader(blue);
    load_subway_data(&mut subway, blue_reader, "blue");
    let red: &str = "--- Braintree Mattapan
Alewife Station
Davis Station
Porter Square Station
Harvard Square Station
Central Square Station
Kendall Station
Charles/MGH Station
Park Street Station
Downtown Crossing Station
South Station
Broadway Station
Andrew Station
JFK/UMass Station
---------- Braintree 
       North Quincy Station
       Wollaston Station
       Quincy Center Station
       Quincy Adams Station
       Braintree Station
---------- Mattapan
       Savin Hill Station
       Fields Corner Station
       Shawmut Station
       Ashmont Station
       Cedar Grove Station
       Butler Station
       Milton Station
       Central Avenue Station
       Valley Road Station
       Capen Street Station
       Mattapan Station";
       let mut red_reader = mk_reader(red);
       load_subway_data(&mut subway, red_reader, "red");
       let orange: &str = "--- orange
Oak Grove Station
Malden Center Station
Wellington Station
Assembly Station
Sullivan Square Station
Community College Station
North Station
Haymarket Station
State Station
Downtown Crossing Station 
Chinatown Station
Tufts Medical Center Station
Back Bay Station
Massachusetts Avenue Station
Ruggles Station
Roxbury Crossing Station
Jackson Square Station
Stony Brook Station
Green Street Station
Forest Hills Station";
       let mut orange_reader = mk_reader(orange);
       load_subway_data(&mut subway, orange_reader, "orange");
       let green: &str = "--- B C D E 
Lechmere Station
Science Park Station
North Station
Haymarket Station
Government Center Station
Park Street Station
Boylston Street Station
Arlington Station
Copley Station
------------------------ E
             Prudential Station
             Symphony Station
             Northeastern University Station
             Museum of Fine Arts Station
             Longwood Medical Area Station
             Brigham Circle Station
             Fenwood Road Station
             Mission Park Station
             Riverway Station
             Back of the Hill Station
             Heath Street Station
--- B C D
Hynes Convention Center
Kenmore Station
---------- C
       St. Marys Street Station
       Hawes Street Station
       Kent Street Station
       St. Paul Street
       Coolidge Corner Station
       Summit Avenue Station
       Brandon Hall Station
       Fairbanks Station
       Washington Square Station
       Tappan Street Station
       Fenway Station
       Dean Road Station
       Englewood Avenue Station
       Cleveland Circle Station
--------------------- D 
              Longwood Station
              Brookline Village Station
              Brookline Hills Station
              Beaconsfield Station
              Reservoir Station
              Chestnut Hill Station D Riverside Line
              Newton Centre Station
              Newton Highlands Station
              Eliot Station
              Waban Station
              Woodland Station
              Riverside Station
-------------------------------- B 
                 Blandford Street Station
                 Boston University East Station
                 Boston University Central Station
                 Boston University West Station
                 St. Paul Street
                 Pleasant Street Station
                 Babcock Street Station
                 Packards Corner Station
                 Harvard Avenue Station
                 Griggs Street/Long Avenue Station
                 Allston Street Station
                 Warren Street Station
                 Washington Street Station
                 Sutherland Road Station 
                 Chiswick Road Station
                 Chestnut Hill Avenue Station
                 South Street Station
                 Boston College Station";
    let green_reader = mk_reader(green);
    load_subway_data(&mut subway, green_reader, "green");

    let start: &str = "Airport Station";

    let end: &str = "Coolidge Corner Station";
    let start_id = *(subway.get_station_id(start).unwrap());
    let end_id = *(subway.get_station_id(end).unwrap());
    let maybe_path = find_route(&subway, start_id, end_id);
    match maybe_path {
        Some(p) => println!("path from {} to {}:\n{}", start, end, p),
        None    => println!("no path"),
    }
}

//fn handle_request<S: io::Stream>(stream: S) -> () {

//}
