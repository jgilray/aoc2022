// AoC 2022 day 2

// part1's rock-paper-scissors interpretation
fn rps(opp: &str, you: &str) -> Result<u32, String> {
    match opp {
        "A" => match you {
            "X" => Ok(4),
            "Y" => Ok(8),
            "Z" => Ok(3),
            _ => Err("bad str for you".to_string()),
        },
        "B" => match you {
            "X" => Ok(1),
            "Y" => Ok(5),
            "Z" => Ok(9),
            _ => Err("bad str for you".to_string()),
        },
        "C" => match you {
            "X" => Ok(7),
            "Y" => Ok(2),
            "Z" => Ok(6),
            _ => Err("bad str for you".to_string()),
        },
        _ => Err("bad str for opponent".to_string()),
    }
}

// part2's rock-paper-scissors interpretation
fn rps2(opp: &str, you: &str) -> Result<u32, String> {
    match opp {
        "A" => match you {
            "X" => Ok(3),
            "Y" => Ok(4),
            "Z" => Ok(8),
            _ => Err("bad str for you".to_string()),
        },
        "B" => match you {
            "X" => Ok(1),
            "Y" => Ok(5),
            "Z" => Ok(9),
            _ => Err("bad str for you".to_string()),
        },
        "C" => match you {
            "X" => Ok(2),
            "Y" => Ok(6),
            "Z" => Ok(7),
            _ => Err("bad str for you".to_string()),
        },
        _ => Err("bad str for opponent".to_string()),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../rock_paper_scissors.dat");
    let coms = input.trim().split('\n').collect::<Vec<&str>>();

    let mut ans = 0;
    for com in &coms {
        let battle = com.split(' ').collect::<Vec<&str>>();
        ans += rps(battle[0], battle[1])?;
    }
    println!("aoc2a: {}", ans);

    ans = 0;
    for com in &coms {
        let battle = com.split(' ').collect::<Vec<&str>>();
        ans += rps2(battle[0], battle[1])?;
    }
    println!("aoc2b: {}", ans);

    Ok(())
}
