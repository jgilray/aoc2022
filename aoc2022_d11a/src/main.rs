// AoC 2022 day 11

#[derive(Debug, Clone)]
// location or move step
struct Monkey {
    items: Vec<usize>,
    operator: char,
    operand: usize,
    testdiv: usize,
    trueidx: usize,
    falseidx: usize,
}

impl Monkey {
    fn new(
        items: Vec<usize>,
        operator: char,
        operand: usize,
        testdiv: usize,
        trueidx: usize,
        falseidx: usize,
    ) -> Self {
        Self {
            items,
            operator,
            operand,
            testdiv,
            trueidx,
            falseidx,
        }
    }
}

// perform a round of monkey business
fn round(mv: &mut [Monkey], mh: &mut [usize], part2: bool, m: usize) -> Result<(), String> {
    for midx in 0..mv.len() {
        for i in 0..mv[midx].items.len() {
            let worrylevel = match mv[midx].operator {
                '+' => {
                    if part2 {
                        (mv[midx].items[i] + mv[midx].operand) % m
                    } else {
                        (mv[midx].items[i] + mv[midx].operand) / 3
                    }
                }
                '*' => {
                    if part2 {
                        (mv[midx].items[i] * mv[midx].operand) % m
                    } else {
                        (mv[midx].items[i] * mv[midx].operand) / 3
                    }
                }
                '^' => {
                    if part2 {
                        (mv[midx].items[i] * mv[midx].items[i]) % m
                    } else {
                        (mv[midx].items[i] * mv[midx].items[i]) / 3
                    }
                }
                _ => return Err("bad operator".to_string()),
            };
            if worrylevel % mv[midx].testdiv == 0 {
                mv[mv[midx].trueidx].items.push(worrylevel);
            } else {
                mv[mv[midx].falseidx].items.push(worrylevel);
            }
            mh[midx] += 1;
        }
        mv[midx].items.clear();
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../monkey_business.dat");
    let mut monkeyvec: Vec<Monkey> = vec![];
    let mut mhandled: Vec<usize> = vec![]; // number of time each monkey handles items
    let mut mi: Vec<usize> = vec![];
    let mut mope: char = 'a';
    let mut mopa: usize = 0;
    let mut mtd: usize = 0;
    let mut mti: usize = 0;
    let mut mfi: usize;

    let lines: Vec<&str> = input.split('\n').collect();

    // fill monkeyvec with the monkey data
    for l in lines {
        if l.is_empty() {
            continue;
        }

        let words: Vec<&str> = l.trim().split(' ').collect();

        match words[1] {
            "items:" => {
                let items: Vec<usize> = words
                    .iter()
                    .skip(2)
                    .map(|w| {
                        let i: Vec<&str> = w.split(',').collect();
                        i[0].parse::<usize>().unwrap()
                    })
                    .collect();
                mi = items;
            }
            "new" => {
                // operator - '+' or '*'
                mope = words[4].chars().next().unwrap();

                // deal with squaring by using '^' as a symbol
                if mope == '*' && words[5] == "old" {
                    mope = '^';
                    mopa = 0;
                } else {
                    mopa = words[5].parse::<usize>().unwrap();
                }
            }

            // divisor
            "divisible" => mtd = words[3].parse::<usize>().unwrap(),

            // index of monkey to throw to if evenly divisible
            "true:" => mti = words[5].parse::<usize>().unwrap(),

            // index of monkey to throw to if not evenly divisible
            "false:" => {
                mfi = words[5].parse::<usize>().unwrap();

                // got all info create a Monkey on monkeyvec
                monkeyvec.push(Monkey::new(mi.to_owned(), mope, mopa, mtd, mti, mfi));
                mhandled.push(0);
            }
            _ => {}
        }
    }

    // create a modulus for part 2 that is a multiple of all the test divisors
    let mut modulus = 100;
    for m in &monkeyvec {
        modulus *= m.testdiv;
    }

    // clone the data structures for part 2
    let mut mv2 = monkeyvec.clone();
    let mut mh2 = mhandled.clone();

    // part 1
    for _ in 0..20 {
        round(&mut monkeyvec, &mut mhandled, false, modulus)?;
    }
    mhandled.sort_unstable();
    mhandled.reverse();

    // part 2
    for _ in 0..10000 {
        round(&mut mv2, &mut mh2, true, modulus)?;
    }
    mh2.sort_unstable();
    mh2.reverse();

    println!(
        "aoc11a: {}, aoc11b: {}",
        mhandled[0] * mhandled[1],
        mh2[0] * mh2[1]
    );

    Ok(())
}
