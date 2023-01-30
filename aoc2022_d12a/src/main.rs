// AoC 2022 day 12

use std::cmp::max;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
// location or move step
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

// Loc: (height, number of steps to get here)
//type Loc = (usize, usize);
struct Loc {
    height: usize,
    numsteps: usize,
}

// function that does a DFS looking for the cheapest path from start to end
// it returns the number of steps
fn dfs(
    cur: &Point,
    steps: usize,
    best_so_far: Option<usize>,
    end: &Point,
    v: &mut Vec<Vec<Loc>>,
) -> Option<usize> {
    // closure to find minimum number of steps to get to the end square, up or over
    let dist_to_end = |x: usize, y: usize, h: usize| {
        max(
            (b'z' - b'a') as usize - h,
            if end.x >= x { end.x - x } else { x - end.x }
                + if end.y >= y { end.y - y } else { y - end.y },
        )
    };

    if steps >= v[cur.y][cur.x].numsteps {
        return None; // already a better path through this point
    } else {
        v[cur.y][cur.x].numsteps = steps; // best path so far
    }

    if cur == end {
        return Some(steps);
    }

    // trys is an array of up to four next locations sorted in order of distance from the end loc
    let mut trys: Vec<(usize, Point)> = vec![];
    for x in cur.x.saturating_sub(1)..=cur.x + 1 {
        for y in cur.y.saturating_sub(1)..=cur.y + 1 {
            // third line below enforces that we can only go up 1 at a time
            if x < v[cur.y].len()
                && y < v.len()
                && (x == cur.x && y != cur.y || x != cur.x && y == cur.y)
                && v[cur.y][cur.x].height + 1 >= v[y][x].height
            {
                let best_possible = dist_to_end(x, y, v[y][x].height) + steps + 1;
                // don't try directions that cannot beat best_so_far
                if best_so_far.is_none() || best_possible < best_so_far.unwrap() {
                    trys.push((best_possible, Point::new(x, y)));
                }
            }
        }
    }

    trys.sort_unstable();
    let mut best = best_so_far;
    for t in &trys {
        if let Some(s) = dfs(&t.1, steps + 1, best, end, v) {
            if best.is_none() || best.is_some() && s < best.unwrap() {
                best = Some(s);
            }
        }
    }

    best
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../terrain.dat");
    let mut hills: Vec<Vec<Loc>> = vec![];
    let mut start: Point = Point::new(0, 0);
    let mut end: Point = Point::new(0, 0);
    let mut all_as: Vec<Point> = vec![];

    let lines: Vec<&str> = input.split('\n').collect();

    for (y, line) in lines.iter().enumerate() {
        if line.is_empty() {
            continue;
        }
        let mut l: Vec<Loc> = vec![];

        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = Point::new(x, y);
                    l.push(Loc {
                        height: 0,
                        numsteps: usize::MAX,
                    });
                }
                'E' => {
                    end = Point::new(x, y);
                    l.push(Loc {
                        height: (b'z' - b'a') as usize,
                        numsteps: usize::MAX,
                    });
                }
                'a' => {
                    all_as.push(Point::new(x, y)); // for part 2
                    l.push(Loc {
                        height: 0,
                        numsteps: usize::MAX,
                    });
                }
                _ => l.push(Loc {
                    height: (c as u8 - b'a') as usize,
                    numsteps: usize::MAX,
                }),
            }
        }

        hills.push(l);
    }

    // part 1
    if let Some(ans) = dfs(&start, 0, None, &end, &mut hills) {
        println!("aoc12a: {}", ans);
    } else {
        println!("no path found");
    }

    // part 2 - to drastically speed up this part, we don't reset hills!
    let mut very_best = usize::MAX;
    for a in &all_as {
        if let Some(ans) = dfs(a, 0, None, &end, &mut hills) {
            if ans < very_best {
                very_best = ans;
            }
        }
    }

    println!("aoc12b: {}", very_best);

    Ok(())
}
