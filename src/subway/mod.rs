#![allow(unstable)]

extern crate regex;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};


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

    /// Creates a new `Subway`
    pub fn new() -> Subway {
        Subway {
            stations: vec![],
            station_name_id_map: HashMap::new(),
            connections: vec![],
        }
    }

    /// Add the given station to the graph and returns its associated id.
    /// If the station already exists in the graph, returns the id of the
    /// existing station.
    pub fn add_station(&mut self, station: &str) -> StationId {
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

    /// Adds a connection from station `f` to station `t` on the subway line
    /// `l` and branch `b`. Depending on the line, `l` and `b` may be equal.
    /// Returns a tuple consisting of the `StationId` of the station the connection
    /// extends from and the index of the connection object in the list of connections
    /// for that station.
    pub fn add_connection(&mut self, f: StationId, t: StationId, l: &str, b: &str)
                          -> (StationId, usize) {
        let i = StationInfo { line: l.to_string(), branch: b.to_string() };
        let c = Connection { to: t, cost: 1, active: true, info: i };
        if self.connections.len() > f {
            if !self.connections[f].contains(&c) {
                self.connections[f].push(c);
                return (f, self.connections[f].len()-1);
            } else {
                let pos = self.connections[f].position_elem(&c).unwrap();
                return (f, pos);
            }
        } else {
            let cs: Vec<Connection> = vec![c];
            self.connections.push(cs);
            return (f, 0);
        }
    }

    /// Possibly retrieves the name of the station with the given id.
    pub fn get_station(&self, stn_id: StationId) -> Option<&Station> {
        self.stations.get(stn_id)
    }

    /// Possibly retrieves the id for the given station.
    pub fn get_station_id(&self, station: &str) -> Option<&StationId>{
        self.station_name_id_map.get(station)
    }

    /// Attempts to find the station with the given name.
    pub fn find_station(&self, stn: &str) -> Result<StationId, String> {
        let stn_re = regex::Regex::new(stn).unwrap();
        // build list of stations whose name matches `stn`
        let stns = self.station_name_id_map.iter()
                                           .filter(|&(ref s, _)| stn_re.is_match(s.as_slice()))
                                           .map(|(_, id)| *id);

        let mut lo_stns: Vec<StationId> = stns.collect();

        if lo_stns.len() > 1 {
            let mut emsg = "disambiguate your destination:".to_string();
            for s in lo_stns.iter() {
                let station = self.get_station(*s).unwrap();
                emsg.push_str(" "); emsg.push_str(station.as_slice());
            }
            return Err(emsg);
        } else if lo_stns.len() == 0 {
            return Err(format!("No such station: {}\n", stn));
        }

        let stn_id = lo_stns.pop();
        Ok(stn_id.unwrap())
    }

    /// Possibly retrieves the list of `Connection`s for the given station.
    pub fn get_connections(&self, from: StationId) -> Option<&Vec<Connection>> {
        self.connections.get(from)
    }

    /// Possibly retrieves a mutable references to the `Vec` of `Connection`s for the
    /// station with the given id
    fn get_connections_mut<'a>(&'a mut self, from: StationId) -> Option<&'a mut Vec<Connection>> {
        self.connections.get_mut(from)
    }

    /// Possibly retrieves the `Connection` from station `from` to station `to`.
    pub fn get_connection(&self, from: StationId, to: StationId) -> Option<&Connection> {
        if let Some(connections) = self.get_connections(from) {
            return connections.iter().find(|c| c.to == to);
        }
        None
    }

    /// Possibly returns a mutable reference to the `Connection` from the station
    /// with id `from` to the station with id `to`.
    fn get_connection_mut<'a>(&'a mut self, from: StationId, to: StationId)
                              -> Option<&'a mut Connection> {
        if let Some(connections) = self.get_connections_mut(from) {
            for c in connections.iter_mut() {
                if c.to == to { return Some(c) }
            }
        }
        None
    }

    /// Returns the number of stations in the graph
    pub fn size(&self) -> usize { self.stations.len() }

    /// Sets connections from and to the station with the given id 
    /// to the state given by `active`
    fn set_station_state(&mut self, stn_id: StationId, active: bool) {
        let mut inbound_stations: Vec<StationId> = vec![];
        // set outbound connections to `active`
        if let Some(outbound_connections) = self.get_connections_mut(stn_id) {
            for outbound_connection in outbound_connections.iter_mut() {
                inbound_stations.push(outbound_connection.to.clone());
                outbound_connection.active = active;
            }
        }
        for inbound in inbound_stations.iter() {
            if let Some(c) = self.get_connection_mut(*inbound, stn_id) {
                c.active = active;
            }
        }
    }

    /// Disables the station with the given id, meaning all connections *from* the station with
    /// id `stn_id` are marked `active: false` as well as all connections *to* `stn_id`.
    pub fn disable_station(&mut self, stn_id: StationId) { self.set_station_state(stn_id, false); }

    /// Enables the station with the given id, meaning all connections *from* the station with
    /// id `stn_id` are marked `active: true` as well as all connections *to* `stn_id`.
    pub fn enable_station(&mut self, stn_id: StationId) { self.set_station_state(stn_id, true); }

    /// Prints a list of stations and their ids.
    pub fn print_stations(&self) -> () {
        println!("StationID\tStation");
        for (sid, stn) in self.stations.iter().enumerate() {
            println!("{}\t{}", sid, stn);
        }
    }
}

