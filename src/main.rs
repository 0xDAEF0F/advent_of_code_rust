use advent_of_code::{challenge_one, challenge_two};
use std::fs;

fn main() -> std::io::Result<()> {
    // CHALLENGE 1
    let input = fs::read_to_string("input_challenge_one.txt")?;
    println!("Challenge one: {:?}", challenge_one::challenge_one(input));

    // CHALLENGE 2
    let input = fs::read_to_string("input_challenge_two.txt")?;
    println!("Challenge two: {:?}", challenge_two::challenge_two(input));

    Ok(())
}
