// AoC 2022 day 7

use std::collections::HashMap;

// add the passed size to all containing directories
fn store_directory_size(
    ds: &mut [usize],
    size: usize,
    hm: &HashMap<String, usize>,
    path: &[String],
) -> Result<(), String> {
    let mut dirname = "".to_string();
    for dir in path {
        dirname += dir;
        if let Some(idx) = hm.get(&dirname) {
            ds[*idx] += size;
        } else {
            return Err(format!("Directory name not found: {}", dirname));
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../directory_instrs.dat");
    let mut hm: HashMap<String, usize> = HashMap::new();
    let mut path: Vec<String> = vec![];
    let mut file_sum = 0;
    let mut dirsize: Vec<usize> = vec![];

    // register the root directory
    hm.insert("/".to_string(), 0);
    dirsize.push(0);

    let coms = input.split('\n').collect::<Vec<&str>>();

    for com in coms {
        if com.is_empty() {
            continue;
        }

        let wordv = com.split(' ').collect::<Vec<&str>>();
        match wordv[0] {
            "$" => {
                // command expected save dir size
                if file_sum > 0 {
                    store_directory_size(&mut dirsize, file_sum, &hm, &path)?;
                    file_sum = 0;
                }
                match wordv[1] {
                    "cd" => match wordv[2] {
                        "/" => {
                            path.clear();
                            path.push("/".to_string());
                        }
                        ".." => {
                            path.pop().unwrap();
                        }
                        _ => {
                            let dirname = wordv[2].to_string() + "/";
                            path.push(dirname);
                        }
                    },

                    &_ => {}
                }
            }

            "dir" => {
                // register a new directory path name with its index
                let mut dirname = path.iter().cloned().collect::<String>();
                dirname += wordv[1];
                dirname += "/";
                hm.insert(dirname, dirsize.len());
                dirsize.push(0);
            }

            _ => file_sum += wordv[0].parse::<usize>()?,
        }
    }
    // save final dir size
    store_directory_size(&mut dirsize, file_sum, &hm, &path)?;

    let ans: usize = dirsize.iter().filter(|&s| *s < 100000).sum();
    println!("aoc7a: {}", ans);

    let remaining = 70_000_000 - dirsize[0];
    let needed = 30_000_000 - remaining;

    let mut smallest = usize::MAX;
    for size in dirsize {
        if size >= needed && size < smallest {
            smallest = size;
        }
    }
    println!("aoc7b: {}", smallest);

    Ok(())
}
