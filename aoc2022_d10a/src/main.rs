// AoC 2022 day 10

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

// Do the sprite display on the CRT according to the rules
fn crt_update(cycle: i64, regx: i64) -> i64 {
    let hpos = cycle % 40;
    if regx - 1 <= hpos && regx + 1 >= hpos {
        print!("#");
    } else {
        print!(".");
    }

    if hpos == 39 {
        println!();
    }
    cycle + 1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../signal_strength.dat");
    let mut regx: i64 = 1;
    let mut cycle: i64 = 0;
    let mut sum_strength: i64 = 0;

    let coms: Vec<&str> = input.split('\n').collect();

    for com in coms {
        if com.is_empty() {
            continue;
        }

        let c: Vec<&str> = com.split(' ').collect();
        let instruction = match c[0] {
            "noop" => c[0],
            "addx" => c[0],
            _ => return Err(Box::new(Error::new("Error: bad instruction"))),
        };

        if instruction == "addx" {
            cycle = crt_update(cycle, regx);
            if cycle <= 220 && cycle % 40 == 20 {
                sum_strength += cycle * regx;
            }
            cycle = crt_update(cycle, regx);
            if cycle <= 220 && cycle % 40 == 20 {
                sum_strength += cycle * regx;
            }

            let addend: i64 = c[1].parse()?;
            regx += addend;
        } else {
            cycle = crt_update(cycle, regx);
            if cycle <= 220 && cycle % 40 == 20 {
                sum_strength += cycle * regx;
            }
        }
    }

    println!("aoc10a: {}", sum_strength);

    Ok(())
}
