#![allow(unstable)]
use std::io;
use std::fmt;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn mk_reader(s: &str) -> io::BufferedReader<io::MemReader> {
    let b = s.to_string().into_bytes();
    io::BufferedReader::new(io::MemReader::new(b))
}

fn main() {

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
    read_graph(&mut subway, blue_reader);
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
       read_graph(&mut subway, red_reader);
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
       read_graph(&mut subway, orange_reader);
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
    read_graph(&mut subway, green_reader);
    println!("");
    //subway.print_stations();
    subway.print_station_connections("Downtown Crossing Station");
    subway.print_station_connections("Park Street Station");
    subway.print_station_connections("North Station");
    subway.print_station_connections("Forest Hills Station");
    subway.print_station_connections("Reservoir Station");
    subway.print_station_connections("Wollaston Station");
    subway.print_station_connections("Lechmere Station");
    subway.print_station_connections("Alewife Station");
    let start_id = subway.get_station_id("Boylston Street Station").unwrap();
    let end_id = subway.get_station_id("Northeastern University Station").unwrap();
    let maybe_path = find_path(&subway, start_id, end_id);
    match maybe_path {
        Some(p) => println!("path: {}", p),
        None    => println!("no path"),
    }
}

fn read_graph<R: Reader>(mut subway: &mut Subway, mut content: io::BufferedReader<R>) {
    let mut subway_line: String = String::new();
    let mut subway_branch: String = String::new();
    let mut prev_stn_id: Option<usize> = None;
    for l in content.lines() {
        let line = l.unwrap();
        let to_trim: &[_] = &['-', ' '];
        
        let id_line: bool = line.starts_with("--- ");
        if id_line {
            //print!("\nbranch converge: {}\n", line);
            subway_line = line.trim_left_matches(to_trim).trim().to_string();
            continue;
        }
        
        let is_branch_line: bool = line.starts_with("----");
        
        if is_branch_line {
            subway_branch = line.trim_left_matches(to_trim).trim().to_string();
            print!("\n");
            //println!("on branch: {}", subway_branch);
            continue;
        } else {
            let station: Station = Station::new(line.trim(), subway_line.as_slice(), subway_branch.as_slice());
            let stn_id: StationId = subway.add_station(station).unwrap();
            if let Some(sid) = prev_stn_id {
                subway.add_connection(sid, stn_id);
            }
            prev_stn_id = Some(stn_id);
            //println!("Added {}: {}", line.trim(), stn_id)
        }
    }
}

type StationId = usize;
struct Subway {
    stations: HashMap<StationId, Station>,
    station_id_map: HashMap<Station, StationId>,
    connections: HashMap<StationId, HashSet<Connection>>,
}

#[derive(Copy, Eq, PartialEq)]
struct Connection {
    stn_id: StationId,
    weight: usize,
}

impl Subway {
    fn new() -> Subway {
        Subway {
            stations: HashMap::new(),
            station_id_map: HashMap::new(),
            connections: HashMap::new(),
        }
    }
    
    fn add_station(&mut self, station: Station) -> Option<StationId> {
        match self.get_station_id(station.name.as_slice()) {
            Some(id) => {
                match self.stations.entry(id) {
                    Occupied(mut en) => {
                        let mut stn = en.get_mut();
                        stn.line.push_all(station.line.as_slice());
                        return Some(id);
                    },
                    Vacant(mut en) => { None },
                }
            },
            None => {
                let stn_id: StationId = self.stations.len();
                self.stations.insert(stn_id, station);
                return Some(stn_id);
            }
        }
    }

    fn get_station_id(&self, station: &str) -> Option<StationId>{
        let mut pairs = self.stations.iter().filter(|&(id, s)| s.name.as_slice() == station);
        match pairs.next() {
            Some((sid, _)) => Some(*sid),
            None => None,
        }
    }

    fn get_station_from_id(&self, stn_id: StationId) -> Option<Station> {
        self.stations.get(&stn_id)
    }
    
