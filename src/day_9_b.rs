use std::collections::HashSet;

#[derive(Debug)]
enum Move {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

struct State {
    current_positions: Vec<(i64, i64)>,
    tail_unique_board_positions: HashSet<(i64, i64)>,
}

impl State {
    fn new() -> Self {
        let mut hash_set = HashSet::new();
        hash_set.insert((0, 0));

        let positions = (0..=9).map(|_| (0, 0)).collect();

        Self {
            current_positions: positions,
            tail_unique_board_positions: hash_set,
        }
    }

    fn get_unique_tail_spots(&self) -> usize {
        self.tail_unique_board_positions.len()
    }

    fn execute_movement(&mut self, to: Move) {
        self.move_head(to);
        for i in 1..=9 {
            if !self.is_knot_stable(i) {
                self.stabilize_knot(i);
            }
        }
        let tail = self.current_positions[self.current_positions.len() - 1];
        self.tail_unique_board_positions.insert(tail);
    }

    fn move_head(&mut self, to: Move) {
        let (x, y) = self.current_positions[0];
        let head_ref = &mut self.current_positions[0];

        match to {
            Move::UP => *head_ref = (x, y + 1),
            Move::LEFT => *head_ref = (x - 1, y),
            Move::DOWN => *head_ref = (x, y - 1),
            Move::RIGHT => *head_ref = (x + 1, y),
        }
    }

    fn is_knot_stable(&self, which_knot: usize) -> bool {
        let (x1, y1) = self.current_positions[which_knot - 1];
        let (x2, y2) = self.current_positions[which_knot];

        if (0..=1).contains(&x1.abs_diff(x2)) && (0..=1).contains(&y1.abs_diff(y2)) {
            true
        } else {
            false
        }
    }

    fn stabilize_knot(&mut self, which_knot: usize) {
        let (x1, y1) = self.current_positions[which_knot - 1];
        let (x2, y2) = self.current_positions[which_knot];

        let back_knot = &mut self.current_positions[which_knot];

        if x1.abs_diff(x2) == 2 && y1.abs_diff(y2) == 0 {
            // horizontal move
            if x1 > x2 {
                *back_knot = (x2 + 1, y2);
            } else {
                *back_knot = (x2 - 1, y2);
            }
        } else if y1.abs_diff(y2) == 2 && x1.abs_diff(x2) == 0 {
            // vertical straight move
            if y1 > y2 {
                *back_knot = (x2, y2 + 1);
            } else {
                *back_knot = (x2, y2 - 1);
            }
        } else {
            // diagonal move
            if y1 > y2 {
                // diagonal up
                if x1 > x2 {
                    // diagonal up-right
                    *back_knot = (x2 + 1, y2 + 1);
                } else {
                    *back_knot = (x2 - 1, y2 + 1);
                }
            } else {
                // diagonal down
                if x1 > x2 {
                    // diagonal down-right
                    *back_knot = (x2 + 1, y2 - 1);
                } else {
                    *back_knot = (x2 - 1, y2 - 1);
                }
            }
        }
    }
}

pub fn day_9_b(a: String) -> usize {
    let all_moves: Vec<Move> = a
        .lines()
        .flat_map(|l| {
            let mut iter = l.split_whitespace();
            let direction = iter.next().expect("Direction should not be empty");
            let quantity: u64 = iter
                .next()
                .expect("Quantity should not be empty")
                .parse()
                .expect("Should be able to parse");

            let moves: Box<dyn Iterator<Item = Move>> = match direction {
                "U" => Box::new((0..quantity).map(|_| Move::UP)),
                "L" => Box::new((0..quantity).map(|_| Move::LEFT)),
                "D" => Box::new((0..quantity).map(|_| Move::DOWN)),
                "R" => Box::new((0..quantity).map(|_| Move::RIGHT)),
                _ => panic!("There is no other direction to move other than U, L, D, R."),
            };

            moves
        })
        .collect();

    let mut state = State::new();

    for movement in all_moves {
        state.execute_movement(movement);
    }

    state.get_unique_tail_spots()
}
