use advent_of_code::{day_1_a, day_1_b, day_2_a, day_2_b, day_3_a, day_3_b, day_4_a};
use std::fs;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input_day_1.txt")?;
    println!("Day 1A: {:?}", day_1_a::day_one_a(input));

    let input = fs::read_to_string("input_day_1.txt")?;
    println!("Day 1B: {:?}", day_1_b::day_one_b(input));

    let input = fs::read_to_string("input_day_2.txt")?;
    println!("Day 2A: {:?}", day_2_a::day_two_a(input));

    let input = fs::read_to_string("input_day_2.txt")?;
    println!("Day 2B: {:?}", day_2_b::day_two_b(input));

    let input = fs::read_to_string("input_day_3.txt")?;
    println!("Day 3A: {:?}", day_3_a::day_three_a(input));

    let input = fs::read_to_string("input_day_3.txt")?;
    println!("Day 3B: {:?}", day_3_b::day_three_b(input));

    let input = fs::read_to_string("input_day_4.txt")?;
    println!("Day 4A: {:?}", day_4_a::day_4_a(input));

    Ok(())
}
