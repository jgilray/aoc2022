// AoC 2022 day 18

use std::collections::HashSet;
use std::collections::VecDeque;

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

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
// state of the cavern
struct Cube {
    x: i16,
    y: i16,
    z: i16,
}

impl Cube {
    fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }

    pub fn dims_in_common(&self, other: &Cube) -> i16 {
        let mut common = 0;
        let xdiff = self.x - other.x;
        if xdiff.abs() > 1 {
            return 0;
        } else if xdiff == 0 {
            common += 1;
        }

        let ydiff = self.y - other.y;
        if ydiff.abs() > 1 {
            return 0;
        } else if ydiff == 0 {
            common += 1;
        }

        let zdiff = self.z - other.z;
        if zdiff.abs() > 1 {
            return 0;
        } else if zdiff == 0 {
            common += 1;
        }

        common
    }
}

// calculate the number of exposed faces in an array of Cubes
// created for part 1 but used in part 2 as well
fn calc_exposed_faces(cv: &[Cube]) -> i16 {
    let mut retval = 0;
    for c1 in cv.iter() {
        let mut matches = 0;
        for c2 in cv.iter() {
            if c1.dims_in_common(c2) == 2 {
                matches += 1;
            }
        }
        retval += 6 - matches;
    }
    retval
}

// BFS to find if the passed Cube c is in an air pocket
fn find_pocket(
    c: &Cube,
    aphs: &mut HashSet<Cube>,
    naphs: &HashSet<Cube>,
    hs: &HashSet<Cube>,
) -> bool {
    let mut bfs: VecDeque<Cube> = VecDeque::new(); // BFS stack
    bfs.push_back(c.clone());

    while !bfs.is_empty() {
        let nc = bfs.pop_front().unwrap();
        aphs.insert(nc.clone());

        for x in (nc.x - 1)..=(nc.x + 1) {
            for y in (nc.y - 1)..=(nc.y + 1) {
                for z in (nc.z - 1)..=(nc.z + 1) {
                    if x != nc.x && y == nc.y && z == nc.z
                        || x == nc.x && y != nc.y && z == nc.z
                        || x == nc.x && y == nc.y && z != nc.z
                    {
                        let cube = Cube::new(x, y, z);

                        if naphs.contains(&cube) {
                            return false;
                        }
                        if hs.contains(&cube) || aphs.contains(&cube) || bfs.contains(&cube) {
                            continue;
                        }
                        bfs.push_back(cube);
                    }
                }
            }
        }
    }

    // if we get this far the cubes in aphs are an air pocket
    true
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../1x1x1cubes.dat");
    let mut cv: Vec<Cube> = vec![];
    let mut hs: HashSet<Cube> = HashSet::new();
    let mut lx = i16::MAX;
    let mut hx = 0;
    let mut ly = i16::MAX;
    let mut hy = 0;
    let mut lz = i16::MAX;
    let mut hz = 0;

    let lines: Vec<&str> = input.split('\n').collect();

    for l in lines {
        if l.is_empty() {
            continue;
        }

        let d: Vec<_> = l.split(',').collect(); // cube dimensions
        if d.len() != 3 {
            return Err(Box::new(Error::new(&format!("bad input line: {}", l))));
        }
        let x = d[0].parse::<i16>()?;
        if x > hx {
            hx = x;
        }
        if x < lx {
            lx = x;
        }

        let y = d[1].parse::<i16>()?;
        if y > hy {
            hy = y;
        }
        if y < ly {
            ly = y;
        }

        let z = d[2].parse::<i16>()?;
        if z > hz {
            hz = z;
        }
        if z < lz {
            lz = z;
        }

        let cube = Cube::new(x, y, z);
        if !hs.insert(cube.clone()) {
            return Err(Box::new(Error::new(&format!(
                "duplicate cube at {}, {}, {}",
                cube.x, cube.y, cube.z
            ))));
        }
        cv.push(cube);
    }

    // part 1
    let ansa = calc_exposed_faces(&cv);
    println!("aoc18a: {}", ansa);

    // part 2
    let mut apv: Vec<Vec<Cube>> = vec![]; // the air pockets we find below

    let mut aphs: HashSet<Cube> = HashSet::new(); // possible air pocket cubes
    let mut not_aphs: HashSet<Cube> = HashSet::new(); // known external or "used" cubes

    // prefill not_aphs with edge cubes
    for x in lx..=hx {
        for y in ly..=hy {
            for z in lz..=hz {
                let cube = Cube::new(x, y, z);
                if !hs.contains(&cube)
                    && (x == lx || y == ly || z == lz || x == hx || y == hy || z == hz)
                {
                    not_aphs.insert(cube);
                }
            }
        }
    }

    // search for air pockets
    for x in lx..=hx {
        for y in ly..=hy {
            for z in lz..=hz {
                let cube = Cube::new(x, y, z);
                if not_aphs.contains(&cube) || hs.contains(&cube) {
                    continue;
                }

                // cube is a possible start to an air pocket
                aphs.clear();
                if find_pocket(&cube, &mut aphs, &not_aphs, &hs) {
                    // put the air pocket cubes in apv
                    let v: Vec<Cube> = aphs.iter().cloned().collect();
                    apv.push(v);
                }
                // remove all aphs from further consideration, whether or not they are in a pocket
                not_aphs.extend(aphs.iter().cloned());
            }
        }
    }

    // subtract out air pockets
    let mut ansb = ansa;
    for ap in apv.iter() {
        ansb -= calc_exposed_faces(ap);
    }
    println!("aoc18b: {}", ansb);

    Ok(())
}
