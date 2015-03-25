#![allow(unstable)]
use std::uint;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;

use super::{Subway, StationId, StationInfo, SubwayGraph};

/// Attempts to find a route from `start` to `end`
pub fn find_route(graph: &Subway, start: &str, end: &str) -> Result<String, String> {

    let maybe_start_id = graph.find_station(start);
    if maybe_start_id.is_err() {
        return Err(maybe_start_id.err().unwrap());
    }

    let start_id = maybe_start_id.unwrap();

    let maybe_end_id = graph.find_station(end);
    if maybe_end_id.is_err() {
        return Err(maybe_end_id.err().unwrap());
    }

    let end_id = maybe_end_id.unwrap();

    if let Some(path_ids) = find_path(graph, start_id, end_id) {
        return Ok(build_path_string(graph, path_ids));
    }
    Err(format!("No path from {} to {}", start, end))
}

fn build_path_string(graph: &Subway, path_ids: Vec<(StationId, StationInfo)>) -> String {
    let mut path_string: String = String::new();
    let mut prev_line: String = String::new();
    let mut prev_branch: String = String::new();
    for &(ref id, ref info) in path_ids.iter() {
        if let Some(n) = graph.get_station(*id) {
            // TODO: below seems brittle AF, must be DRYer way
            if prev_line.is_empty() && prev_branch.is_empty() {
                prev_line = info.line.to_string();
                prev_branch = info.branch.to_string();
            }
            if prev_branch.as_slice() != info.branch.as_slice() {
                if info.branch.as_slice() != info.line.as_slice() {
                    path_string = format!("{}---ensure you are on {}\n", path_string,
                                                                         info.branch)
                }
            }
            if prev_line.as_slice() != info.line.as_slice() {
                path_string = format!("{}---switch from {} to {}\n", path_string,
                                                                     prev_line,
                                                                     info.line)
            }

            if info.branch.as_slice() == info.line.as_slice() {
                path_string = format!("{}{}, take {}\n", path_string, n, info.line);
            } else {
                path_string = format!("{}{}, take {}\n", path_string, n, info.branch);
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

        // Important as we may have already found a better way
        if cost > dist[current] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        let connections = graph.get_connections(current).unwrap();
        for connection in connections.iter() {
            let mut c: usize = connection.cost;
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

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
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

