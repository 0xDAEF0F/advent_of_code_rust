use std::{collections::{HashSet, hash_set::Intersection}, str::Lines, hash::Hash,};

use itertools::{Itertools, Chunk};

pub fn day_three_b(all_rucksacks: String) -> u32 {

    for chunk in all_rucksacks.lines().chunks(3).into_iter() {
        let mut a: Vec<HashSet<char>> = vec![];

        for l in chunk.into_iter() {
            let set: HashSet<char> = l.chars().collect();
            a.push(set);
        }

        // a.into_iter().fold(None, |acc, set| )

    };

    0
}

fn foo(l: &str) -> HashSet<char> {
    l.chars().collect()
}


fn char_to_value(&ch: &char) -> u32 {
    match ch {
        'a'..='z' => ch as u32 - 'a' as u32 + 1,
        'A'..='Z' => ch as u32 - 'A' as u32 + 27,
        _ => panic!("not a valid character")
    }
}