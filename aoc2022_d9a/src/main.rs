// AoC 2022 day 9

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// location or move step
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

// function that implements knot "motion"
fn knot_step(from: Point, to: Point) -> Point {
    let mut m = Point::new(0, 0);
    if to.x == 2 + from.x {
        m.x = 1;
        if to.y - from.y >= 1 {
            m.y = 1;
        } else if from.y - to.y >= 1 {
            m.y = -1;
        }
    } else if to.x + 2 == from.x {
        m.x = -1;
        if to.y - from.y >= 1 {
            m.y = 1;
        } else if from.y - to.y >= 1 {
            m.y = -1;
        }
    }

    if to.y == 2 + from.y {
        m.y = 1;
        if to.x - from.x >= 1 {
            m.x = 1;
        } else if from.x - to.x >= 1 {
            m.x = -1;
        }
    } else if to.y + 2 == from.y {
        m.y = -1;
        if to.x - from.x >= 1 {
            m.x = 1;
        } else if from.x - to.x >= 1 {
            m.x = -1;
        }
    }
    m
}

// move an array of connected knots
fn move_cord(dir: Point, dist: i32, cord: &mut [Point], v: &mut HashSet<Point>) {
    for _ in 0..dist {
        cord[0].x += dir.x;
        cord[0].y += dir.y;
        for idx in 1..cord.len() {
            let move_seg = knot_step(cord[idx], cord[idx - 1]);
            cord[idx].x += move_seg.x;
            cord[idx].y += move_seg.y;
        }
        v.insert(cord[cord.len() - 1]);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../head_moves.dat");
    let mut visited: HashSet<Point> = HashSet::new();
    let mut cord_tail_visited: HashSet<Point> = HashSet::new();
    let mut hloc = Point::new(0, 0);
    let mut tloc = Point::new(0, 0);
    let mut cord: Vec<Point> = vec![Point::new(0, 0); 10];

    let coms: Vec<&str> = input.split('\n').collect();

    for com in coms {
        if com.is_empty() {
            continue;
        }

        let c: Vec<&str> = com.split(' ').collect();
        let dir: Point = match c[0] {
            "R" => Point::new(1, 0),
            "L" => Point::new(-1, 0),
            "U" => Point::new(0, 1),
            "D" => Point::new(0, -1),
            _ => return Err(Box::new(Error::new("Error: bad direction"))),
        };
        let dist: i32 = c[1].parse()?;

        // part 1 - move two knots
        for _ in 0..dist {
            hloc.x += dir.x;
            hloc.y += dir.y;
            let move_seg = knot_step(tloc, hloc);
            tloc.x += move_seg.x;
            tloc.y += move_seg.y;

            visited.insert(tloc);
        }

        // part 2 - move a cord of knots
        move_cord(dir, dist, &mut cord, &mut cord_tail_visited);
    }

    println!(
        "aoc9a: {}, aoc9b: {}",
        visited.len(),
        cord_tail_visited.len()
    );

    Ok(())
}
