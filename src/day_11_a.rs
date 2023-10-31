struct Monkey {
    items: Vec<u32>,
    monkey_index: usize,
    inspected_items: u32,
    throw_idx_if_true: usize,
    throw_idx_if_false: usize,
}

impl Monkey {
    fn get_initial_items(monkey: u8) -> Vec<u32> {
        match monkey {
            0 => vec![85, 77, 77],
            1 => vec![80, 99],
            2 => vec![74, 60, 74, 63, 86, 92, 80],
            3 => vec![71, 58, 93, 65, 80, 68, 54, 71],
            4 => vec![97, 56, 79, 65, 58],
            5 => vec![77],
            6 => vec![99, 90, 84, 50],
            7 => vec![50, 66, 61, 92, 64, 78],
            _ => panic!("must be from 0..=7"),
        }
    }

    fn who_to_throw(monkey_idx: u8, flag: bool) -> usize {
        match monkey_idx {
            0 => match flag {
                true => 6,
                false => 7,
            },
            1 => match flag {
                true => 3,
                false => 5,
            },
            2 => match flag {
                true => 0,
                false => 6,
            },
            3 => match flag {
                true => 2,
                false => 4,
            },
            4 => match flag {
                true => 2,
                false => 0,
            },
            5 => match flag {
                true => 4,
                false => 3,
            },
            6 => match flag {
                true => 7,
                false => 1,
            },
            7 => match flag {
                true => 5,
                false => 1,
            },
            _ => panic!("must be from 0..=7"),
        }
    }

    fn execute_monkey_turn(&mut self, monkeys: &mut Vec<Monkey>) {
        for item in self.items.iter() {
            //
        }
    }

    fn get_new_worry_level(&self, worry_level: u32) -> u32 {
        match self.monkey_index {
            0 => worry_level * 7,
            1 => worry_level * 11,
            2 => worry_level + 8,
            3 => worry_level + 7,
            4 => worry_level + 5,
            5 => worry_level + 4,
            6 => worry_level * worry_level,
            7 => worry_level + 3,
            _ => panic!("Monkey does not exist!"),
        }
    }

    fn throw_item_to(&self, worry_level: u32) -> bool {
        match self.monkey_index {
            0 => worry_level % 19 == 0,
            1 => worry_level % 3 == 0,
            2 => worry_level % 13 == 0,
            3 => worry_level % 7 == 0,
            4 => worry_level % 5 == 0,
            5 => worry_level % 11 == 0,
            6 => worry_level % 17 == 0,
            7 => worry_level % 2 == 0,
            _ => panic!("Monkey does not exist!"),
        }
    }
}

pub fn day_11_a(str: String) {
    let mut monkeys: Vec<Monkey> = (0..=7)
        .map(|i| {
            let items = Monkey::get_initial_items(i);
            Monkey {
                items,
                monkey_index: i as usize,
                inspected_items: 0,
                throw_idx_if_true: Monkey::who_to_throw(i, true),
                throw_idx_if_false: Monkey::who_to_throw(i, false),
            }
        })
        .collect();

    for _ in 0..=20 {
        for monkey in 0..monkeys.len() {
            // iterate over the monkey's items
            let a = monkey.items.iter().map(|item| {
                let new_worry_level = monkey.get_new_worry_level(*item);
                let throw_to = match monkey.throw_item_to(new_worry_level) {
                    true => monkey.throw_idx_if_true,
                    false => monkey.throw_idx_if_false,
                };

                (new_worry_level, throw_to)
            });

            for (item, to) in a {
                &monkeys[to].items.push(item);
            }
        }
    }
}
