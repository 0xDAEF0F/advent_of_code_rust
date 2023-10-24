use std::collections::HashSet;

#[derive(Debug)]
enum Move {
    UP(u64),
    LEFT(u64),
    DOWN(u64),
    RIGHT(u64),
}

struct State {
    head_position: (i64, i64),
    tail_position: (i64, i64),
    tail_unique_board_positions: HashSet<(i64, i64)>,
}

impl State {
    fn new() -> Self {
        let mut hs = HashSet::new();
        hs.insert((0, 0));
        Self {
            head_position: (0, 0),
            tail_position: (0, 0),
            tail_unique_board_positions: hs,
        }
    }

    fn get_unique_tail_spots(&self) -> usize {
        self.tail_unique_board_positions.len()
    }

    fn execute_movement(&mut self, to: Move) {
        match to {
            Move::UP(a) => {
                let mut a = a;
                while a != 0 {
                    self.move_head(Move::UP(0));
                    if !self.is_tail_stable() {
                        self.stabilize_tail();
                    }
                    a -= 1;
                }
            }
            Move::LEFT(a) => {
                let mut a = a;
                while a != 0 {
                    self.move_head(Move::LEFT(0));
                    if !self.is_tail_stable() {
                        self.stabilize_tail();
                    }
                    a -= 1;
                }
            }
            Move::DOWN(a) => {
                let mut a = a;
                while a != 0 {
                    self.move_head(Move::DOWN(0));
                    if !self.is_tail_stable() {
                        self.stabilize_tail();
                    }
                    a -= 1;
                }
            }
            Move::RIGHT(a) => {
                let mut a = a;
                while a != 0 {
                    self.move_head(Move::RIGHT(0));
                    if !self.is_tail_stable() {
                        self.stabilize_tail();
                    }
                    a -= 1;
                }
            }
        }
    }

    fn move_head(&mut self, to: Move) {
        match to {
            Move::UP(_) => self.head_position = (self.head_position.0, self.head_position.1 + 1),
            Move::LEFT(_) => self.head_position = (self.head_position.0 - 1, self.head_position.1),
            Move::DOWN(_) => self.head_position = (self.head_position.0, self.head_position.1 - 1),
            Move::RIGHT(_) => self.head_position = (self.head_position.0 + 1, self.head_position.1),
        }
    }

    fn is_tail_stable(&self) -> bool {
        let (x1, y1) = self.head_position;
        let (x2, y2) = self.tail_position;

        if (0..=1).contains(&x1.abs_diff(x2)) && (0..=1).contains(&y1.abs_diff(y2)) {
            true
        } else {
            false
        }
    }

    fn stabilize_tail(&mut self) {
        let (x1, y1) = self.head_position;
        let (x2, y2) = self.tail_position;

        if x1.abs_diff(x2) == 2 && y1.abs_diff(y2) == 0 {
            // horizontal move
            if x1 > x2 {
                self.tail_position = (x2 + 1, y2);
            } else {
                self.tail_position = (x2 - 1, y2);
            }
        } else if y1.abs_diff(y2) == 2 && x1.abs_diff(x2) == 0 {
            // vertical straight move
            if y1 > y2 {
                self.tail_position = (x2, y2 + 1);
            } else {
                self.tail_position = (x2, y2 - 1);
            }
        } else {
            // diagonal move
            if y1 > y2 {
                // diagonal up
                if x1 > x2 {
                    // diagonal up-right
                    self.tail_position = (x2 + 1, y2 + 1);
                } else {
                    self.tail_position = (x2 - 1, y2 + 1);
                }
            } else {
                // diagonal down
                if x1 > x2 {
                    // diagonal down-right
                    self.tail_position = (x2 + 1, y2 - 1);
                } else {
                    self.tail_position = (x2 - 1, y2 - 1);
                }
            }
        }

        self.tail_unique_board_positions.insert(self.tail_position);
    }
}

pub fn day_9_a(a: String) -> usize {
    let a: Vec<Move> = a
        .lines()
        .map(|l| {
            let mut iter = l.split_whitespace();
            let direction = iter.next().expect("Direction should not be empty");
            let quantity: u64 = iter
                .next()
                .expect("Quantity should not be empty")
                .parse()
                .expect("Should be able to parse");

            match direction {
                "U" => Move::UP(quantity),
                "L" => Move::LEFT(quantity),
                "D" => Move::DOWN(quantity),
                "R" => Move::RIGHT(quantity),
                _ => panic!("There is no other direction to move other than U, L, D, R."),
            }
        })
        .collect();

    let mut state = State::new();

    for movement in a {
        state.execute_movement(movement);
    }

    state.get_unique_tail_spots()
}
