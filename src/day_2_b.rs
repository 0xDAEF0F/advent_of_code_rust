pub fn day_two_b(input: String) -> u32 {
    let scores: u32 = input
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            let opponent = chars.next().unwrap();
            let me = chars.next().unwrap();

            play_match(opponent, me)
        })
        .sum();

    scores
}

fn play_match(opponent: char, me: char) -> u32 {
    match opponent {
        'A' => match me {
            // Rock
            'X' => 0 + 3, // Lose = 0, Scissors = 3
            'Y' => 3 + 1, // Draw = 3, Rock = 1
            'Z' => 6 + 2, // Win = 6, Paper = 2
            _ => panic!("Invalid input"),
        },
        'B' => match me {
            // Paper
            'X' => 0 + 1, // Lose = 0, Rock = 1
            'Y' => 3 + 2, // Draw = 3, Paper = 2
            'Z' => 6 + 3, // Win = 6, Scissors = 3
            _ => panic!("Invalid input"),
        },
        'C' => match me {
            // Scissors
            'X' => 0 + 2, // Lose = 0, Paper = 2
            'Y' => 3 + 3, // Draw = 3, Scissors = 3
            'Z' => 6 + 1, // Win = 6, Rock = 1
            _ => panic!("Invalid input"),
        },
        _ => panic!("Invalid input"),
    }
}
