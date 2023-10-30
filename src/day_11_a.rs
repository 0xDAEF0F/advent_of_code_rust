struct Monkey {
    items: Vec<u32>,
    throw_idx_if_true: u8,
    throw_idx_if_false: u8,
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

    fn who_to_throw(monkey_idx: u8, flag: bool) -> u8 {
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
}

pub fn day_11_a(str: String) {
    let monkeys: Vec<Monkey> = (0..=7)
        .map(|i| Monkey {
            items: Monkey::get_initial_items(i),
            throw_idx_if_true: Monkey::who_to_throw(i, true),
            throw_idx_if_false: Monkey::who_to_throw(i, false),
        })
        .collect();
}
