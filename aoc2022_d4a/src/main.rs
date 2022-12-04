// AoC 2022 day 4

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../cleaning_sectors.dat");

    let pairs = input.trim().split('\n').collect::<Vec<&str>>();
    let mut ansa: u32 = 0;
    let mut ansb: u32 = 0;

    for p in pairs {
        // v1 is a vec of the two ranges, one for each elf
        let v1: Vec<&str> = p.split(',').collect();

        // e0 is the range for the first elf, etc.
        let e0: Vec<&str> = v1[0].split('-').collect();
        let e1: Vec<&str> = v1[1].split('-').collect();

        // the lower and upper bounds of the two ranges
        let e0lb = e0[0].parse::<u32>().unwrap();
        let e0ub = e0[1].parse::<u32>().unwrap();
        let e1lb = e1[0].parse::<u32>().unwrap();
        let e1ub = e1[1].parse::<u32>().unwrap();

        if e0lb >= e1lb && e0ub <= e1ub || e1lb >= e0lb && e1ub <= e0ub {
            ansa += 1; // fully overlap
        } else if e0lb <= e1lb && e0ub >= e1lb || e1lb <= e0lb && e1ub >= e0lb {
            ansb += 1; // any overlap except fully
        }
    }

    println!("aoc4a: {}", ansa);
    println!("aoc4b: {}", ansa + ansb);

    Ok(())
}
