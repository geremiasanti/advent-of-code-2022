fn main() {
    const BASE_TEN: u32 = 10;

    let _input = include_str!("./input0.txt");
    let input = include_str!("./input1.txt");
    println!("---------------------------------------------------------------");
    println!("---------------------------------------------------------------");

    let trees_grid = input.clone().chars().filter(|c| c.is_numeric());
    let rows = input.clone().lines().filter(|line| !line.is_empty()).count();
    let cols = input.split_once("\n").unwrap().0.chars().count();
    println!("{input}");
    println!("---------------------------------------------------------------");
    println!("nrows:\t{rows}\ncols:\t{cols}");
    println!("---------------------------------------------------------------");
    println!("---------------------------------------------------------------");
    /* TEST
    let second_row_index = 1;
    let mut second_row_iter = trees_grid.clone().enumerate()
        .filter(|(i, _)| i / cols == second_row_index)
        .map(|(_, tree)| tree);
    let second_row: String = second_row_iter.clone().collect();
    let third_col_index = 2;
    let third_col: String = trees_grid.clone().enumerate()
        .filter(|(i, _)| i % cols == third_col_index)
        .map(|(_, tree)| tree)
        .collect();
    let second_row_third_col_tree: char = second_row_iter.nth(third_col_index)
        .unwrap();
    println!("second_row:\t{second_row}\nthird_col:\t{third_col}\nsecond_row_third_col_tree:\t{second_row_third_col_tree}");
    println!("---------------------------------------------------------------"); 
    */

    let mut visible_trees: usize = 0;
    for current_row_i in 0..rows {
        let current_row_iter = trees_grid.clone().enumerate()
            .filter(|(i, _)| i / cols == current_row_i)
            .map(|(_, tree)| tree);
        let current_row: String = current_row_iter.clone().collect();
        for current_col_i in 0..cols {
            // edge
            if current_row_i == 0 || current_col_i == 0 || current_row_i == rows-1 || current_col_i == cols-1 {
                visible_trees += 1;
                println!("> tree on the edge, visible\nvisible_trees:\t{visible_trees}");
                println!("---------------------------------------------------------------"); 
                continue;
            }

            let current_col_iter = trees_grid.clone().enumerate()
                .filter(|(i, _)| i % cols == current_col_i)
                .map(|(_, tree)| tree);
            let current_col: String = current_col_iter.clone().collect();
            println!("row {current_row_i}:\t\t{current_row}\ncol {current_col_i}:\t\t{current_col}");
            let current_tree = current_row_iter.clone()
                .nth(current_col_i).unwrap()
                .to_digit(BASE_TEN).unwrap();
            println!("tree r{current_row_i} c{current_col_i}:\t{current_tree}");

            // up
            let gte_trees_up = current_col_iter.clone().take(current_row_i)
                .filter(|tree| 
                    tree.to_digit(BASE_TEN).unwrap() >= current_tree
                ).count();
            if gte_trees_up == 0 {
                println!("> visible from up");
                visible_trees += 1;
                continue;
            }
            // down
            let gte_trees_down = current_col_iter.skip(current_row_i + 1)
                .filter(|tree| 
                    tree.to_digit(BASE_TEN).unwrap() >= current_tree
                ).count();
            if gte_trees_down == 0 {
                println!("> visible from below");
                visible_trees += 1;
                continue;
            }
            // left
            let gte_trees_left = current_row_iter.clone().take(current_col_i)
                .filter(|tree| 
                    tree.to_digit(BASE_TEN).unwrap() >= current_tree
                ).count();
            if gte_trees_left == 0 {
                println!("> visible from left");
                visible_trees += 1;
                continue;
            }
            // right
            let gte_trees_right = current_row_iter.clone().skip(current_col_i + 1)
                .filter(|tree| 
                    tree.to_digit(BASE_TEN).unwrap() >= current_tree
                ).count();
            if gte_trees_right == 0 {
                println!("> visible from right");
                visible_trees += 1;
                continue;
            }

            println!("visible_trees:\t{visible_trees}");
            println!("---------------------------------------------------------------"); 
        }
    }
}
