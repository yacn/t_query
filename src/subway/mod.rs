#![allow(unstable)]

extern crate regex;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};


pub use self::data::load_subway_data;
pub use self::route::find_route;
pub use self::route::find_path;

pub use self::route::Query;
pub use self::route::Query::{Enable, Disable, Route};

pub mod data;
pub mod route;

pub type StationId = usize;
pub type Station = String;


pub struct Subway {
    pub stations: Vec<Station>,
    pub station_name_id_map: HashMap<String, StationId>,
    pub connections: Vec<Vec<Connection>>,
}

#[derive(Eq, PartialEq, Hash, Clone, Show)]
pub struct Connection {
    pub to: StationId,
    pub cost: usize,
    pub active: bool,
    pub info: StationInfo,
}

#[derive(Eq, PartialEq, Hash, Clone, Show)]
pub struct StationInfo {
    pub line: String,
    pub branch: String,
}

impl Subway {
    pub fn new() -> Subway {
        Subway {
            stations: vec![],
            station_name_id_map: HashMap::new(),
            connections: vec![],
        }
    }
}

pub trait SubwayGraph {
	/// Add the given station to the graph and returns its associated id.
	/// If the station already exists in the graph, returns the id of the
	/// existing station.
	fn add_station(&mut self, station: &str) -> StationId;

	/// Possibly retrieves the id for the given station.
	fn get_station_id(&self, station: &str) -> Option<&StationId>;

	/// Possibly retrieves the name of the station with the given id.
	fn get_station(&self, stn_id: StationId) -> Option<&Station>;

	/// Adds a connection from station `f` to station `t` on the subway line
	/// `l` and branch `b`. Depending on the line, `l` and `b` may be equal.
	fn add_connection(&mut self, f: StationId, t: StationId, l: &str, b: &str) -> ();

	/// Returns the number of stations in the graph
	fn size(&self) -> usize;

	/// Possibly retrieves the list of `Connection`s for the given station.
	fn get_connections(&self, from: StationId) -> Option<&Vec<Connection>>;

    fn get_connection_mut<'a>(&'a mut self, from: StationId, to: StationId)
                              -> Option<&'a mut Connection>;

    fn find_station(&self, stn: &str) -> Result<StationId, String>;

	/// Possibly retrieves the `Connection` from station `from` to station `to`.
	fn get_connection(&self, from: StationId, to: StationId) -> Option<&Connection>;

    fn disable_station(&mut self, stn_id: StationId);

    fn enable_station(&mut self, stn_id: StationId);

	/// Prints a list of stations and their ids.
	fn print_stations(&self) -> () { println!("Mmm... implement me you must") }
}

impl SubwayGraph for Subway {
    fn add_station(&mut self, station: &str) -> StationId {
        match self.station_name_id_map.entry(station.to_string()) {
            Occupied(ent) => {
                let stn_id: &StationId = ent.get();
                return *stn_id;
            },
            Vacant(ent) => {
                let stn_id: StationId = self.stations.len();
                self.stations.push(station.to_string());
                ent.insert(stn_id);
                return stn_id;
            }
        }
    }

    fn get_station_id(&self, station: &str) -> Option<&StationId>{
        self.station_name_id_map.get(station)
    }

    fn add_connection(&mut self, f: StationId, t: StationId, l: &str, b: &str) -> () {
        let i = StationInfo { line: l.to_string(), branch: b.to_string() };
        let c = Connection { to: t, cost: 1, active: true, info: i };
        if self.connections.len() > f {
            if !self.connections[f].contains(&c) { self.connections[f].push(c); }
        } else {
            let cs: Vec<Connection> = vec![c];
            self.connections.push(cs);
        }
    }

    fn find_station(&self, stn: &str) -> Result<StationId, String> {
        let stn_re = regex::Regex::new(stn).unwrap();
        let stns = self.station_name_id_map.iter()
                                           .filter(|&(ref s, _)| stn_re.is_match(s.as_slice()))
                                           .map(|(_, id)| *id);

        let mut lo_stns: Vec<StationId> = stns.collect();

        if lo_stns.len() > 1 {
            let mut emsg = "disambiguate your destination: ".to_string();
            for s in lo_stns.iter() {
                let station = self.get_station(*s).unwrap();
                emsg.push_str(" "); emsg.push_str(station.as_slice());
            }
            return Err(emsg);
        } else if lo_stns.len() == 0 {
            return Err(format!("No such station: {}", stn));
        }

        let stn_id = lo_stns.pop();
        Ok(stn_id.unwrap())
    }

    fn size(&self) -> usize { self.stations.len() }

    fn get_station(&self, stn_id: StationId) -> Option<&Station> {
        self.stations.get(stn_id)
    }

    fn get_connections(&self, from: StationId) -> Option<&Vec<Connection>> {
        self.connections.get(from)
    }

    fn get_connection(&self, from: StationId, to: StationId) -> Option<&Connection> {
        if let Some(connections) = self.get_connections(from) {
            return connections.iter().find(|c| c.to == to);
        }
        None
    }

    fn get_connection_mut<'a>(&'a mut self, from: StationId, to: StationId)
                              -> Option<&'a mut Connection> {
        for c in self.connections[from].iter_mut() {
            if c.to == to { return Some(c) }
        }
        None
    }

    fn disable_station(&mut self, stn_id: StationId) {
        let mut inbound_stations: Vec<StationId> = vec![];
        // disable outbound connections
        for outbound_connection in self.connections[stn_id].iter_mut() {
            inbound_stations.push(outbound_connection.to.clone());
            outbound_connection.active = false;
        }
        for inbound in inbound_stations.iter() {
            if let Some(c) = self.get_connection_mut(*inbound, stn_id) {
                c.active = false;
            }
        }
    }

    fn enable_station(&mut self, stn_id: StationId) {
        let mut inbound_stations: Vec<StationId> = vec![];
        // enable outbound connections
        for outbound_connection in self.connections[stn_id].iter_mut() {
            inbound_stations.push(outbound_connection.to.clone());
            outbound_connection.active = true;
        }
        for inbound in inbound_stations.iter() {
            if let Some(c) = self.get_connection_mut(*inbound, stn_id) {
                c.active = true;
            }
        }
    }

    fn print_stations(&self) -> () {
        for (sid, stn) in self.stations.iter().enumerate() {
            println!("{} {}", sid, stn);
        }
    }
}

