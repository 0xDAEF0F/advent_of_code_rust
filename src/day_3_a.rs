use std::collections::HashSet;

pub fn day_three_a(all_rucksacks: String) -> u32 {
    all_rucksacks.lines().map(rucksack_to_sum_of_intersection).sum()
}

fn rucksack_to_sum_of_intersection(rucksack: &str) -> u32 {
    let (a,b) = rucksack_to_compartments(&rucksack);

    let set_a: HashSet<char> = a.chars().collect();
    let set_b: HashSet<char> = b.chars().collect();

    let intersection: u32 = set_a.intersection(&set_b).map(|&c| char_to_value(c).unwrap_or(0)).sum();

    intersection
}

fn rucksack_to_compartments(str: &str) -> (&str, &str) {
    let half = str.len() / 2;

    str.split_at(half)
}

fn char_to_value(ch: char) -> Option<u32> {
    match ch {
        'a'..='z' => Some(ch as u32 - 'a' as u32 + 1),
        'A'..='Z' => Some(ch as u32 - 'A' as u32 + 27),
        _ => None
    }
}