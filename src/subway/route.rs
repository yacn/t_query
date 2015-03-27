#![allow(unstable)]


#[plugin]
extern crate regex_macros;
extern crate regex;

use std::uint;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;

use super::{Subway, StationId, StationInfo};

const DISABLE_COST: usize = 100;

// so rustc shushes
#[derive(Copy)]
pub enum Query {
    Route(StationId, StationId),
    Enable(StationId),
    Disable(StationId),
}

impl Query {
    pub fn new(subway: &Subway, line: &str) -> Result<Query, String> {
        let route_re: regex::Regex = regex!(r"^from (?P<from>.+) to (?P<to>.+)$");
        let disable_re: regex::Regex = regex!(r"^disable (?P<station>.+)$");
        let enable_re: regex::Regex  = regex!(r"^enable (?P<station>.+)$");

        if route_re.is_match(line) {
            let caps = route_re.captures(line).unwrap();
            let to = caps.name("to").unwrap();
            let from = caps.name("from").unwrap();
            match subway.find_station(from) {
                Ok(from_id) => {
                    match subway.find_station(to) {
                        Ok(to_id) => { return Ok(Query::Route(from_id, to_id)); },
                        Err(e)    => { return Err(e); }
                    }
                },
                Err(e) => { return Err(e); },
            }
        }

        if disable_re.is_match(line) {
            let caps = disable_re.captures(line).unwrap();
            let stn = caps.name("station").unwrap();
            match subway.find_station(stn) {
                Ok(sid) => { return Ok(Query::Disable(sid)); },
                Err(e)  => { return Err(e); },
            }
        }

        if enable_re.is_match(line) {
            let caps = enable_re.captures(line).unwrap();
            let stn = caps.name("station").unwrap();
            match subway.find_station(stn) {
                Ok(sid) => { return Ok(Query::Enable(sid)); },
                Err(e)  => { return Err(e); },
            }
        }
        let emsg = format!("unable to parse query: {}", line);
        Err(emsg)
    }

    pub fn is_route(&self) -> bool {
        match *self {
            Query::Route(_, _) => true,
            _ => false,
        }
    }

    pub fn is_disable(&self) -> bool {
        match *self {
            Query::Disable(_) => true,
            _ => false,
        }
    }

