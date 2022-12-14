// AoC 2022 day 8

// returns (number of visible trees, highest scenic score)
fn score_visibility(v: &[Vec<u8>]) -> (usize, usize) {
    let mut vis = 2 * v.len() + 2 * v[0].len() - 4;
    let mut max_scenic = 0;

    for x in 1..(v[0].len() - 1) {
        for y in 1..(v.len() - 1) {
            let height = v[y][x];
            let mut sc = [x, v[0].len() - x - 1, y, v.len() - y - 1];

            let mut blocked_left = false;
            for xx in 1..=x {
                if v[y][x - xx] >= height {
                    blocked_left = true;
                    sc[0] = xx;
                    break;
                }
            }

            let mut blocked_right = false;
            for xx in (x + 1)..v[0].len() {
                if v[y][xx] >= height {
                    blocked_right = true;
                    sc[1] = xx - x;
                    break;
                }
            }

            let mut blocked_up = false;
            for yy in 1..=y {
                if v[y - yy][x] >= height {
                    blocked_up = true;
                    sc[2] = yy;
                    break;
                }
            }

            let mut blocked_down = false;
            for yy in (y + 1)..v.len() {
                if v[yy][x] >= height {
                    blocked_down = true;
                    sc[3] = yy - y;
                    break;
                }
            }

            // part 1 calculation
            if !blocked_left || !blocked_right || !blocked_up || !blocked_down {
                vis += 1;
            }

            // part 2 calculation
            let scenic: usize = sc.iter().product();
            if scenic > max_scenic {
                max_scenic = scenic;
            }
        }
    }

    (vis, max_scenic)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../trees.dat");

    let v: Vec<Vec<u8>> = input
        .split('\n')
        .into_iter()
        .map(|s| s.chars().map(|c| c as u8 - b'0').collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let (ansa, ansb) = score_visibility(&v);
    println!("aoc8a: {}, aoc8b: {}", ansa, ansb);

    Ok(())
}
