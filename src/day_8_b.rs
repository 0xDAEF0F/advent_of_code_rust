pub fn day_8_b(s: String) -> usize {
    let forest: Vec<Vec<u32>> = s
        .lines()
        .map(|l| {
            let a: Vec<u32> = l
                .chars()
                .map(|c| c.to_digit(10).expect("Character to be digit."))
                .collect::<Vec<u32>>();
            a
        })
        .collect();

    let mut highest_scenic_score = 0;
    for (i, row) in forest.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let scenic_score = calculate_scenic_score((i, j), &forest);
            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }
        }
    }

    highest_scenic_score
}

fn calculate_scenic_score(coordinates: (usize, usize), forest: &Vec<Vec<u32>>) -> usize {
    let (i, j) = coordinates;
    let tree = forest[i][j];

    // left neighbors
    let mut left_visible_trees = 0;
    let mut is_view_blocked = false;
    for j in (0..j).rev() {
        let neighbor = forest[i][j];

        if is_view_blocked {
            break;
        }

        if neighbor < tree && !is_view_blocked {
            left_visible_trees += 1;
        } else {
            is_view_blocked = true;
            left_visible_trees += 1;
        }
    }

    // down neighbors
    let mut down_visible_trees = 0;
    let mut is_view_blocked = false;
    for i in i + 1..forest.len() {
        let neighbor = forest[i][j];

        if is_view_blocked {
            break;
        }

        if neighbor < tree && !is_view_blocked {
            down_visible_trees += 1;
        } else {
            is_view_blocked = true;
            down_visible_trees += 1;
        }
    }

    // right neighbors
    let mut right_visible_trees = 0;
    let mut is_view_blocked = false;
    for j in j + 1..forest[i].len() {
        let neighbor = forest[i][j];

        if is_view_blocked {
            break;
        }

        if neighbor < tree && !is_view_blocked {
            right_visible_trees += 1;
        } else {
            is_view_blocked = true;
            right_visible_trees += 1;
        }
    }

    // up neighbors
    let mut up_visible_trees = 0;
    let mut is_view_blocked = false;
    for i in (0..i).rev() {
        let neighbor = forest[i][j];

        if is_view_blocked {
            break;
        }

        if neighbor < tree && !is_view_blocked {
            up_visible_trees += 1;
        } else {
            is_view_blocked = true;
            up_visible_trees += 1;
        }
    }

    // scenic score formula
    left_visible_trees * down_visible_trees * right_visible_trees * up_visible_trees
}
