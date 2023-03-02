// AoC 2022 day 17

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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
// state of the cavern
struct State {
    bidx: u16,
    afidx: u16,
    shaft: Vec<u16>,
}

impl State {
    fn new(bidx: u16, afidx: u16, ch: &[u8], cht: usize) -> Self {
        let mut swipev: Vec<u16> = vec![u16::MAX; 7];
        let mut swipe = 0_u8;
        let mut idx = cht;

        while swipe < 127 {
            let lastswipe = swipe;
            swipe |= ch[idx];
            let diff = swipe - lastswipe;
            let mut scan: u8 = 1;
            for bit in 0..7 {
                if scan & diff > 0 {
                    swipev[bit] = (cht - idx) as u16;
                }
                scan <<= 1;
            }
            if idx > 0 {
                idx -= 1;
            } else {
                break;
            }
        }

        Self {
            bidx,
            afidx,
            shaft: swipev,
        }
    }
}

#[derive(Debug, Clone)]
// falling rock shape
struct Shape {
    lines: Vec<u8>,
    bot: Vec<(usize, usize)>,
    lft: Vec<(usize, usize)>,
    rgt: Vec<(usize, usize)>,
}

impl Shape {
    fn new(id: u8) -> Self {
        let mut l: Vec<u8> = vec![];
        let mut bot: Vec<(usize, usize)> = vec![];
        let mut lft: Vec<(usize, usize)> = vec![];
        let mut rgt: Vec<(usize, usize)> = vec![];

        match id {
            0 => {
                // horizontal bar
                l.push(30_u8);
                bot.push((0, 1));
                bot.push((0, 2));
                bot.push((0, 3));
                bot.push((0, 4));
                lft.push((0, 4));
                rgt.push((0, 1));
            }

            1 => {
                // "+" sign
                l.push(8_u8);
                l.push(28_u8);
                l.push(8_u8);
                bot.push((0, 3));
                bot.push((1, 2));
                bot.push((1, 4));
                lft.push((1, 4)); // note that most left, most right are always idx 0 for lft and rgt
                lft.push((2, 3));
                lft.push((0, 3));
                rgt.push((1, 2));
                rgt.push((0, 3));
                rgt.push((2, 3));
            }

            2 => {
                // backwards "l"
                l.push(28_u8);
                l.push(4_u8);
                l.push(4_u8);
                bot.push((0, 2));
                bot.push((0, 3));
                bot.push((0, 4));
                lft.push((0, 4));
                lft.push((1, 2));
                lft.push((2, 2));
                rgt.push((0, 2));
                rgt.push((1, 2));
                rgt.push((2, 2));
            }

            3 => {
                // vertical bar
                l.push(16_u8);
                l.push(16_u8);
                l.push(16_u8);
                l.push(16_u8);
                bot.push((0, 4));
                lft.push((0, 4));
                lft.push((1, 4));
                lft.push((2, 4));
                lft.push((3, 4));
                rgt.push((0, 4));
                rgt.push((1, 4));
                rgt.push((2, 4));
                rgt.push((3, 4));
            }

            4 => {
                // square block
                l.push(24_u8);
                l.push(24_u8);
                bot.push((0, 3));
                bot.push((0, 4));
                lft.push((0, 4));
                lft.push((1, 4));
                rgt.push((0, 3));
                rgt.push((1, 3));
            }

            _ => unreachable!(),
        }
        Self {
            lines: l,
            bot,
            lft,
            rgt,
        }
    }
}

#[derive(Debug, Clone)]
enum WindDirection {
    Left,
    Right,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const TARGET: usize = 1_000_000_000_000;
    let input = include_str!("../../airflow.dat");
    let mut afv: Vec<WindDirection> = vec![];
    let mut hm: HashMap<State, (usize, usize)> = HashMap::new();

    let mut found_ansa: bool = false;
    let mut found_ansb: bool = false;
    let mut found_repeat: bool = false;
    let mut residual: usize = 0;
    let mut total_cht: usize = 0;

    let aflines: Vec<&str> = input.split('\n').collect();

    for l in aflines {
        for c in l.chars() {
            match c {
                '>' => afv.push(WindDirection::Right),
                '<' => afv.push(WindDirection::Left),
                _ => return Err(Box::new(Error::new(&format!("bad char in input: {}", c)))),
            }
        }
    }

    // create first 10 rows of the vertical chamber
    let mut chamber: Vec<u8> = vec![0; 10]; // 128 = left wall
    chamber[0] |= 127_u8; // add the floor
    let mut chamber_top = 0;
    let mut prev_chamber_top = 0;

    // create the vector of Shapes
    let sv: Vec<Shape> = (0..=4).map(Shape::new).collect();

