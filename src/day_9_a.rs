use std::collections::HashSet;

#[derive(Debug)]
enum Move {
    UP(u64),
    LEFT(u64),
    DOWN(u64),
    RIGHT(u64),
}

struct State {
    head_position: (u64, u64),
    tail_position: (u64, u64),
    tail_unique_board_positions: HashSet<(u64, u64)>,
}

impl State {
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

        if x1.abs_diff(x2) == 2 && y1.abs_diff(y2) == 2 {
            // diagonal move
            if x2 > x1 {
                // notch x1 && y1 one plus
            } else {
                // notch x2 && y2 one plus
            }
        } else if x1.abs_diff(x2) == 2 {
            // horizontal move
            if x2 > x1 {
                // notch to the
            }
        } else if y1.abs_diff(y2) == 2 {
            // vertical move
        } else {
            panic!("Unreachable");
        }
    }
}

pub fn day_9_a(a: String) -> u64 {
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

    println!("{:?}", a);

    0
}

fn execute_moves(moves: Vec<Move>) {
    for movement in moves {}
}

fn execute_movement(movement: Move, state: State) -> State {
    let (x, y) = state.head_position;

    let new_head_position = match movement {
        Move::UP(_) => (x, y + 1),
        Move::LEFT(_) => (x - 1, y),
        Move::DOWN(_) => (x, y - 1),
        Move::RIGHT(_) => (x + 1, y),
    };

    let tail = state.tail_position;

    if is_tail_stable(new_head_position, tail) {
        return State {
            head_position: new_head_position,
            tail_position: tail,
            tail_unique_board_positions: state.tail_unique_board_positions,
        };
    } else {
        // recompute the tail position
    }
}
