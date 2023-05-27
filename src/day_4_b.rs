pub fn day_4_b(s: String) -> u32 {
    s.lines()
        .map(|l| {
            let pair = line_to_pair(l);
            let is_overlap = is_overlap(pair);
            match is_overlap {
                true => 1,
                false => 0,
            }
        })
        .sum()
}

fn line_to_pair(l: &str) -> ((u32, u32), (u32, u32)) {
    let mut it = l.split(',');

    let a = parse_range(it.next().unwrap());
    let b = parse_range(it.next().unwrap());

    (a, b)
}

fn parse_range(range: &str) -> (u32, u32) {
    let mut splits = range.split('-');

    let start: u32 = splits.next().unwrap().parse().unwrap();
    let end: u32 = splits.next().unwrap().parse().unwrap();

    (start, end)
}

fn is_overlap(((a, b), (c, d)): ((u32, u32), (u32, u32))) -> bool {
    if (a >= c && a <= d) || (b >= c && b <= d) {
        true
    } else if (c >= a && c <= b) || (d >= a && d <= b) {
        true
    } else {
        false
    }
}
