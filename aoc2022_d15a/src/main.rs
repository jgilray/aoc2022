// AoC 2022 day 15

use regex::Regex;
use std::collections::HashSet;

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

type SVec = Vec<(i64, i64, i64)>;
type HSet = HashSet<(i64, i64)>;

const P1_TARGET: i64 = 2_000_000;
const BLIM_MIN: i64 = 0;
const BLIM_MAX: i64 = 4_000_000;

// manhattan distance between two points
fn mdist(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

// function that returns a hidden beacon if found or else None
fn is_hidden_beacon(v: &SVec, x: i64, y: i64) -> Option<(i64, i64)> {
    if (BLIM_MIN..=BLIM_MAX).contains(&x) && (BLIM_MIN..=BLIM_MAX).contains(&y) {
        let mut covered = false;
        for (sx, sy, dist) in v.iter() {
            if mdist(*sx, *sy, x, y) <= *dist {
                covered = true;
                break;
            }
        }

        if !covered {
            return Some((x, y));
        }
    }

    None
}

// create a vector of points that form a manhattan "circle" of radius r around the origin (ox, oy)
fn circle(ox: i64, oy: i64, r: i64) -> Vec<(i64, i64)> {
    let mut retval = Vec::new();
    for x in 0..=r {
        let dy = r - x;
        retval.push((ox + x, oy + dy));
        retval.push((ox + x, oy - dy));
        retval.push((ox - x, oy + dy));
        retval.push((ox - x, oy - dy));
    }

    retval
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../sensor_beacon.dat");
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")?;
    let mut v: SVec = vec![];
    let mut bhs: HSet = HashSet::new();

    let lines: Vec<&str> = input.split('\n').collect();

    for l in lines {
        if l.is_empty() {
            continue;
        }

        if re.is_match(l) {
            let caps = re.captures(l).unwrap();
            let sx = caps
                .get(1)
                .map_or(0, |m| m.as_str().parse::<i64>().unwrap());
            let sy = caps
                .get(2)
                .map_or(0, |m| m.as_str().parse::<i64>().unwrap());
            let bx = caps
                .get(3)
                .map_or(0, |m| m.as_str().parse::<i64>().unwrap());
            let by = caps
                .get(4)
                .map_or(0, |m| m.as_str().parse::<i64>().unwrap());

            bhs.insert((bx, by));
            v.push((sx, sy, mdist(sx, sy, bx, by)));
        } else {
            return Err(Box::new(Error::new(&format!("bad line in input: {}", l))));
        }
    }

    // part 1 - this is pretty crude, but it works: basically just create an array for the line
    // and fill it with 1s (covered), and 0s (not).
    let mut lowest_x = i64::MAX;
    let mut highest_x = i64::MIN;
    let mut norm_v: Vec<(i64, i64)> = vec![];
    for (sx, sy, dist) in v.iter() {
        let dy = (sy - P1_TARGET).abs();
        if dy <= *dist {
            let low_x = sx - (dist - dy);
            if low_x < lowest_x {
                lowest_x = low_x;
            }
            let high_x = sx + (dist - dy);
            if high_x > highest_x {
                highest_x = high_x;
            }
            norm_v.push((low_x, high_x));
        }
    }
    let rshift = -lowest_x;
    let mut countvec: Vec<u64> = vec![0; (highest_x + rshift + 1) as usize];
    for (l, h) in norm_v.iter() {
        for idx in (l + rshift)..=(h + rshift) {
            countvec[idx as usize] = 1;
        }
    }
    for (bx, by) in bhs.iter() {
        if *bx >= 0 && *bx <= highest_x + rshift && *by == P1_TARGET {
            countvec[(*bx + rshift) as usize] = 0;
        }
    }
    let ansa: u64 = countvec.iter().sum();
    println!("aoc15a: {}", ansa);

    // part 2 - first I looked for pairs of sensors where the sum of their beacon distances was
    // two less than their distance apart, but that didn't work so thinking about it, it must be just
    // outside the "circle" of one (or several) of the sensors, so I simply looked there for possibles
    'lp: for (sx, sy, dist) in v.iter() {
        for (x, y) in circle(*sx, *sy, dist + 1) {
            if let Some((fx, fy)) = is_hidden_beacon(&v, x, y) {
                println!("aoc15b: {}", 4_000_000 * fx + fy);
                break 'lp;
            }
        }
    }

    Ok(())
}
