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

        for (j, tree) in row.iter().enumerate() {
            if j == 0 || j == row.len() - 1 {
                visible_trees += 1;
                continue;
            }

            // compare it to every neighbor (hard part)
        }
    }

    visible_trees
}

fn is_tree_visible(coordinates: (usize, usize), forest: &Vec<Vec<u32>>) {
    let (i, j) = coordinates;

    let size_of_tree = forest[i][j];
    let mut is_visible = true;

    // all left neighbors
    for k in (0..i).rev() {
        let tree_to_compare = forest[k][j];
    }
}
