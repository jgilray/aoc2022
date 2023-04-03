// AoC 2022 day 23

use std::collections::HashMap;
type HMap = HashMap<(usize, usize), Vec<usize>>;

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
struct Elf {
    curx: usize,
    cury: usize,
    nextx: usize,
    nexty: usize,
}

impl Elf {
    fn new(x: usize, y: usize) -> Self {
        Self {
            curx: x,
            cury: y,
            nextx: x,
            nexty: y,
        }
    }

    fn count_neighbors(&self, grove: &[Vec<usize>]) -> usize {
        let mut count = 0;

        for x in (self.curx - 1)..=(self.curx + 1) {
            for y in (self.cury - 1)..=(self.cury + 1) {
                if (x != self.curx || y != self.cury) && grove[y][x] != 0 {
                    count += 1;
                }
            }
        }

        count
    }

    // if a move is to be proposed (nextx, nexty) are changed from (curx, cury) and true is returned
    fn propose_move(&mut self, grove: &[Vec<usize>], dir: usize) -> bool {
        match dir {
            0 => {
                if grove[self.cury - 1][self.curx - 1] == 0
                    && grove[self.cury - 1][self.curx] == 0
                    && grove[self.cury - 1][self.curx + 1] == 0
                {
                    self.nexty -= 1;
                    return true;
                }
            }

            1 => {
                if grove[self.cury + 1][self.curx - 1] == 0
                    && grove[self.cury + 1][self.curx] == 0
                    && grove[self.cury + 1][self.curx + 1] == 0
                {
                    self.nexty += 1;
                    return true;
                }
            }

            2 => {
                if grove[self.cury - 1][self.curx - 1] == 0
                    && grove[self.cury][self.curx - 1] == 0
                    && grove[self.cury + 1][self.curx - 1] == 0
                {
                    self.nextx -= 1;
                    return true;
                }
            }

            3 => {
                if grove[self.cury - 1][self.curx + 1] == 0
                    && grove[self.cury][self.curx + 1] == 0
                    && grove[self.cury + 1][self.curx + 1] == 0
                {
                    self.nextx += 1;
                    return true;
                }
            }

            _ => unreachable!(),
        }
        false
    }
}

// each elf looks around (0 = North, 1 = South, 2 = West, 3 = East) then optionally proposes a move
fn phase1(grove: &[Vec<usize>], elves: &mut [Elf], hm: &mut HMap, dir: usize) {
    for (i, e) in elves.iter_mut().enumerate() {
        if i == 0 {
            continue; // skip placeholder elf
        }

        if e.count_neighbors(grove) == 0 {
            continue; // no neighbors
        }

        for delta_dir in 0..4 {
            if e.propose_move(grove, (dir + delta_dir) % 4) {
                hm.entry((e.nextx, e.nexty))
                    .and_modify(|v| v.push(i))
                    .or_insert(vec![i]);

                break;
            }
        }
    }
}

// move elves when there is no conflict with another elf's proposal, return the number of elves moved
fn phase2(grove: &mut [Vec<usize>], elves: &mut [Elf], hm: &HMap) -> usize {
    let mut retval = 0;

    for (k, v) in hm {
        if v.len() == 1 {
            // do the move in elves and grove
            retval += 1;
            grove[elves[v[0]].cury][elves[v[0]].curx] = 0;
            grove[k.1][k.0] = v[0];
            elves[v[0]].curx = elves[v[0]].nextx;
            elves[v[0]].cury = elves[v[0]].nexty;
        } else {
            // rescind move for all elves proposing to move to the same square
            for en in v.iter() {
                elves[*en].nextx = elves[*en].curx;
                elves[*en].nexty = elves[*en].cury;
            }
        }
    }

    retval
}

fn find_answera(elves: &[Elf]) -> usize {
    let mut minx = usize::MAX;
    let mut miny = usize::MAX;
    let mut maxx = 0;
    let mut maxy = 0;

    for (i, e) in elves.iter().enumerate() {
        if i == 0 {
            continue; // skip placeholder Elf
        }

        if e.curx < minx {
            minx = e.curx;
        }
        if e.cury < miny {
            miny = e.cury;
        }
        if e.curx > maxx {
            maxx = e.curx;
        }
        if e.cury > maxy {
            maxy = e.cury;
        }
    }
    (maxx - minx + 1) * (maxy - miny + 1) - (elves.len() - 1)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../grove_of_elves.dat");
    let mut grove: Vec<Vec<usize>> = vec![];
    grove.resize(100, vec![0; 270]);

    let mut elves: Vec<Elf> = vec![];
    elves.push(Elf::new(0, 0)); // placeholder elf for empty spaces

    let lines: Vec<&str> = input.split('\n').collect();

    for (row, l) in lines.iter().enumerate() {
        if l.is_empty() {
            continue;
        }
        let mut gl: Vec<usize> = vec![0; 100];

        for (col, c) in l.chars().enumerate() {
            match c {
                '.' => gl.push(0),
                '#' => {
                    gl.push(elves.len());
                    elves.push(Elf::new(col + 100, row + 100));
                }
                _ => return Err(Box::new(Error::new(&format!("bad grove char: {}", c)))),
            }
        }
        gl.resize(270, 0);
        grove.push(gl);
    }
    grove.resize(270, vec![0; 270]);

    // parts 1 & 2
    let mut hm: HMap = HashMap::new();
    let mut dir = 0;
    for round in 1..1000 {
        phase1(&grove, &mut elves, &mut hm, dir);
        let num_moved = phase2(&mut grove, &mut elves, &hm);
        if round == 10 {
            let ansa = find_answera(&elves);
            println!("aoc23a: {}", ansa);
        }
        if num_moved == 0 {
            println!("aoc23b: {}", round);
            break;
        }
        dir = (dir + 1) % 4;
        hm.clear();
    }

    Ok(())
}
