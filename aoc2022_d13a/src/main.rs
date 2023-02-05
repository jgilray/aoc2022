// AoC 2022 day 13

use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};

#[derive(Debug, Clone)]
// location or move step
struct List {
    items: Vec<Loc>,
}

impl List {
    fn new() -> Self {
        Self { items: vec![] }
    }
}
impl Eq for List {}
impl Ord for List {
    // follow the rules in order to compare two lists
    // returns 1 if l is first, 0 if equal and -1 if r is first
    fn cmp(&self, other: &List) -> Ordering {
        let mut litr = self.items.iter();
        let mut ritr = other.items.iter();

        let mut lnxt = litr.next();
        let mut rnxt = ritr.next();
        while rnxt.is_some() && lnxt.is_some() {
            match (lnxt.unwrap(), rnxt.unwrap()) {
                (Loc::Integer(li), Loc::Integer(ri)) => {
                    let cval = li.cmp(ri);
                    if cval != Ordering::Equal {
                        return cval;
                    }
                }

                (Loc::Integer(li), Loc::SubList(rlst)) => {
                    let mut llst = List::new();
                    llst.items.push(Loc::Integer(*li));
                    let cval = llst.cmp(rlst);
                    if cval != Ordering::Equal {
                        return cval;
                    }
                }

                (Loc::SubList(llst), Loc::Integer(ri)) => {
                    let mut rlst = List::new();
                    rlst.items.push(Loc::Integer(*ri));
                    let cval = llst.cmp(&rlst);
                    if cval != Ordering::Equal {
                        return cval;
                    }
                }

                (Loc::SubList(llst), Loc::SubList(rlst)) => {
                    let cval = llst.cmp(rlst);
                    if cval != Ordering::Equal {
                        return cval;
                    }
                }
            }
            lnxt = litr.next();
            rnxt = ritr.next();
        }

        if lnxt.is_none() && rnxt.is_none() {
            Ordering::Equal // they are equivalent lists
        } else if lnxt.is_none() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}
impl PartialOrd for List {
    fn partial_cmp(&self, other: &List) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for List {
    fn eq(&self, other: &List) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

#[derive(Debug, Clone)]
enum Loc {
    Integer(u8),
    SubList(List),
}

fn build_list(ch: &mut std::str::Chars) -> Result<List, String> {
    let mut list = List::new();
    let mut getting_num = false;
    let mut num: u8 = 0;

    while let Some(c) = ch.next() {
        match c {
            '[' => match build_list(ch) {
                Ok(lst) => list.items.push(Loc::SubList(lst)),
                Err(e) => return Err(e),
            },
            ']' => {
                if getting_num {
                    list.items.push(Loc::Integer(num));
                }
                return Ok(list);
            }
            '0'..='9' => {
                if getting_num {
                    num = num * 10 + (c as u8 - b'0');
                } else {
                    getting_num = true;
                    num = c as u8 - b'0';
                }
            }
            ',' => {
                if getting_num {
                    list.items.push(Loc::Integer(num));
                    getting_num = false;
                }
            }
            _ => return Err(format!("bad character in data: {}", c)),
        }
    }

    // if execution reaches here, there is a problem
    Err("incomplete list detected".to_string())
}

fn parse_line(s: &str) -> Result<List, String> {
    let mut ch = s.chars();

    // make sure the first char is an open paren
    match ch.next() {
        Some('[') => {
            let retval = build_list(&mut ch);
            if ch.next().is_some() {
                Err("build_line terminated prematurely - badly formed list".to_string())
            } else {
                retval
            }
        }
        _ => Err("all lines must start with '['".to_string()),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../packet_pairs.dat");
    let mut get_left: bool = true;
    let mut left: List = List::new();
    let mut right: List;
    let mut pair_idx = 0_u32;
    let mut correct_order_sum = 0_u32;

    // for part 2 create a list of all the lists including divider packets
    let mut list2 = List::new();
    let mut sublist = List::new();
    sublist.items.push(Loc::Integer(2));
    list2.items.push(Loc::SubList(sublist));
    let mut list6 = List::new();
    let mut sublist = List::new();
    sublist.items.push(Loc::Integer(6));
    list6.items.push(Loc::SubList(sublist));
    let mut all_lists: Vec<List> = vec![];
    all_lists.push(list2.clone());
    all_lists.push(list6.clone());

    let lines: Vec<&str> = input.split('\n').collect();

    for l in lines {
        if l.is_empty() {
            get_left = true;
            continue;
        }

        if get_left {
            left = parse_line(l)?;
            all_lists.push(left.clone());
            get_left = false;
        } else {
            right = parse_line(l)?;
            pair_idx += 1;
            if left <= right {
                correct_order_sum += pair_idx;
            }
            all_lists.push(right);
        }
    }

    // part 1
    println!("aoc13a: {}", correct_order_sum);

    // part 2 - find the indices of the divider packets
    all_lists.sort_unstable();
    let idx2 = all_lists.binary_search(&list2).unwrap();
    let idx6 = all_lists.binary_search(&list6).unwrap();
    println!("aoc13b: {}", (idx2 + 1) * (idx6 + 1));

    Ok(())
}
