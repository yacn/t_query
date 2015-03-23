#![allow(unstable)]
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

pub use self::data::load_subway_data;
pub use self::route::find_route;
pub use self::route::find_path;

pub mod data;
pub mod route;

pub type StationId = usize;
pub type Station = String;

pub struct Subway {
    pub stations: HashMap<StationId, Station>,
    pub station_id_map: HashMap<Station, StationId>,
    pub connections: HashMap<StationId, Vec<Connection>>,
}

#[derive(Eq, PartialEq, Hash, Clone, Show)]
pub struct Connection {
    pub to: StationId,
    pub cost: usize,
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
            stations: HashMap::new(),
            station_id_map: HashMap::new(),
            connections: HashMap::new(),
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

	/// Possibly retrieves the `Connection` from station `from` to station `to`.
	fn get_connection(&self, from: StationId, to: StationId) -> Option<&Connection>;

	/// Prints a list of stations and their ids.
	fn print_stations(&self) -> () { println!("Mmm... implement me you must") }
}

impl SubwayGraph for Subway {
    fn add_station(&mut self, station: &str) -> StationId {
        match self.station_id_map.entry(station.to_string()) {
            Occupied(ent) => {
                let stn_id: &StationId = ent.get();
                return *stn_id;
            },
            Vacant(ent) => {
                let stn_id: StationId = self.stations.len();
                self.stations.insert(stn_id, station.to_string());
                ent.insert(stn_id);
                return stn_id;
            },
        }
    }

    fn get_station_id(&self, station: &str) -> Option<&StationId>{
        self.station_id_map.get(&station.to_string())
    }

    fn add_connection(&mut self, f: StationId, t: StationId, l: &str, b: &str) -> () {
        let i = StationInfo { line: l.to_string(), branch: b.to_string() };
        let c = Connection { to: t, cost: 1, info: i };
        match self.connections.entry(f) {
            Occupied(mut ent) => {
                let cs = ent.get_mut();
                if !cs.contains(&c) { cs.push(c); }
            },
            Vacant(ent) => {
                let cs: Vec<Connection> = vec![c];
                ent.insert(cs);
            }
        }
    }

    fn size(&self) -> usize { self.stations.len() }

    fn get_station(&self, stn_id: StationId) -> Option<&Station> {
        self.stations.get(&stn_id)
    }

    fn get_connections(&self, from: StationId) -> Option<&Vec<Connection>> {
        self.connections.get(&from)
    }

    fn get_connection(&self, from: StationId, to: StationId) -> Option<&Connection> {
        if let Some(connections) = self.connections.get(&from) {
            return connections.iter().find(|c| c.to == to);
        }
        None
    }

    fn print_stations(&self) -> () {
        for (sid, stn) in self.stations.iter() {
            println!("{} {}", sid, stn);
        }
    }
}

