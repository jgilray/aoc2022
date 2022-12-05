// AoC 2022 day 5

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../crate_stacks.dat");
    const NUMSTACKS: usize = 9;
    let mut vsa: Vec<Vec<char>> = vec![vec![]; NUMSTACKS];
    let mut vsb: Vec<Vec<char>> = vec![vec![]; NUMSTACKS];
    let mut collecting_stacks = true;

    let coms = input.split('\n').collect::<Vec<&str>>();

    for com in coms {
        if com.is_empty() {
            collecting_stacks = false;
        } else if collecting_stacks {
            // building the crate stacks
            for (i, c) in com.chars().enumerate() {
                if i > 4 * NUMSTACKS + 2 {
                    break;
                }
                if i % 4 == 1 {
                    if c == '1' {
                        break;
                    } else if c != ' ' {
                        vsa[i / 4].insert(0, c);
                        vsb[i / 4].insert(0, c);
                    }
                }
            }
        } else {
            // executing commands
            let w = com.split(' ').collect::<Vec<&str>>();
            let num = w[1].parse::<usize>().unwrap();
            let from = w[3].parse::<usize>().unwrap() - 1;
            let dest = w[5].parse::<usize>().unwrap() - 1;
            let ins_idx = vsb[dest].len();

            for _ in 0..num {
                if let Some(elem) = vsa[from].pop() {
                    vsa[dest].push(elem);
                }
                if let Some(elem) = vsb[from].pop() {
                    vsb[dest].insert(ins_idx, elem);
                }
            }
        }
    }

    let mut ans: String = "".to_string();
    for stack in vsa.iter_mut().take(NUMSTACKS) {
        if let Some(elem) = stack.pop() {
            ans.push(elem);
        }
    }
    println!("aoc5a: {}", ans);

    ans = "".to_string();
    for stack in vsb.iter_mut().take(NUMSTACKS) {
        if let Some(elem) = stack.pop() {
            ans.push(elem);
        }
    }
    println!("aoc5b: {}", ans);

    Ok(())
}
