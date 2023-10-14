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

    let mut max_scenic_score: usize = 0;
    for current_row_i in 0..rows {
        let current_row_iter = trees_grid.clone().enumerate()
            .filter(|(i, _)| i / cols == current_row_i)
            .map(|(_, tree)| tree);
        let current_row: String = current_row_iter.clone().collect();
        for current_col_i in 0..cols {
            let current_col_iter = trees_grid.clone().enumerate()
                .filter(|(i, _)| i % cols == current_col_i)
                .map(|(_, tree)| tree);
            let current_col: String = current_col_iter.clone().collect();
            println!("row {current_row_i}:\t{current_row}\ncol {current_col_i}:\t{current_col}");
            let current_tree = current_row_iter.clone()
                .nth(current_col_i).unwrap()
                .to_digit(BASE_TEN).unwrap();
            println!("tree r{current_row_i} c{current_col_i}:\t{current_tree}");

            // UP
            let up: String = current_col_iter.clone()
                .take(current_row_i).collect::<String>()
                .chars().rev().collect();
            let mut up_count = up.chars().take_while(|tree|
                tree.to_digit(BASE_TEN).unwrap() < current_tree
            ).count();
            if up_count < up.len() {
                up_count += 1;
            }

            // LEFT
            let left: String = current_row_iter.clone()
                .take(current_col_i).collect::<String>()
                .chars().rev().collect();
            let mut left_count = left.chars().take_while(|tree|
                tree.to_digit(BASE_TEN).unwrap() < current_tree
            ).count();
            if left_count < left.len() {
                left_count += 1;
            }

            // DOWN
            let down: String = current_col_iter
                .skip(current_row_i + 1)
                .collect();
            let mut down_count = down.chars().take_while(|tree|
                tree.to_digit(BASE_TEN).unwrap() < current_tree
            ).count();
            if down_count < down.len() {
                down_count += 1;
            }

            // RIGHT
            let right: String = current_row_iter.clone()
                .skip(current_col_i + 1)
                .collect();
            let mut right_count = right.chars().take_while(|tree|
                tree.to_digit(BASE_TEN).unwrap() < current_tree
            ).count();
            if right_count < right.len() {
                right_count += 1;
            }

            let scenic_score = up_count
                * left_count
                * down_count
                * right_count; 

            println!("up:\t[{up}] -> {up_count}");
            println!("left:\t[{left}] -> {left_count}");
            println!("down:\t[{down}] -> {down_count}");
            println!("right:\t[{right}] -> {right_count}");
            println!("> scenic score:\t{scenic_score}");

            if scenic_score > max_scenic_score {
                println!("> new max scenic score");
                max_scenic_score = scenic_score;
            }

            println!("---------------------------------------------------------------"); 
        }
    }
    println!("---------------------------------------------------------------"); 
    println!("> final max scenic score: {max_scenic_score}");
}
