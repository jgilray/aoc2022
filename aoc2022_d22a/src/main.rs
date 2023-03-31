// AoC 2022 day 22

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

#[derive(Debug)]
enum PathElem {
    Right,
    Left,
    Straight(u32),
}

// find the answer according to the rules and b being a flat wrap-around board
fn find_answera(b: &[Vec<char>], path: &[PathElem]) -> usize {
    // 0 = right, 1 = down, 2 = left, 3 = up
    let mut curdir = 0_usize;
    let mut cury = 0_usize;
    let mut curx = b[cury]
        .iter()
        .enumerate()
        .filter(|(_, e)| **e == '.')
        .map(|(i, _)| i)
        .next()
        .unwrap();

    for p in path.iter() {
        match p {
            PathElem::Left => curdir = if curdir == 0 { 3 } else { curdir - 1 },
            PathElem::Right => curdir = (curdir + 1) % 4,
            PathElem::Straight(num) => {
                for _ in 0..*num {
                    let (nexty, nextx) = match curdir {
                        0 => {
                            let mut nx = if curx == b[cury].len() - 1 {
                                0
                            } else {
                                curx + 1
                            };
                            while b[cury][nx] == ' ' {
                                nx = if nx == b[cury].len() - 1 { 0 } else { nx + 1 };
                            }

                            // blocked?
                            if b[cury][nx] == '#' {
                                (cury, curx)
                            } else {
                                (cury, nx)
                            }
                        }

                        1 => {
                            let mut ny = if cury == b.len() - 1 { 0 } else { cury + 1 };
                            while b[ny][curx] == ' ' {
                                ny = if ny == b.len() - 1 { 0 } else { ny + 1 };
                            }

                            // blocked?
                            if b[ny][curx] == '#' {
                                (cury, curx)
                            } else {
                                (ny, curx)
                            }
                        }

                        2 => {
                            let mut nx = if curx == 0 {
                                b[cury].len() - 1
                            } else {
                                curx - 1
                            };
                            while b[cury][nx] == ' ' {
                                nx = if nx == 0 { b[cury].len() - 1 } else { nx - 1 };
                            }

                            // blocked?
                            if b[cury][nx] == '#' {
                                (cury, curx)
                            } else {
                                (cury, nx)
                            }
                        }

                        3 => {
                            let mut ny = if cury == 0 { b.len() - 1 } else { cury - 1 };
                            while b[ny][curx] == ' ' {
                                ny = if ny == 0 { b.len() - 1 } else { ny - 1 };
                            }

                            // blocked?
                            if b[ny][curx] == '#' {
                                (cury, curx)
                            } else {
                                (ny, curx)
                            }
                        }

                        _ => unreachable!(),
                    };
                    (cury, curx) = (nexty, nextx);
                }
            }
        }
    }

    1000 * (cury + 1) + 4 * (curx + 1) + curdir
}

