// AoC 2022 day 25

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

// returns (top_power, digit_1_or_2) or an err if the number is too large to handle
fn find_top_power(n: i64) -> Result<(u32, i64), String> {
    for p in 0..25 {
        let v = 5_i64.pow(p);
        let hv = v / 2;
        if v * 2 + hv >= n && v - hv <= n {
            if v + hv >= n {
                return Ok((p, 1));
            } else {
                return Ok((p, 2));
            }
        }
    }

    Err("SNAFU number too large".to_owned())
}

// convert the passed decimal number to SNAFU base (as a String)
fn dec2snafu(n: i64) -> Result<String, String> {
    let mut ans = "".to_string();
    let (mut power, d) = find_top_power(n)?;
    let mut nn = n - d * 5_i64.pow(power);
    ans.push((b'0' + d as u8) as char);

    while power > 0 {
        power -= 1;
        let v = 5_i64.pow(power);
        let hv = v / 2;

        if nn.abs() > v + hv {
            if nn < 0 {
                ans.push('=');
                nn += 2 * v;
            } else {
                ans.push('2');
                nn -= 2 * v;
            }
        } else if nn.abs() >= v - hv {
            if nn < 0 {
                ans.push('-');
                nn += v;
            } else {
                ans.push('1');
                nn -= v;
            }
        } else {
            ans.push('0');
        }
    }

    Ok(ans)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../snafu_numbers.dat");
    let mut total = 0;

    let lines: Vec<&str> = input.split('\n').collect();

    // sum all the SNAFU numbers in decimal
    for l in lines {
        if l.is_empty() {
            continue;
        }

        let mut power = 5_i64.pow(l.len() as u32);
        let mut sum = 0;

        for c in l.chars() {
            power /= 5;
            match c {
                '0' => {}

                '1' => sum += power,

                '2' => sum += 2 * power,

                '-' => sum -= power,

                '=' => sum -= 2 * power,

                _ => return Err(Box::new(Error::new(&format!("bad SNAFU digit: {}", c)))),
            }
        }
        total += sum;
    }

    // part 1
    println!(
        "aoc25a: {} decimal or {} in SNAFU",
        total,
        dec2snafu(total)?
    );

    Ok(())
}
