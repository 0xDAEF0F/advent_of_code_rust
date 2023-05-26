use std::fs;
use advent_of_code::{day_one_a, day_one_b, day_two_a, day_two_b, day_three_a, day_three_b};

fn main() -> std::io::Result<()> {
    // Day One - A
    let input = fs::read_to_string("input_day_one.txt")?;
    println!("Day One A: {:?}", day_one_a::day_one_a(input));

    // Day One - B
    let input = fs::read_to_string("input_day_one.txt")?;
    println!("Day One B: {:?}", day_one_b::day_one_b(input));

    // Day Two - A
    let input = fs::read_to_string("input_day_two.txt")?;
    println!("Day Two A: {:?}", day_two_a::day_two_a(input));

    // Day Two - B
    let input = fs::read_to_string("input_day_two.txt")?;
    println!("Day Two B: {:?}", day_two_b::day_two_b(input));

    // Day Three - A
    let input = fs::read_to_string("input_day_three.txt")?;
    println!("Day Three A: {:?}", day_three_a::day_three_a(input));

    // Day Three - B
    let input = fs::read_to_string("input_day_three.txt")?;
    println!("Day Three B: {:?}", day_three_b::day_three_b(input));

    Ok(())
}
