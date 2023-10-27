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

pub fn day_10_a(a: String) -> i32 {
    let instructions = parse_instructions(a);

    let mut state = State::new();

    for instruction in instructions {
        state.execute_instruction(instruction);
    }

    let a = state.cycle_count[19] * 20;
    let b = state.cycle_count[59] * 60;
    let c = state.cycle_count[99] * 100;
    let d = state.cycle_count[139] * 140;
    let e = state.cycle_count[179] * 180;
    let f = state.cycle_count[219] * 220;

    a + b + c + d + e + f
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