    fn add_connection_single(&mut self, from: StationId, to: StationId) -> () {
        let from_stn = self.get_station_from_id(from).unwrap();
        let to_stn = self.get_station_from_id(to).unwrap();
        let from_line = from_stn.line;
        let to_line = to_stn.line;
        let no_line_transfer = from_line.iter().all(|s| to_line.contains(s));
        let connection_cost = if no_line_transfer { 1 } else { 2 };
        let connection = Connection { stn_id: to, cost: connection_cost };
        match self.connections.entry(from) {
            Occupied(mut en) => {
                en.get_mut().insert(connection);
            },
            Vacant(mut en) => {
                let mut hs: HashSet<Connection> = HashSet::new();
                hs.insert(connection);
                en.insert(hs);
            },
        }
    }
    fn add_connection(&mut self, from: StationId, to: StationId) -> () {
        self.add_connection_single(from, to);
        self.add_connection_single(to, from);
    }

    fn get_station_name(&self, stn_id: StationId) -> Option<String> {
        match self.stations.get(&stn_id) {
            Some(stn) => Some(stn.name.to_string()),
            None      => None
        }
    }

    fn get_connections_for_id(&self, from: StationId) -> Option<&HashSet<Connection>> {
        self.connections.get(&from)
    }
    
    fn print_stations(&self) {
        for (sid, stn) in self.stations.iter() {
            println!("{} {}", sid, stn);
        }
    }
    fn print_station_connections(&self, station: &str) {
        let sid = self.get_station_id(station).unwrap();
        let root = self.stations.get(&sid).unwrap();
        match self.connections.get(&sid) {
            Some(cs) => {
                print!("{}: {{", station);
                for line in root.line.iter() {
                    print!("{}, ", line);
                }
                let br = match root.branch {
                    Some(ref b) => b.to_string(),
                    None    => String::new(),
                };
                print!(" {} }}\n", br);
                print!("\t[");
                for c in cs.iter() {
                    match self.stations.get(c.stn_id) {
                        Some(stn) => {
                            let special_names = ["South Station", "North Station"];
                            let prnt_name =
                                if special_names.iter().any(|&s| stn.name.as_slice() == s) {
                                    stn.name.to_string()
                                } else {
                                    stn.name.replace(" Station", "")
                                };
                            print!("{} ({}), ", prnt_name, c.stn_id);
                        },
                        None => print!("({})", c.stn_id)
                    }

                }
                print!("]\n");
            },
            None => { },
        }
    }
}

fn find_path(graph: &Subway, start: StationId, end: StationId) -> Option<String> {
   // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = range(0, adj_list.len()).map(|_| uint::MAX).collect(); 
    let mut current: StationId = start;
    let mut todo: Vec<StationId> = vec![];
    let mut visited: Vec<StationId> = vec![];
    loop {
        if current == end {
            visited.push(current);
            break;
        }
        let neighbors = match graph.get_connections_for_id(current) {
            Some(n) => n,
            None => {
                return None;
            }
        };
        for neighbor in neighbors.iter() {
            if visited.contains(neighbor) {
                continue;
            }
            todo.push(*neighbor);
        }
        if !visited.as_slice().contains(&current) {
            visited.push(current); 
        }
        if todo.len() == 0 { return None; }
        current = todo.remove(0);
    }
    let mut path_string: String = String::new();
    for v in visited.iter() {
        match graph.get_station_name(*v) {
            Some(n) => { path_string = format!("{}->{}", path_string, n) },
            None    => { path_string = format!("{}->{}", path_string, v) },
        }
    }
    //let path_string: String = visited.as_slice().connect("->");
    Some(path_string)
}

#[derive(PartialEq, Eq, Hash, Clone, Show)]
struct Station {
    name: String,
    line: Vec<String>,
    branch: Option<String>,
}

impl fmt::String for Station {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let l = self.line.connect(":");
        let b = match self.branch {
          Some(ref br) => br.to_string(),
          None     => String::new(),
        };
        write!(f, "{} ({}, {})", self.name, l, b)
    }
}

impl Station {
    fn new(name_: &str, line_: &str, branch_: &str) -> Station {
        Station {
            name: name_.to_string(),
            line: vec![line_.to_string()],
            branch: if branch_.len() == 0 { None } else { Some(branch_.to_string()) },
        }
    }
}



