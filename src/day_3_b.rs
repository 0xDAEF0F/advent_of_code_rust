use std::collections::HashSet;
use itertools::Itertools;

pub fn day_three_b(all_rucksacks: String) -> u32 {
    let mut count: u32 = 0;

    for chunk in all_rucksacks.lines().chunks(3).into_iter() {
        let mut intersection: Option<HashSet<char>> = None;

        for l in chunk.into_iter() {
            let set: HashSet<char> = l.chars().collect();

            intersection = match intersection {
                None => Some(set),
                Some(acc) => Some(&acc & &set)
            }
        }

        if let Some(hm) = intersection {
            count += hm.iter().map(char_to_value).sum::<u32>();
        }
    };

    count
}

fn char_to_value(&ch: &char) -> u32 {
    match ch {
        'a'..='z' => ch as u32 - 'a' as u32 + 1,
        'A'..='Z' => ch as u32 - 'A' as u32 + 27,
        _ => panic!("not a valid character")
    }
}