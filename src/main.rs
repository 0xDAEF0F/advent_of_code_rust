#[allow(unused_imports)]
use advent_of_code::{
    day_10_a, day_1_a, day_1_b, day_2_a, day_2_b, day_3_a, day_3_b, day_4_a, day_4_b, day_7_a,
    day_8_a, day_8_b, day_9_a, day_9_b,
};
use std::fs;

fn main() -> std::io::Result<()> {
    // let input = fs::read_to_string("input_day_1.txt")?;
    // println!("Day 1A: {:?}", day_1_a::day_one_a(input));
    //
    // let input = fs::read_to_string("input_day_1.txt")?;
    // println!("Day 1B: {:?}", day_1_b::day_one_b(input));
    //
    // let input = fs::read_to_string("input_day_2.txt")?;
    // println!("Day 2A: {:?}", day_2_a::day_two_a(input));
    //
    // let input = fs::read_to_string("input_day_2.txt")?;
    // println!("Day 2B: {:?}", day_2_b::day_two_b(input));
    //
    // let input = fs::read_to_string("input_day_3.txt")?;
    // println!("Day 3A: {:?}", day_3_a::day_three_a(input));
    //
    // let input = fs::read_to_string("input_day_3.txt")?;
    // println!("Day 3B: {:?}", day_3_b::day_three_b(input));
    //
    // let input = fs::read_to_string("input_day_4.txt")?;
    // println!("Day 4A: {:?}", day_4_a::day_4_a(input));
    //
    // let input = fs::read_to_string("input_day_4.txt")?;
    // println!("Day 4B: {:?}", day_4_b::day_4_b(input));
    //
    // let input = fs::read_to_string("input_day_7.txt")?;
    // println!("Day 7: {:?}", day_7_a::day_7_a(input));
    //
    // let input = fs::read_to_string("input_day_8.txt")?;
    // println!("Day 8A: {:?}", day_8_a::day_8_a(input));
    //
    // let input = fs::read_to_string("input_day_8.txt")?;
    // println!("Day 8B: {:?}", day_8_b::day_8_b(input));
    //
    // let input = fs::read_to_string("input_day_9.txt")?;
    // println!("Day 9A: {:?}", day_9_a::day_9_a(input));
    //
    // let input = fs::read_to_string("input_day_9.txt")?;
    // println!("Day 9B: {:?}", day_9_b::day_9_b(input));

    let input = fs::read_to_string("input_day_10.txt")?;
    println!("Day 9B: {:?}", day_10_a::day_10_a(input));

    Ok(())
}
