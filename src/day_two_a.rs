pub fn day_two_a(input: String) -> u32 {
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
    let points_of_my_selection = match me {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Invalid selection for me"),
    };

    let outcome_of_round = match me {
        'X' => match opponent {
            'A' => 3,
            'B' => 0,
            'C' => 6,
            _ => panic!("Invalid selection for opponent"),
        },
        'Y' => match opponent {
            'A' => 6,
            'B' => 3,
            'C' => 0,
            _ => panic!("Invalid selection for opponent"),
        },
        'Z' => match opponent {
            'A' => 0,
            'B' => 6,
            'C' => 3,
            _ => panic!("Invalid selection for opponent"),
        },
        _ => panic!("Invalid selection for me"),
    };

    points_of_my_selection + outcome_of_round
}
