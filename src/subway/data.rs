#![allow(unstable)]
use std::io;

use super::{Subway, StationId};

pub fn load_subway_data<R: Reader>(mut subway: &mut Subway,
                                   mut content: io::BufferedReader<R>,
                                   tline: &str) -> Result<(), String> {

    let mut subway_branch: String = tline.to_string();
    let mut prev_stn_id: Option<usize> = None;
    let mut pre_branch_stn: Option<usize> = None;
    let mut in_branch: bool = false;
    let mut just_branched: bool = false;

    let to_trim: &[_] = &['-', ' '];

    let mut lines = content.lines();
    // TODO: use for graph validation, i.e. all branches listed appear
    let branch_list_line = lines.next().unwrap().unwrap()
                                                .trim_left_matches(to_trim)
                                                .trim()
                                                .to_string();
    let num_branches: usize = branch_list_line.split(' ').count();
    let mut seen_branches: usize = 0;
    for l in lines {
        let line = l.unwrap();
        
        let branch_converge_line: bool = line.starts_with("--- ");
        if branch_converge_line {
            subway_branch = line.trim_left_matches(to_trim).trim().to_string();
            in_branch = false;
            just_branched = false;
            prev_stn_id = pre_branch_stn;
            pre_branch_stn = None;
            continue;
        }
        
        let single_branch_line: bool = line.starts_with("----");
        if single_branch_line {
            in_branch = true;
            just_branched = true;
            seen_branches += 1;
            subway_branch = line.trim_left_matches(to_trim).trim().to_string();
            continue;
        }

        let mut station: String  = line.trim().to_string();
        // FIXME: this is a hack, need general solution to stations w/ same name but diff stn.
        if station.as_slice() == "St. Paul Street" {
            station.push_str(" "); station.push_str(subway_branch.as_slice());
        }
        let stn_id: StationId = subway.add_station(station.as_slice());

        if !in_branch { pre_branch_stn = Some(stn_id); }

        if in_branch && just_branched {
            if let Some(id) = pre_branch_stn {
                subway.add_connection(id, stn_id, tline, subway_branch.as_slice());
                subway.add_connection(stn_id, id, tline, subway_branch.as_slice());
            }
            just_branched = false;
            prev_stn_id = Some(stn_id);
            continue;
        }
        if let Some(sid) = prev_stn_id {
            subway.add_connection(sid, stn_id, tline, subway_branch.as_slice());
            subway.add_connection(stn_id, sid, tline, subway_branch.as_slice());
        }
        prev_stn_id = Some(stn_id);
    }
    if (num_branches != 1) && (seen_branches != num_branches) {
        Err(format!("More branches defined in header than appear in graph data for {}!", tline))
    } else {
        Ok(())
    }
}