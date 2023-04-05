// AoC 2022 day 24

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

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct State {
    time: usize,
    myx: usize,
    myy: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct Blizzard {
    curx: usize,
    cury: usize,
    delta: (i16, i16), // (delta-x, delta-y)
}

impl Blizzard {
    fn new(x: usize, y: usize, d: (i16, i16)) -> Self {
        Self {
            curx: x,
            cury: y,
            delta: d,
        }
    }

    // update a blizzard one time step
    fn update(&mut self, valley: &[Vec<u8>]) {
        let mut nextx = (self.curx as i16 + self.delta.0) as usize;
        let mut nexty = (self.cury as i16 + self.delta.1) as usize;
        if valley[nexty][nextx] == 1 {
            match self.delta {
                (-1, 0) => nextx = valley[self.cury].len() - 2,
                (1, 0) => nextx = 1,
                (0, 1) => nexty = 1,
                (0, -1) => nexty = valley.len() - 2,
                _ => unreachable!(),
            }
        }
        self.curx = nextx;
        self.cury = nexty;
    }
}

// move all blizzards one time step and update valley as well
fn move_blizzards(v: &mut Vec<Vec<Vec<u8>>>, mb: &mut [Blizzard]) {
    // clone last 2D valley and clear the clone of blizzards
    let mut nv = v[v.len() - 1].clone();
    for r in 1..(nv.len() - 1) {
        for c in 1..(nv[r].len() - 1) {
            nv[r][c] = 0;
        }
    }

    // move the blizzards and insert them in their new positions in the 2D valley
    for b in mb.iter_mut() {
        b.update(&nv);
        nv[b.cury][b.curx] = 2;
    }

    // push the new 2D valley onto the 3D vector as the latest time.
    v.push(nv);
}

// breadth first search for a path from (sx, sy) to (_, ey)
// it returns the time steps needed or an error if no path found
// first I tried adding distance to state and sorting on that, but that gave an incorrect answer
// sorting on time ran too long, to get reasonable runtime I also needed to avoid redundant states
fn bfs(
    v: &mut Vec<Vec<Vec<u8>>>,
    mb: &mut [Blizzard],
    sx: usize,
    sy: usize,
    ey: usize,
    t: usize,
) -> Result<usize, String> {
    let mut hs: HashSet<State> = HashSet::new();
    let mut stack: Vec<State> = vec![];
    stack.push(State {
        time: t,
        myx: sx,
        myy: sy,
    });

    while !stack.is_empty() {
        let curstate = stack.pop().unwrap();

        if curstate.myy == ey {
            return Ok(curstate.time); // reached the destination!
        }

        if curstate.time >= v.len() - 1 {
            move_blizzards(v, mb);
        }

        // create new states from curstate and positions of blizzards
        for nx in (curstate.myx - 1)..=(curstate.myx + 1) {
            let endy = if curstate.myy == v[0].len() - 1 {
                curstate.myy
            } else {
                curstate.myy + 1
            };
            for ny in curstate.myy.saturating_sub(1)..=endy {
                if (curstate.myx == nx || curstate.myy == ny) && v[curstate.time + 1][ny][nx] == 0 {
                    let s = State {
                        time: curstate.time + 1,
                        myx: nx,
                        myy: ny,
                    };
                    if !hs.contains(&s) {
                        stack.push(s.clone());
                        hs.insert(s);
                    }
                }
            }
        }

        // prioritize moves that get closer to the destination
        stack.sort_unstable();
        stack.reverse();
    }

    Err("BFS found no viable path".to_owned())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../blizzard_valley.dat");

    // valley is vector of [time][y][x] where values are -> 0 = open, 1 = wall, 2 = blizzard
    let mut valley: Vec<Vec<Vec<u8>>> = vec![];
    valley.push(vec![]); // the valley at time 0

    let mut blizzards: Vec<Blizzard> = vec![];

    let lines: Vec<&str> = input.split('\n').collect();

    for (row, l) in lines.iter().enumerate() {
        if l.is_empty() {
            continue;
        }

        let mut gl = vec![];
        for (col, c) in l.chars().enumerate() {
            match c {
                '.' => gl.push(0),

                '#' => gl.push(1),

                '>' => {
                    gl.push(2);
                    blizzards.push(Blizzard::new(col, row, (1, 0)));
                }

                '<' => {
                    gl.push(2);
                    blizzards.push(Blizzard::new(col, row, (-1, 0)));
                }

                '^' => {
                    gl.push(2);
                    blizzards.push(Blizzard::new(col, row, (0, -1)));
                }

                'v' => {
                    gl.push(2);
                    blizzards.push(Blizzard::new(col, row, (0, 1)));
                }

                _ => return Err(Box::new(Error::new(&format!("bad valley char: {}", c)))),
            }
        }
        valley[0].push(gl);
    }

    // parts 1 & 2
    let endy = valley[0].len() - 1;
    let endx = valley[0][0].len() - 2;
    let time_forward = bfs(&mut valley, &mut blizzards, 1, 0, endy, 0)?;
    println!("aoc24a: {}", time_forward);
    let time_back = bfs(&mut valley, &mut blizzards, endx, endy, 0, time_forward)?;
    let time_forward_again = bfs(&mut valley, &mut blizzards, 1, 0, endy, time_back)?;
    println!("aoc24b: {}", time_forward_again);

    Ok(())
}
