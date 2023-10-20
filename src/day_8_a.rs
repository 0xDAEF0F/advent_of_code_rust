pub fn day_8_a(s: String) -> usize {
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

    let mut visible_trees = 0;
    for (i, row) in forest.iter().enumerate() {
        // first and last row
        if i == 0 || i == forest.len() - 1 {
            visible_trees += row.len();
            continue;
        }

        for (j, _) in row.iter().enumerate() {
            if j == 0 || j == row.len() - 1 {
                visible_trees += 1;
                continue;
            }

            // compare it to every neighbor
            if is_tree_visible((i, j), &forest) {
                visible_trees += 1;
            }
        }
    }

    visible_trees
}

fn is_tree_visible(coordinates: (usize, usize), forest: &Vec<Vec<u32>>) -> bool {
    let (i, j) = coordinates;

    let size_of_tree = forest[i][j];

    // all left neighbors
    let mut is_left_visible = true;
    for j in (0..j).rev() {
        let tree_to_compare = forest[i][j];
        if tree_to_compare >= size_of_tree {
            is_left_visible = false;
        }
    }

    // all down neighbors
    let mut is_down_visible = true;
    for i in i + 1..forest.len() {
        let tree_to_compare = forest[i][j];
        if tree_to_compare >= size_of_tree {
            is_down_visible = false;
        }
    }

    // all right neighbors
    let mut is_right_visible = true;
    for j in j + 1..forest[i].len() {
        let tree_to_compare = forest[i][j];
        if tree_to_compare >= size_of_tree {
            is_right_visible = false;
        }
    }

    // all up neighbors
    let mut is_up_visible = true;
    for i in (0..i).rev() {
        let tree_to_compare = forest[i][j];
        if tree_to_compare >= size_of_tree {
            is_up_visible = false;
        }
    }

    is_left_visible || is_down_visible || is_right_visible || is_up_visible
}
