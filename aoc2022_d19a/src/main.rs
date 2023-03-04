// AoC 2022 day 15

use regex::Regex;
use std::collections::HashMap;

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

#[derive(Default, Debug, Clone, Eq, Hash, PartialEq)]
// state of the cavern
struct State {
    remtime: u16,
    ore_amt: u16,
    clay_amt: u16,
    obs_amt: u16,
    geo_amt: u16,
    ore_robot_num: u16,
    clay_robot_num: u16,
    obs_robot_num: u16,
    geo_robot_num: u16,
}

impl State {
    fn new(remtime: u16) -> Self {
        Self {
            remtime,
            ore_robot_num: 1,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
// state of the cavern
struct Blueprint {
    id: u16,
    ore_rob_cost: u16,
    clay_rob_cost: u16,
    obs_rob_cost_ore: u16,
    obs_rob_cost_clay: u16,
    geo_rob_cost_ore: u16,
    geo_rob_cost_obs: u16,
}

impl Blueprint {
    fn new(
        id: u16,
        ore_rob_cost: u16,
        clay_rob_cost: u16,
        obs_rob_cost_ore: u16,
        obs_rob_cost_clay: u16,
        geo_rob_cost_ore: u16,
        geo_rob_cost_obs: u16,
    ) -> Self {
        Self {
            id,
            ore_rob_cost,
            clay_rob_cost,
            obs_rob_cost_ore,
            obs_rob_cost_clay,
            geo_rob_cost_ore,
            geo_rob_cost_obs,
        }
    }
}

// DFS to find the most geodes that can be cracked in the time remaining, memoized on the search state
fn dfs(s: &State, bp: &Blueprint, hm: &mut HashMap<State, u16>, best: &mut u16) -> u16 {
    if let Some(&best) = hm.get(s) {
        return best;
    }

    if s.remtime == 0 {
        return s.geo_amt;
    } else if *best >= s.geo_amt + s.geo_robot_num * s.remtime + s.remtime * (s.remtime - 1) / 2 {
        // building geo robots for the rest of the turns cannot beat the best found so far (big speedup)
        return *best;
    }

    let mut next_state = s.clone();
    next_state.remtime -= 1;
    next_state.ore_amt += s.ore_robot_num;
    next_state.clay_amt += s.clay_robot_num;
    next_state.obs_amt += s.obs_robot_num;
    next_state.geo_amt += s.geo_robot_num;

    // build geode robot option
    if s.ore_amt >= bp.geo_rob_cost_ore && s.obs_amt >= bp.geo_rob_cost_obs {
        next_state.ore_amt -= bp.geo_rob_cost_ore;
        next_state.obs_amt -= bp.geo_rob_cost_obs;
        next_state.geo_robot_num += 1;
        let val = dfs(&next_state, bp, hm, best);
        if val > *best {
            *best = val;
        }
        next_state.ore_amt += bp.geo_rob_cost_ore;
        next_state.obs_amt += bp.geo_rob_cost_obs;
        next_state.geo_robot_num -= 1;
    }

    // build obsidian robot option
    if s.ore_amt >= bp.obs_rob_cost_ore && s.clay_amt >= bp.obs_rob_cost_clay {
        next_state.ore_amt -= bp.obs_rob_cost_ore;
        next_state.clay_amt -= bp.obs_rob_cost_clay;
        next_state.obs_robot_num += 1;
        let val = dfs(&next_state, bp, hm, best);
        if val > *best {
            *best = val;
        }
        next_state.ore_amt += bp.obs_rob_cost_ore;
        next_state.clay_amt += bp.obs_rob_cost_clay;
        next_state.obs_robot_num -= 1;
    }

    // build clay robot option
    if s.ore_amt >= bp.clay_rob_cost {
        next_state.ore_amt -= bp.clay_rob_cost;
        next_state.clay_robot_num += 1;
        let val = dfs(&next_state, bp, hm, best);
        if val > *best {
            *best = val;
        }
        next_state.ore_amt += bp.clay_rob_cost;
        next_state.clay_robot_num -= 1;
    }

    // build ore robot option - only if we don't already have enough ore to build first 3 robots (2x speedup)
    if s.ore_amt < bp.ore_rob_cost + bp.clay_rob_cost + bp.obs_rob_cost_ore
        && s.ore_amt >= bp.ore_rob_cost
    {
        next_state.ore_amt -= bp.ore_rob_cost;
        next_state.ore_robot_num += 1;
        let val = dfs(&next_state, bp, hm, best);
        if val > *best {
            *best = val;
        }
        next_state.ore_amt += bp.ore_rob_cost;
        next_state.ore_robot_num -= 1;
    }

    // no-build option
    let val = dfs(&next_state, bp, hm, best);
    if val > *best {
        *best = val;
    }

    hm.insert(s.clone(), *best);
    *best
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../robot_blueprints.dat");
    let re_str = concat!(
        r#"Blueprint (\d+): Each ore robot costs (\d+) ore. "#,
        r#"Each clay robot costs (\d+) ore. "#,
        r#"Each obsidian robot costs (\d+) ore and (\d+) clay. "#,
        r#"Each geode robot costs (\d+) ore and (\d+) obsidian."#
    );
    let re = Regex::new(re_str)?;

    let mut bpv: Vec<Blueprint> = vec![];

    let lines: Vec<&str> = input.split('\n').collect();

    for l in lines {
        if l.is_empty() {
            continue;
        }

        if re.is_match(l) {
            let caps = re.captures(l).unwrap();
            let id = caps
                .get(1)
                .map_or(0, |m| m.as_str().parse::<u16>().unwrap());
            let ore_rob_cost = caps
                .get(2)
                .map_or(0, |m| m.as_str().parse::<u16>().unwrap());
            let clay_rob_cost = caps
                .get(3)
                .map_or(0, |m| m.as_str().parse::<u16>().unwrap());
            let obs_rob_cost_ore = caps
                .get(4)
                .map_or(0, |m| m.as_str().parse::<u16>().unwrap());
            let obs_rob_cost_clay = caps
                .get(5)
                .map_or(0, |m| m.as_str().parse::<u16>().unwrap());
            let geo_rob_cost_ore = caps
                .get(6)
                .map_or(0, |m| m.as_str().parse::<u16>().unwrap());
            let geo_rob_cost_obs = caps
                .get(7)
                .map_or(0, |m| m.as_str().parse::<u16>().unwrap());

            bpv.push(Blueprint::new(
                id,
                ore_rob_cost,
                clay_rob_cost,
                obs_rob_cost_ore,
                obs_rob_cost_clay,
                geo_rob_cost_ore,
                geo_rob_cost_obs,
            ));
        } else {
            return Err(Box::new(Error::new(&format!("bad line in input: {}", l))));
        }
    }
    // part 1
    let mut ansa = 0;
    for b in bpv.iter() {
        let s = State::new(24);
        let mut hm: HashMap<State, u16> = HashMap::new();
        ansa += dfs(&s, b, &mut hm, &mut 0) * b.id;
    }
    println!("aoc19a: {}", ansa);

    // part 2
    let mut ansb = 1;
    for b in bpv.iter().take(3) {
        let s = State::new(32);
        let mut hm: HashMap<State, u16> = HashMap::new();
        ansb *= dfs(&s, b, &mut hm, &mut 0);
    }
    println!("aoc19b: {}", ansb);

    Ok(())
}
