// AoC 2022 day 20

#[derive(Clone, Copy)]
struct SeqElem {
    val: isize,
    next: usize,
    prev: usize,
}

// a single mixing operation
fn mix_one(cl: &mut [SeqElem], idx: usize) {
    // two important observations: 1) use rem_euclid to deal with negative numbers
    // 2) use len() - 1 because moving an element that much leaves the list unchanged
    let dist = cl[idx].val.rem_euclid((cl.len() - 1) as isize) as usize;

    if dist > 0 {
        let mut nidx = idx;
        if dist > cl.len() / 2 {
            for _ in 0..cl.len() - dist {
                nidx = cl[nidx].prev;
            }
        } else {
            for _ in 0..dist {
                nidx = cl[nidx].next;
            }
        }

        // move the SeqVal in the circular list
        cl[cl[idx].next].prev = cl[idx].prev;
        cl[cl[idx].prev].next = cl[idx].next;
        let p = nidx;
        let n = cl[p].next;
        cl[p].next = idx;
        cl[n].prev = idx;
        cl[idx].prev = p;
        cl[idx].next = n;
    }
}

// returns the sum of three values in the passed list
fn find_answer(cl: &[SeqElem]) -> isize {
    let zeroidx = cl
        .iter()
        .enumerate()
        .find_map(|(i, &s)| if s.val == 0 { Some(i) } else { None })
        .unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|i| {
            let mut loc = zeroidx;
            for _ in 0..i % cl.len() {
                loc = cl[loc].next;
            }
            cl[loc].val
        })
        .sum::<isize>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../encrypted_coords.dat");
    let mut circlist: Vec<SeqElem> = vec![];

    let lines: Vec<&str> = input.split('\n').collect();

    for l in lines {
        if l.is_empty() {
            continue;
        }

        let val = l.parse::<isize>()?;
        let idx = circlist.len();
        let prev = idx.saturating_sub(1);
        let next = idx + 1;

        circlist.push(SeqElem { val, next, prev });
    }

    // make the list circular
    let end = circlist.len() - 1;
    circlist[end].next = 0;
    circlist[0].prev = circlist.len() - 1;
    let mut circlist2 = circlist.clone();

    // part 1
    for idx in 0..circlist.len() {
        mix_one(&mut circlist, idx);
    }
    let ansa = find_answer(&circlist);
    println!("aoc20a: {}", ansa);

    // part 2 - multiplying the values by a key and mixing 10 times
    const KEY: isize = 811589153;
    for elem in circlist2.iter_mut() {
        elem.val *= KEY;
    }

    for _ in 0..10 {
        for idx in 0..circlist2.len() {
            mix_one(&mut circlist2, idx);
        }
    }
    let ansb = find_answer(&circlist2);
    println!("aoc20b: {}", ansb);

    Ok(())
}
