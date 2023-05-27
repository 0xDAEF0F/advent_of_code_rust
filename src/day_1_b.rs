pub fn day_one_b(input: String) -> u32 {
    let mut raindeers: Vec<u32> = Vec::new();

    let mut count: u32 = 0;

    for line in input.lines() {
        let num = line.trim().parse::<u32>().unwrap_or(0);

        if num == 0 {
            raindeers.push(count);
            count = 0;
            continue;
        }

        count += num;
    }

    raindeers.sort_by(|a, b| b.cmp(a));

    raindeers.into_iter().take(3).sum()
}
