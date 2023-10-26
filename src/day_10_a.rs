#[derive(Debug)]
enum Instr {
    NOOP,
    ADDX(i32),
}

struct State {
    total_count: i32,
    pending_to_add: i32,
    cycle_count: Vec<i32>,
}

impl State {
    fn new() -> State {
        State {
            total_count: 1,
            pending_to_add: 0,
            cycle_count: vec![],
        }
    }
    fn execute_instruction(&mut self, instr: Instr) {
        match instr {
            Instr::NOOP => {
                // During cycle
                // After cycle
                self.cycle_count.push(self.total_count);
                self.total_count += self.pending_to_add;
                self.pending_to_add = 0;
            }
            Instr::ADDX(a) => {
                self.cycle_count.push(self.total_count);
                self.total_count += self.pending_to_add;
                self.pending_to_add = a;
            }
        }
    }
}

pub fn day_10_a(a: String) -> usize {
    let instructions = parse_instructions(a);

    let mut state = State::new();

    for instruction in instructions {
        state.execute_instruction(instruction);
    }

    println!("{:?}", state.cycle_count);

    0
}

fn parse_instructions(str: String) -> Vec<Instr> {
    str.lines()
        .map(|l| {
            if l.starts_with("noop") {
                Instr::NOOP
            } else {
                let quantity: i32 = l.split_whitespace().nth(1).unwrap().parse().unwrap();
                Instr::ADDX(quantity)
            }
        })
        .collect()
}