// same as above but with b interpreted as the faces of a cube
fn find_answerb(b: &[Vec<char>], path: &[PathElem], fs: usize) -> usize {
    let mut curdir = 0_usize;
    let mut cury = 0_usize;
    let mut curx = b[cury]
        .iter()
        .enumerate()
        .filter(|(_, e)| **e == '.')
        .map(|(i, _)| i)
        .next()
        .unwrap();

    for p in path.iter() {
        match p {
            PathElem::Left => curdir = if curdir == 0 { 3 } else { curdir - 1 },
            PathElem::Right => curdir = (curdir + 1) % 4,
            PathElem::Straight(num) => {
                for _ in 0..*num {
                    (cury, curx, curdir) = match curdir {
                        0 => {
                            let (ny, nx, nd) = if curx == b[cury].len() - 1 {
                                if cury < fs {
                                    (fs * 3 - cury - 1, b[fs * 3 - cury - 1].len() - 1, 2)
                                } else if cury < fs * 2 {
                                    (fs - 1, cury + fs, 3)
                                } else if cury < fs * 3 {
                                    (fs * 3 - cury - 1, b[fs * 3 - cury - 1].len() - 1, 2)
                                } else {
                                    (3 * fs - 1, cury - 2 * fs, 3)
                                }
                            } else {
                                (cury, curx + 1, 0)
                            };

                            // blocked?
                            if b[ny][nx] == '#' {
                                (cury, curx, curdir)
                            } else {
                                (ny, nx, nd)
                            }
                        }

                        1 => {
                            let (ny, nx, nd) = if cury == fs - 1 && curx >= fs * 2 {
                                (curx - fs, b[curx - fs].len() - 1, 2)
                            } else if cury == 3 * fs - 1 && curx >= fs {
                                (curx + fs * 2, b[curx + fs * 2].len() - 1, 2)
                            } else if cury == fs * 4 - 1 {
                                (0, curx + fs * 2, 1)
                            } else {
                                (cury + 1, curx, 1)
                            };

                            // blocked?
                            if b[ny][nx] == '#' {
                                (cury, curx, curdir)
                            } else {
                                (ny, nx, nd)
                            }
                        }

                        2 => {
                            let (ny, nx, nd) = if curx == fs && cury < fs * 2 {
                                if cury < fs {
                                    (fs * 3 - cury - 1, 0, 0)
                                } else {
                                    (2 * fs, cury - fs, 1)
                                }
                            } else if curx == 0 {
                                if cury < fs * 3 {
                                    (fs * 3 - cury - 1, fs, 0)
                                } else {
                                    (0, cury - 2 * fs, 1)
                                }
                            } else {
                                (cury, curx - 1, 2)
                            };

                            // blocked?
                            if b[ny][nx] == '#' {
                                (cury, curx, curdir)
                            } else {
                                (ny, nx, nd)
                            }
                        }

                        3 => {
                            let (ny, nx, nd) = if cury == 0 && curx < fs * 2 {
                                (curx + fs * 2, 0, 0)
                            } else if cury == 0 && curx >= fs * 2 {
                                (fs * 4 - 1, curx - fs * 2, 3)
                            } else if cury == fs * 2 && curx < fs {
                                (curx + fs, fs, 0)
                            } else {
                                (cury - 1, curx, 3)
                            };

                            // blocked?
                            if b[ny][nx] == '#' {
                                (cury, curx, curdir)
                            } else {
                                (ny, nx, nd)
                            }
                        }

                        _ => unreachable!(),
                    };
                }
            }
        }
    }

    1000 * (cury + 1) + 4 * (curx + 1) + curdir
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../monkey_board.dat");
    let mut getting_board = true;
    let mut brd: Vec<Vec<char>> = vec![];
    let mut brd2: Vec<Vec<char>> = vec![];
    let mut path: Vec<PathElem> = vec![];
    let mut maxlinelen = 0;

    let lines: Vec<&str> = input.split('\n').collect();

    for l in lines {
        if l.is_empty() {
            // add trailing spaces to short lines in board for part 1
            if getting_board {
                brd2 = brd.clone();
                for bl in brd.iter_mut() {
                    while bl.len() < maxlinelen {
                        bl.push(' ');
                    }
                }
            }
            getting_board = false;

            continue;
        }

        if getting_board {
            // inputting the board
            brd.push(l.chars().clone().collect());
            if brd[brd.len() - 1].len() > maxlinelen {
                maxlinelen = brd[brd.len() - 1].len();
            }
        } else {
            // inputting the path
            let mut num = 0_u32;
            for c in l.chars() {
                match c {
                    'R' => {
                        if num != 0 {
                            path.push(PathElem::Straight(num));
                        }
                        path.push(PathElem::Right);
                        num = 0;
                    }

                    'L' => {
                        if num != 0 {
                            path.push(PathElem::Straight(num));
                        }
                        path.push(PathElem::Left);
                        num = 0;
                    }

                    '0'..='9' => num = num * 10 + (c as u8 - b'0') as u32,

                    _ => return Err(Box::new(Error::new(&format!("bad path char: {}", c)))),
                }
            }
            if num != 0 {
                path.push(PathElem::Straight(num));
            }
        }
    }

    // part 1
    let ansa = find_answera(&brd, &path);
    println!("aoc22a: {}", ansa);

    // part 2
    let ansb = find_answerb(&brd2, &path, maxlinelen / 3);
    println!("aoc22b: {}", ansb);

    Ok(())
}
