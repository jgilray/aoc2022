// AoC 2022 day 6

use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../datastream.dat");

    let ds = input.chars().collect::<Vec<char>>();

    for (i, v) in ds.windows(4).enumerate() {
        let hs: HashSet<char> = v.iter().copied().collect();
        if hs.len() == 4 {
            println!("aoc6a: {}", i + 4);
            break;
        }
    }

    for (i, v) in ds.windows(14).enumerate() {
        let hs: HashSet<char> = v.iter().copied().collect();
        if hs.len() == 14 {
            println!("aoc6b: {}", i + 14);
            break;
        }
    }

    Ok(())
}
