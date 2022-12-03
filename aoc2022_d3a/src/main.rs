// AoC 2022 day 3

use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../rucksacks.dat");

    let sacks = input.trim().split('\n').collect::<Vec<&str>>();
    let mut ansa: u32 = 0;

    let mut ahs: Vec<HashSet<char>> = vec![HashSet::new(); 3];
    let mut idx = 0;  // 1, 2 or 3 for each elf in a group
    let mut ansb: u32 = 0;

    for s in sacks {
        let sack_size = s.len() / 2;
        let fs = &s[..sack_size];
        let ss = &s[sack_size..];
        let mut fshs: HashSet<char> = HashSet::new();
        let mut sshs: HashSet<char> = HashSet::new();
        for c in fs.chars() {
            fshs.insert(c);
            ahs[idx].insert(c);
        }
        for c in ss.chars() {
            sshs.insert(c);
            ahs[idx].insert(c);
        } 

        for c in fshs.intersection(&sshs) {
            if *c as u8 >= b'a' && *c as u8 <= b'z' {
                ansa += *c as u32 - b'a' as u32 + 1;
            } else {
                ansa += *c as u32 - b'A' as u32 + 27;
            }
        }

        if idx == 2 {
            let isect01: HashSet<_> = ahs[0].intersection(&ahs[1]).collect();
            let isect12: HashSet<_> = ahs[1].intersection(&ahs[2]).collect();
            let intersection: HashSet<_> = isect01.intersection(&isect12).collect();
            for c in intersection {
                if **c as u8 >= b'a' && **c as u8 <= b'z' {
                    ansb += **c as u32 - b'a' as u32 + 1;
                } else {
                    ansb += **c as u32 - b'A' as u32 + 27;
                }
            }
        }

        idx += 1;
        idx %= 3;
        if idx == 0 {
            for hs in ahs.iter_mut() {
                hs.clear();
            }
        }
    }

    println!("aoc3a: {}", ansa);
    println!("aoc3b: {}", ansb);

    Ok(())
}
