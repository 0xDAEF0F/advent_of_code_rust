use std::vec;

#[derive(Debug)]
enum Instr {
    NOOP,
    ADDX(i32),
}

struct State {
    total_count: i32,
    cycle_count: Vec<i32>,
}

impl State {
    fn new() -> State {
        State {
            total_count: 1,
            cycle_count: vec![],
        }
    }
    fn execute_instruction(&mut self, instr: Instr) {
        match instr {
            Instr::NOOP => {
                self.cycle_count.push(self.total_count);
            }
            Instr::ADDX(a) => {
                self.cycle_count.push(self.total_count);
                self.total_count += a;
            }
        }
    }
}

pub fn day_10_b(a: String) -> String {
    let instructions = parse_instructions(a);

    let mut state = State::new();

    for instruction in instructions {
        state.execute_instruction(instruction);
    }

    let mut screen: String = String::new();

    let cycles: Vec<_> = state.cycle_count.chunks(40).collect();

    for (_, row) in cycles.iter().enumerate() {
        for (j, x_register) in row.iter().enumerate() {
            let sprite_position = x_register % 39;
            let range = sprite_position - 1..=sprite_position + 1;
            if range.contains(&(j as i32)) {
                screen.push('#');
            } else {
                screen.push('.')
            }
        }
        screen.push('\n');
    }

    screen
}

fn parse_instructions(str: String) -> Vec<Instr> {
    str.lines()
        .flat_map(|l| {
            if l.starts_with("noop") {
                vec![Instr::NOOP].into_iter()
            } else {
                let quantity: i32 = l.split_whitespace().nth(1).unwrap().parse().unwrap();
                vec![Instr::NOOP, Instr::ADDX(quantity)].into_iter()
            }
        })
        .collect()
}
