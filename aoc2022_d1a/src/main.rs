// AoC 2022 day 1
//

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../calories.dat");
    let mut v: Vec<u32> = vec![];
    let mut cal = 0;

    for s in input.lines() {
        if s.is_empty() {
            v.push(cal);
            cal = 0;
        } else {
            let c = s.parse::<u32>().unwrap();
            cal += c;
        }
    }
    v.push(cal);

    v.sort_unstable();
    v.reverse();

    println!("aoc1a: {}", v[0]);

    println!("aoc1b: {}", v[0] + v[1] + v[2]);
    Ok(())
}
