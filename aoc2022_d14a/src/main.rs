// AoC 2022 day 14

// initialize the cave data and return the x coordinate where sand is pouring in
fn init_cave(sd: &[Vec<(usize, usize)>], dx: usize, cave: &mut [Vec<char>]) -> usize {
    let sand_x = 500 - dx;
    cave[0][sand_x] = '+';
    for ldat in sd {
        let mut x = usize::MAX;
        let mut y = usize::MAX;

        for l in ldat {
            if x == usize::MAX {
                x = l.0;
                y = l.1;
            } else if l.0 == x {
                let ytarget = l.1;
                let ys = if ytarget < y {
                    ytarget..=y
                } else {
                    y..=ytarget
                };
                for yc in ys {
                    cave[yc][x - dx] = '#';
                }
                y = l.1;
            } else {
                let xtarget = l.0;
                let xs = if xtarget < x {
                    xtarget..=x
                } else {
                    x..=xtarget
                };
                for xc in xs {
                    cave[y][xc - dx] = '#';
                }
                x = l.0;
            }
        }
    }

    sand_x
}

// drop one grain of sand into the cave, returns false if that sand leaks out the bottom
// or if it plugs the hole through which the sand is leaking
fn simulate_sand(sx: usize, cave: &mut [Vec<char>]) -> bool {
    let mut sand_y = 0_usize;
    let mut sand_x = sx;

    while sand_y < cave.len() - 1 {
        if cave[sand_y + 1][sand_x] == '.' {
            sand_y += 1;
        } else if cave[sand_y + 1][sand_x - 1] == '.' {
            sand_y += 1;
            sand_x -= 1;
        } else if cave[sand_y + 1][sand_x + 1] == '.' {
            sand_y += 1;
            sand_x += 1;
        } else {
            cave[sand_y][sand_x] = 'o';
            return sand_y != 0; // will return false one grain earlier than the false return below
        }
    }

    false
}

fn display_cave(cave: &[Vec<char>]) {
    for line in cave.iter() {
        for x in line.iter() {
            print!("{}", x);
        }
        println!();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../rock_shelves.dat");
    let mut shelf_dat: Vec<Vec<(usize, usize)>> = vec![];
    let mut max_y = 0;
    let mut min_x = usize::MAX;
    let mut max_x = 0;

    let lines: Vec<&str> = input.split('\n').collect();

    for l in lines {
        if l.is_empty() {
            continue;
        }

        let pairs: Vec<&str> = l.split(" -> ").collect();
        let mut ldat: Vec<(usize, usize)> = vec![];
        for p in pairs {
            let xy: Vec<&str> = p.split(',').collect();
            let x = xy[0].parse::<usize>()?;
            let y = xy[1].parse::<usize>()?;

            if y > max_y {
                max_y = y;
            }
            if x > max_x {
                max_x = x;
            }
            if x < min_x {
                min_x = x;
            }

            ldat.push((x, y));
        }
        shelf_dat.push(ldat);
    }

    // leave one space on the left and right of the cave for "escaping" sand grains
    let delta_x = min_x - 1;
    max_x -= delta_x - 1;
    let mut cave: Vec<Vec<char>> = vec![vec!['.'; max_x + 1]; max_y + 2];
    let sx = init_cave(&shelf_dat, delta_x, &mut cave);

    for grain_number in 0.. {
        if !simulate_sand(sx, &mut cave) {
            println!("aoc14a: {}", grain_number);
            break;
        }
    }
    display_cave(&cave); // pretty picture (part2 is much wider and not as nice looking)

    // part 2 - adjust numbers to accomodate a complete sand pyramid
    let delta_x = 500 - max_y - 3;
    let bottom_line = vec![
        (delta_x + 1, max_y + 2),
        (delta_x + 2 * max_y + 5, max_y + 2),
    ];
    shelf_dat.push(bottom_line);
    let mut cave: Vec<Vec<char>> = vec![vec!['.'; 2 * max_y + 7]; max_y + 3];
    let sx = init_cave(&shelf_dat, delta_x, &mut cave);

    // note 1 instead of 0 here, see comment in simulate_sand above
    for grain_number in 1.. {
        if !simulate_sand(sx, &mut cave) {
            println!("aoc14b: {}", grain_number);
            break;
        }
    }

    Ok(())
}