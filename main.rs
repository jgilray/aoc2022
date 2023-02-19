// AoC 2022 day 16

use regex::Regex;
use std::collections::HashMap;

type HMap1 = HashMap<(Vec<bool>, usize, usize), usize>;
type HMap2 = HashMap<State, i32>;

// custom error type
#[derive(Debug)]
struct Error {
    details: String,
}

impl Error {
    fn new(msg: &str) -> Error {
        Error {
            details: msg.to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug, Clone)]
struct Valve {
    flow: usize,
    vout: Vec<usize>,
}

impl Valve {
    fn new() -> Self {
        Self {
            flow: 0,
            vout: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Searcher {
    remtime: i32,
    vidx: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    offvalves: Vec<usize>,
    searchers: Vec<Searcher>,
}

// recursive DFS to find the best valve turning strategy, returning the total flow achieved
// it is memoized on the state of the search (valve state, current valve, current total flow)
// note that time doesn't need to be part of the state because tflow implicitly includes it
fn dfs(
    idx: &usize,          // current location (valve index) of the search
    vvec: &Vec<Valve>,    // the valve information
    vonv: &mut Vec<bool>, // whether each valve is open (true) or closed (false)
    hm: &mut HMap1,       // memoization cache
    flowrate: usize,      // rate of flow at this time
    tflow: usize,         // total flow so far
    time: usize,
) -> usize {
    if let Some(retval) = hm.get(&(vonv.to_vec(), *idx, tflow)) {
        return *retval;
    }

    let mut retval = tflow + flowrate * (30 - time);
    if time >= 29 {
        return retval;
    }
    if vvec[*idx].flow > 0 && !vonv[*idx] {
        let newflowrate = flowrate + vvec[*idx].flow;
        let newtflow = tflow + flowrate + newflowrate;
        vonv[*idx] = true;
        for vidx in vvec[*idx].vout.iter() {
            let val = dfs(vidx, vvec, vonv, hm, newflowrate, newtflow, time + 2);
            if val > retval {
                retval = val;
            }
        }
        vonv[*idx] = false;
    }
    for vidx in vvec[*idx].vout.iter() {
        let val = dfs(vidx, vvec, vonv, hm, flowrate, tflow + flowrate, time + 1);
        if val > retval {
            retval = val;
        }
    }
    hm.insert((vonv.clone(), *idx, tflow), retval);
    retval
}

// even though the raw DFS approach worked OK for part 1, I decided to compress the graph to only look at
// the valves that actually control the flow for part 2 and redo the search algorithm (see dfs2 below)
// this function will produce a matrix that shows the time between all controlling valves
// using the floyd warshall algorithm
fn floyd(vvec: &Vec<Valve>) -> Vec<Vec<i32>> {
    let mut retval: Vec<Vec<i32>> = vec![vec![99; vvec.len()]; vvec.len()];
    for (i, v) in vvec.iter().enumerate() {
        retval[i][i] = 0;
        for j in v.vout.iter() {
            retval[i][*j] = 1;
        }
    }
    for a in 0..retval.len() {
        for b in 0..retval.len() {
            for c in 0..retval.len() {
                retval[b][c] = std::cmp::min(retval[b][c], retval[b][a] + retval[a][c]);
            }
        }
    }
    // remove stuck valves
    for i in 0..vvec.len() {
        for j in 0..vvec.len() {
            if vvec[j].flow == 0 {
                retval[i][j] = 0;
            }
        }
    }
    retval
}

// a DFS that accomodates more than one searcher
fn dfs2(cur: State, vvec: &Vec<Valve>, rvec: &Vec<Vec<i32>>, hm: &mut HMap2) -> i32 {
    if let Some(cval) = hm.get(&cur) {
        return *cval;
    }

    let mut best_so_far = 0;
    let searcher = &cur.searchers[0];
    for nv in cur.offvalves.iter() {
        let noff: Vec<usize> = cur.offvalves.iter().filter(|v| *v != nv).copied().collect();
        let remtime = searcher.remtime - rvec[searcher.vidx][*nv] - 1;
        if remtime < 0 {
            continue;
        }
        let mut nsearchers = cur.searchers.clone();
        nsearchers[0] = Searcher { remtime, vidx: *nv };
        let nstate = State {
            offvalves: noff,
            searchers: nsearchers,
        };
        let val = dfs2(nstate, vvec, rvec, hm) + (vvec[*nv].flow as i32) * remtime;
        if val > best_so_far {
            best_so_far = val;
        }
    }

    if cur.searchers.len() > 1 {
        let nsearchers = cur.searchers[1..].to_vec();
        let nstate = State {
            offvalves: cur.offvalves.clone(),
            searchers: nsearchers,
        };
        let val = dfs2(nstate, vvec, rvec, hm);
        if val > best_so_far {
            best_so_far = val;
        }
    }

    hm.insert(cur, best_so_far);
    best_so_far
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../valves.dat");
    let re =
        Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)$")?;

    let mut vhs: HashMap<String, usize> = HashMap::new();
    let mut valvevec: Vec<Valve> = vec![];
    let mut vonv: Vec<bool> = vec![];
    let mut vcount = 0_usize;

    let lines: Vec<&str> = input.split('\n').collect();

    for l in lines {
        if l.is_empty() {
            continue;
        }

        if re.is_match(l) {
            let caps = re.captures(l).unwrap();
            let vin = caps.get(1).map_or("", |m| m.as_str());
            if let std::collections::hash_map::Entry::Vacant(e) = vhs.entry(vin.to_string()) {
                e.insert(vcount);
                valvevec.push(Valve::new());
                vonv.push(false);
                vcount += 1;
            }
            let vhsc = vhs.clone();
            let in_idx = vhsc.get(&vin.to_string()).unwrap();
            let flow = caps
                .get(2)
                .map_or(0, |m| m.as_str().parse::<usize>().unwrap());
            let outs = caps.get(3).map_or("", |m| m.as_str());
            for s in outs.split(", ") {
                if let std::collections::hash_map::Entry::Vacant(e) = vhs.entry(s.to_string()) {
                    e.insert(vcount);
                    valvevec.push(Valve::new());
                    vonv.push(false);
                    vcount += 1;
                }
                let vhsc = vhs.clone();
                let out_idx = vhsc.get(&s.to_string()).unwrap();
                // fill in valve vector
                valvevec[*in_idx].flow = flow;
                valvevec[*in_idx].vout.push(*out_idx);
            }
        } else {
            return Err(Box::new(Error::new(&format!("bad line in input: {}", l))));
        }
    }

    // part 1
    let mut hm: HMap1 = HMap1::new();
    let start = vhs.get("AA").unwrap();
    let ansa = dfs(start, &valvevec, &mut vonv, &mut hm, 0, 0, 0);
    println!("aoc16a: {}", ansa);

    // part 2
    let mut hm: HMap2 = HMap2::new();
    let rvec = floyd(&valvevec);
    let start = vhs.get("AA").unwrap();
    let off: Vec<usize> = valvevec
        .iter()
        .enumerate()
        .filter(|(_, v)| v.flow > 0)
        .map(|(i, _)| i)
        .collect();
    let s1 = Searcher {
        remtime: 26,
        vidx: *start,
    };
    let s2 = Searcher {
        remtime: 26,
        vidx: *start,
    };
    let state = State {
        offvalves: off,
        searchers: vec![s1, s2],
    };

    let ansb = dfs2(state, &valvevec, &rvec, &mut hm);
    println!("aoc16b: {}", ansb);

    Ok(())
}