#[cfg(test)]
mod subway_tests {
    use super::Subway;

    #[test]
    fn test_add_station() {
        let mut subway = Subway::new();
        let a_id = subway.add_station("A");
        assert_eq!(a_id, 0);
        assert_eq!(subway.stations[a_id], "A");
        let b_id = subway.add_station("B");
        let dupe_id = subway.add_station("A");
        assert_eq!(a_id, dupe_id);
    }

    #[test]
    fn test_add_connection() {
        let mut subway = Subway::new();
        let a_id = subway.add_station("A");
        let b_id = subway.add_station("B");
        let (from_id, conn_idx) = subway.add_connection(a_id, b_id, "foo", "bar");
        assert_eq!(from_id, a_id);
        assert_eq!(conn_idx, 0);
        let c = &subway.connections[a_id][conn_idx];
        assert!(c.active);
        assert_eq!(c.cost, 1);
        assert_eq!(c.to, b_id);
        assert_eq!(c.info.line.as_slice(), "foo");
        assert_eq!(c.info.branch.as_slice(), "bar");
    }

    #[test]
    fn test_get_station() {
        let mut subway = Subway::new();
        let a_id = subway.add_station("A");
        let maybe_a = subway.get_station(a_id);
        assert!(maybe_a.is_some());
        assert_eq!(maybe_a.unwrap().as_slice(), "A");
        let definitely_none = subway.get_station(100);
        assert!(definitely_none.is_none());
    }

    #[test]
    fn test_get_station_id() {
        let mut subway = Subway::new();
        let a_id = subway.add_station("A");
        let also_a_id = subway.get_station_id("A");
        assert!(also_a_id.is_some());
        assert_eq!(&a_id, also_a_id.unwrap());
        let none = subway.get_station_id("B");
        assert!(none.is_none());
    }

    #[test]
    fn test_find_station() {
        let mut subway = Subway::new();
        let a1_id = subway.add_station("A1");
        let a2_id = subway.add_station("A2");

        let a1_res = subway.find_station("A1");
        assert!(a1_res.is_ok());
        assert_eq!(a1_id, a1_res.unwrap());

        let multi_a = subway.find_station("A");
        assert!(multi_a.is_err());
        let multi_a_emsg = multi_a.unwrap_err();
        assert!(multi_a_emsg.starts_with("disambiguate your destination"));

        let no_b = subway.find_station("B");
        assert!(no_b.is_err());
        let no_b_emsg = no_b.unwrap_err();
        assert!(no_b_emsg.starts_with("No such station"));
    }

    #[test]
    fn test_get_connections() {
        let mut subway = Subway::new();
        let a_id = subway.add_station("A");
        let b_id = subway.add_station("B");
        let (from_id, conn_idx) = subway.add_connection(a_id, b_id, "foo", "bar");
        let maybe_cs = subway.get_connections(from_id);
        assert!(maybe_cs.is_some());
        let cs = maybe_cs.unwrap();
        assert!(cs.len() == 1);
        assert_eq!(cs[0].to, b_id);

        let no_cs = subway.get_connections(b_id);
        assert!(no_cs.is_none());
    }

    #[test]
    fn test_get_connection() {
        let mut subway = Subway::new();
        let a_id = subway.add_station("A");
        let b_id = subway.add_station("B");
        let c_id = subway.add_station("C");
        subway.add_connection(a_id, b_id, "foo", "bar");

        let maybe_c = subway.get_connection(a_id, b_id);
        assert!(maybe_c.is_some());
        let c = maybe_c.unwrap();
        assert_eq!(c.to, b_id);

        let no_c = subway.get_connection(a_id, c_id);
        assert!(no_c.is_none());
    }

    #[test]
    fn test_size() {
        let mut subway = Subway::new();
        assert!(subway.size() == 0);
        let a_id = subway.add_station("A");
        assert!(subway.size() == 1);
        let b_id = subway.add_station("B");
        assert!(subway.size() == 2);
        let c_id = subway.add_station("C");
        assert!(subway.size() == 3);
    }

    #[test]
    fn test_disable_enable_station() {
        let mut subway = Subway::new();
        let a_id = subway.add_station("A");
        let b_id = subway.add_station("B");
        let c_id = subway.add_station("C");
        subway.add_connection(a_id, b_id, "", "");
        subway.add_connection(b_id, a_id, "", "");
        subway.add_connection(c_id, b_id, "", "");

        subway.disable_station(a_id);
        {
            let a_b = subway.get_connection(a_id, b_id).unwrap();
            assert_eq!(a_b.active, false);
            let b_a = subway.get_connection(b_id, a_id).unwrap();
            assert_eq!(b_a.active, false);

            let c_b = subway.get_connection(c_id, b_id).unwrap();
            assert_eq!(c_b.active, true);
        }

        subway.enable_station(a_id);
        {
            let a_b = subway.get_connection(a_id, b_id).unwrap();
            assert_eq!(a_b.active, true);
            let b_a = subway.get_connection(b_id, a_id).unwrap();
            assert_eq!(b_a.active, true);
        }

    }

}