    let mut afidx = 0;
    for bidx in 0..10000 {
        let shape = &sv[bidx % 5];

        // capture state, look for repeats
        let state = State::new((bidx % 5) as u16, afidx as u16, &chamber, chamber_top);
        if hm.contains_key(&state) && !found_repeat {
            let (oct, obidx) = hm.get(&state).unwrap();
            let rise = chamber_top - oct;
            let numshapes = bidx - obidx;
            let full_repeats = (TARGET - bidx) / numshapes;
            residual = TARGET - bidx - full_repeats * numshapes;
            total_cht = chamber_top + full_repeats * rise;
            found_repeat = true;
        } else {
            hm.insert(state, (chamber_top, bidx));
        }

        chamber_top += 3 + shape.lines.len();
        if chamber_top >= chamber.len() {
            chamber.extend((0..10).map(|_| 0_u8)); // 10 more locations on top of the chamber
        }

        // move the new shape
        let mut lrdelta = 0_isize;
        let mut dwndelta = 0_usize;
        loop {
            // first, gauge the wind and move left or right if possible
            let dir = &afv[afidx]; 
            afidx += 1;
            if afidx >= afv.len() {
                afidx = 0;
            }

            match dir {
                WindDirection::Right => {
                    let mostrightx = if lrdelta < 0 {
                        shape.rgt[0].1 - lrdelta.abs() as usize
                    } else {
                        shape.rgt[0].1 + lrdelta as usize
                    };
                    let mut moveok = true;
                    for idx in 0..shape.rgt.len() {
                        let ysidx = shape.rgt[idx].0;
                        let ycidx = chamber_top - shape.lines.len() + 1 + ysidx - dwndelta;

                        let rightone = lrdelta - 1;
                        let shline = if rightone < 0 {
                            shape.lines[ysidx] >> rightone.abs()
                        } else {
                            shape.lines[ysidx] << rightone
                        };

                        if chamber[ycidx] & shline != 0 {
                            moveok = false;
                            break;
                        }
                    }
                    if moveok && mostrightx > 0 {
                        lrdelta -= 1;
                    }
                }

                WindDirection::Left => {
                    let mostleftx = if lrdelta < 0 {
                        shape.lft[0].1 - lrdelta.abs() as usize
                    } else {
                        shape.lft[0].1 + lrdelta as usize
                    };
                    let mut moveok = true;
                    for idx in 0..shape.lft.len() {
                        let ysidx = shape.lft[idx].0;
                        let ycidx = chamber_top - shape.lines.len() + 1 + ysidx - dwndelta;

                        let leftone = lrdelta + 1;
                        let shline = if leftone < 0 {
                            shape.lines[ysidx] >> leftone.abs()
                        } else {
                            shape.lines[ysidx] << leftone
                        };

                        if chamber[ycidx] & shline != 0 {
                            moveok = false;
                            break;
                        }
                    }
                    if moveok && mostleftx < 6 {
                        lrdelta += 1;
                    }
                }
            }

            // second, check if can move down one
            let mut moveok = true;
            for idx in 0..shape.bot.len() {
                let ysidx = shape.bot[idx].0;
                let ycidx = chamber_top - shape.lines.len() + ysidx - dwndelta;

                let shline = if lrdelta < 0 {
                    shape.lines[ysidx] >> lrdelta.abs()
                } else {
                    shape.lines[ysidx] << lrdelta
                };

                if chamber[ycidx] & shline != 0 {
                    moveok = false;
                    break;
                }
            }

            if moveok {
                dwndelta += 1;
            } else {
                // shape has stopped, insert it into the chamber
                let insidx = chamber_top - shape.lines.len() + 1 - dwndelta;
                for i in 0..shape.lines.len() {
                    let shline = if lrdelta < 0 {
                        shape.lines[i] >> lrdelta.abs()
                    } else {
                        shape.lines[i] << lrdelta
                    };
                    chamber[insidx + i] |= shline;
                }

                break;
            }
        }

        if dwndelta < 3 + shape.lines.len() {
            chamber_top -= dwndelta;
        } else {
            chamber_top = prev_chamber_top;
        }

        // found a repeat, so calculate the answer to part b using the information above
        // allowing the cavern building code to continue until the residual portion in calculated
        if found_repeat && !found_ansb {
            residual -= 1;
            total_cht += chamber_top - prev_chamber_top;
            if residual == 0 {
                found_ansb = true;
                println!("aoc17b: {}", total_cht);
                if found_ansa {
                    break;
                }
            }
        }
        prev_chamber_top = chamber_top;

        // find answer to part a, note that this can sometimes be printed after the answer to part b
        if bidx == 2021 {
            found_ansa = true;
            println!("aoc17a: {}", chamber_top);
            if found_ansb {
                break;
            }
        }
    }

    Ok(())
}