    pub fn is_enable(&self) -> bool {
        match *self {
            Query::Enable(_) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod query_tests {
    use super::Query;
    use super::Query::{Route, Enable, Disable};
    use super::super::{Subway, StationId, StationInfo};

    #[test]
    fn test_new() {
        let mut subway = Subway::new();
        let a_id = subway.add_station("A");
        let b_id = subway.add_station("B");
        let c_id = subway.add_station("C");

        let route_str = "from A to B";
        let maybe_route = Query::new(&subway, route_str);
        assert!(maybe_route.is_ok());
        let route = maybe_route.unwrap();
        assert!(route.is_route());
        //assert_eq!(Query::Route(0, 1), route);

        let no_route = Query::new(&subway, "from A to D");
        assert!(no_route.is_err());
        let no_route_msg = no_route.err().unwrap();
        assert_eq!(no_route_msg.as_slice(), "No such station: D");

        let dis_a = Query::new(&subway, "disable A");
        assert!(dis_a.is_ok());
        let disable_a = dis_a.unwrap();
        assert!(disable_a.is_disable());

        let en_a = Query::new(&subway, "enable A");
        assert!(en_a.is_ok());
        let enable_a = en_a.unwrap();
        assert!(enable_a.is_enable());
    }
}


/// Attempts to find a route from `start` to `end`
pub fn find_route(graph: &Subway, start: StationId, end: StationId) -> Result<String, String> {
    if let Some(path_ids) = find_path(graph, start, end) {
        return Ok(build_path_string(graph, path_ids));
    }
    Err(format!("No path from {} to {}", start, end))
}



fn build_path_string(graph: &Subway, path_ids: Vec<(StationId, StationInfo)>) -> String {
    let mut path_string: String = String::new();
    let mut prev_line: String = String::new();
    let mut prev_branch: String = String::new();
    for &(ref id, ref info) in path_ids.iter() {
        if let Some(stn) = graph.get_station(*id) {
            // TODO: below seems brittle AF, must be DRYer way
            if prev_line.is_empty() && prev_branch.is_empty() {
                prev_line = info.line.to_string();
                prev_branch = info.branch.to_string();
            }
            if prev_branch.as_slice() != info.branch.as_slice() {
                if info.branch.as_slice() != info.line.as_slice() {
                    for &s in ["---ensure you are on ", info.branch.as_slice(), "\n"].iter() {
                        path_string.push_str(s);
                    }
                }
            }
            if prev_line.as_slice() != info.line.as_slice() {
                let strs = ["---switch from ",
                            prev_line.as_slice(),
                            " to ",
                            info.line.as_slice(),
                            "\n"];
                for &s in strs.iter() { path_string.push_str(s); }
            }

            if info.branch.as_slice() == info.line.as_slice() {
                for &s in [stn.as_slice(), ", take ", info.line.as_slice(), "\n"].iter() {
                    path_string.push_str(s);
                }
            } else {
                for &s in [stn.as_slice(), ", take ", info.branch.as_slice(), "\n"].iter() {
                    path_string.push_str(s);
                }
            }
            prev_line = info.line.to_string();
            prev_branch = info.branch.to_string();
        }
    }
    return path_string;
}

#[derive(Copy, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Implmentation of Dijkstra's algorithm to find the shortest path.
/// based on implementation in Rust documentation:
/// http://doc.rust-lang.org/1.0.0-alpha/collections/binary_heap/index.html
pub fn find_path(graph: &Subway, start: StationId, end: StationId)
				 -> Option<Vec<(StationId, StationInfo)>> {

    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = range(0, graph.size()).map(|_| uint::MAX).collect();
    let mut heap = BinaryHeap::new();
    let mut came_from: HashMap<StationId, (StationId, StationInfo)> = HashMap::new();
    let mut prev_stn: Option<StationInfo> = None;

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position: current }) = heap.pop() {
        if current == end { return Some(reconstruct_path(came_from, end)); }

        if cost > dist[current] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        let connections = graph.get_connections(current).unwrap();
        for connection in connections.iter() {
            let mut c: usize = connection.cost;
            if !connection.active  { c += DISABLE_COST; }
            if let Some(prev_info) = prev_stn {
                let prev_line = prev_info.line.as_slice();
                let prev_branch = prev_info.branch.as_slice();
                let cur_line = connection.info.line.as_slice();
                let cur_branch = connection.info.branch.as_slice();
                // Line transfers considered heaviest cost
                if prev_line != cur_line { c = 3; }
                // branch transfers not as heavy
                else if  prev_branch != cur_branch { c = 2; }
            }
            prev_stn = Some(connection.info.clone());
            let next = State { cost: cost + c, position: connection.to };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
                came_from.insert(next.position, (current, connection.info.clone()));
            }
        }
    }
    None
}

/// Retrace steps to build the path that was found as a list of nodes
fn reconstruct_path(came_from: HashMap<StationId, (StationId, StationInfo)>, goal: StationId)
					-> Vec<(StationId, StationInfo)> {

    let mut total_path: Vec<(StationId, StationInfo)> = vec![];
    let &(_, ref i) = came_from.get(&goal).unwrap();
    total_path.push((goal, i.clone()));
    let mut current: Option<&(StationId, StationInfo)> = came_from.get(&goal);
    while let Some(&(ref id, ref info)) = current {
        total_path.push((*id, info.clone()));
        current = came_from.get(id);
    }
    total_path.reverse();
    return total_path;
}

#[test]
fn test_find_path() {
    let mut subway = Subway::new();
    let a_id = subway.add_station("A");
    let b_id = subway.add_station("B");
    let c_id = subway.add_station("C");
    let d_id = subway.add_station("D");

    subway.add_connection(a_id, b_id, "", "");
    subway.add_connection(a_id, d_id, "", "");

    subway.add_connection(b_id, a_id, "", "");
    subway.add_connection(b_id, d_id, "", "");

    subway.add_connection(d_id, c_id, "", "");

    let maybe_route = find_path(&subway, a_id, d_id);
    assert!(maybe_route.is_some());
    let route = maybe_route.unwrap();

    let route: Vec<StationId> = route.iter().map(|&(id, _)| id).collect();
    assert_eq!(route, vec![0, 3]);
}

