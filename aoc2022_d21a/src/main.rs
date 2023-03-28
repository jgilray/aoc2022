// AoC 2022 day 21

use regex::Regex;
use std::collections::HashMap;

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

#[derive(Debug, Clone)]
enum Operator {
    Minus,
    Plus,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
// monkey operation
struct MonkeyOp {
    lhs: String,
    rhs: String,
    op: Operator,
}

impl MonkeyOp {
    fn new(l: &str, r: &str, op: Operator) -> Self {
        Self {
            lhs: l.to_owned(),
            rhs: r.to_owned(),
            op,
        }
    }
}

#[derive(Debug, Clone)]
enum Monkey {
    Number(i64),
    Operation(MonkeyOp),
}

// recursive function that solves the expression starting at monkey m
fn resolve(mv: &Vec<Monkey>, hm: &HashMap<String, usize>, m: &str) -> Result<i64, String> {
    if let Some(idx) = hm.get(m) {
        match mv[*idx].clone() {
            Monkey::Number(n) => Ok(n),
            Monkey::Operation(mo) => {
                let lhs = resolve(mv, hm, &mo.lhs)?;
                let rhs = resolve(mv, hm, &mo.rhs)?;
                match mo.op {
                    Operator::Minus => Ok(lhs - rhs),
                    Operator::Plus => Ok(lhs + rhs),
                    Operator::Multiply => Ok(lhs * rhs),
                    Operator::Divide => Ok(lhs / rhs),
                }
            }
        }
    } else {
        Err(format!("monkey name not found: {}", m))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../monkey_operations.dat");
    let re_str = r#"^([[:alpha:]]+): ((\d+)|([[:alpha:]]+) ([\-\+/\*]) ([[:alpha:]]+))$"#;
    let re = Regex::new(re_str)?;

    let mut mv: Vec<Monkey> = vec![];
    let mut hm: HashMap<String, usize> = HashMap::new();

    let lines: Vec<&str> = input.split('\n').collect();

    for l in lines {
        if l.is_empty() {
            continue;
        }

        if re.is_match(l) {
            let caps = re.captures(l).unwrap();
            let name = caps.get(1).map_or("", |m| m.as_str());

            hm.insert(name.to_owned(), mv.len());

            if caps.get(3).is_none() {
                let lhs = caps.get(4).map_or("", |m| m.as_str());

                let op = caps.get(5).map_or("", |m| m.as_str());

                let rhs = caps.get(6).map_or("", |m| m.as_str());

                match op {
                    "-" => mv.push(Monkey::Operation(MonkeyOp::new(lhs, rhs, Operator::Minus))),
                    "+" => mv.push(Monkey::Operation(MonkeyOp::new(lhs, rhs, Operator::Plus))),
                    "/" => mv.push(Monkey::Operation(MonkeyOp::new(lhs, rhs, Operator::Divide))),
                    "*" => mv.push(Monkey::Operation(MonkeyOp::new(
                        lhs,
                        rhs,
                        Operator::Multiply,
                    ))),
                    _ => return Err(Box::new(Error::new(&format!("bad operator: {}", l)))),
                }
            } else {
                let num = caps
                    .get(2)
                    .map_or(0, |m| m.as_str().parse::<i64>().unwrap());

                mv.push(Monkey::Number(num));
            }
        } else {
            return Err(Box::new(Error::new(&format!("bad line in input: {}", l))));
        }
    }
    // part 1
    let ansa = resolve(&mv, &hm, "root")?;
    println!("aoc21a: {}", ansa);

    // part 2 - search for humn number that makes the root expression lhs equal to its rhs
    let ridx = hm.get("root").unwrap();
    let (rlhs, rrhs) = match mv[*ridx].clone() {
        Monkey::Operation(mo) => (mo.lhs, mo.rhs),
        _ => {
            return Err(Box::new(Error::new(&format!(
                "bad root expr: {:?}",
                mv[*ridx]
            ))))
        }
    };
    let hidx = hm.get("humn").unwrap();
    let mut bot = 1;
    let mut top = i64::MAX / 100;
    mv[*hidx] = Monkey::Number(bot);
    let lhvala = resolve(&mv, &hm, &rlhs)?;
    let rhvala = resolve(&mv, &hm, &rrhs)?;
    mv[*hidx] = Monkey::Number(top);
    let lhvalb = resolve(&mv, &hm, &rlhs)?;
    let rhvalb = resolve(&mv, &hm, &rrhs)?;
    let (cs, target, mut tt, mut tb) = if lhvala == lhvalb {
        (
            rrhs,
            lhvala,
            (lhvala - rhvalb).abs(),
            (lhvala - rhvala).abs(),
        )
    } else {
        (
            rlhs,
            rhvala,
            (rhvala - lhvalb).abs(),
            (rhvala - lhvala).abs(),
        )
    };

    while top > bot {
        let half = (top + bot) / 2;
        mv[*hidx] = Monkey::Number(half);
        let nv = resolve(&mv, &hm, &cs)?;
        if tt < tb {
            bot = half;
            tb = (target - nv).abs();
        } else {
            top = half;
            tt = (target - nv).abs();
        }
    }
    println!("aoc21b: {}", top);

    Ok(())
}
