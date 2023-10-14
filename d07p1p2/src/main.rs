use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let lines = input.split("\n").filter(|line| !line.is_empty());

    let mut current_dir = String::new();
    let mut dirs_sizes: HashMap<String, usize> = HashMap::new();

    for line in lines {
        println!("--------------------------------------------\n{line}\n-----");
        let mut line_words = line.split(' '); // saved in variable so it can be used in default case
        let first_word = line_words.next().unwrap();
        match  first_word {
            "$" => {
                // "cd" or "ls" (dgaf about "ls")
                match line_words.next().unwrap() {
                    // if "cd" push next item to current_dir
                    // unless is ".." else pop last dir from current_dir
                    "cd" => {
                        let cd_arg = line_words.next().unwrap();
                        match cd_arg {
                            ".." => {
                                current_dir.pop(); // remove last "/"
                                let (previous_dir, _) = current_dir.rsplit_once('/').unwrap();
                                current_dir = String::from(previous_dir);
                                current_dir.push('/'); // add last "/" again
                            },
                            _ => {
                                let mut new_dir: String = cd_arg.to_owned();
                                if new_dir != "/" {
                                    new_dir.push('/');
                                }
                                current_dir.push_str(&new_dir);
                            }
                        }
                        println!("current_dir: {current_dir}");
                    },
                    _ => {}
                }
            },
            "dir" => {
                println!("this is dir, fuck it!");
            },
            _ => {
                println!("this is file");
                let file_size = first_word.parse::<usize>().unwrap();
                for ancestor in get_ancestors(current_dir.clone()) {
                    println!("adding {file_size} to {ancestor}");
                    match dirs_sizes.get(&ancestor) {
                        Some(dir_size) => { 
                            dirs_sizes.insert(ancestor, dir_size + file_size); 
                        }
                        None => { 
                            dirs_sizes.insert(ancestor, file_size); 
                        }
                    }
                }
            }
        }
    }
    println!("--------------------------------------------");
    let mut sum_lte_100000 = 0;
    let mut dirs_names: Vec<String> = dirs_sizes.clone().into_keys().collect();
    dirs_names.sort();
    for dir_name in dirs_names.clone() {
        let size = dirs_sizes.get(&dir_name).unwrap();
        println!("{}: {}", dir_name, size);
        if size < &100000 {
            sum_lte_100000 += size;
        }
    }
    println!("--------------------------------------------");
    println!("result p1: {sum_lte_100000}");
    println!("============================================");   
    // p2)
    // total_space:     70000000
    // update_size:     30000000
    // unused_space:    70000000 - dirs_sizes["/"] 
    // space_needed:    30000000 - unused_space
    //
    // > need to find the smallest directory with size >= space_needed

    let total_space = 70000000;
    let update_size = 30000000;
    let used_space = dirs_sizes.get("/").unwrap();
    let unused_space = total_space - used_space;
    let missing_space = update_size - unused_space; 
    println!("missing {missing_space} space");
    println!("--------------------------------------------");
    
    let mut lowest_gte_space = &usize::MAX;
    for dir_name in dirs_names {
        let size = dirs_sizes.get(&dir_name).unwrap();
        if size >= &missing_space && size < lowest_gte_space {
            println!("found {size}, keep searching...");
            lowest_gte_space = size;
        }
    }
    println!("--------------------------------------------");
    println!("result p2: {lowest_gte_space}");
    println!("============================================");   
}

fn get_ancestors(mut dir_path: String) -> Vec<String> {
    // remove first and last "/"
    dir_path.pop();

    let mut tokens_accum = String::new();
    let mut ancestors: Vec<String> = Vec::new();
    for token in dir_path.split('/') {
        tokens_accum.push_str(token);
        tokens_accum.push('/');
        ancestors.push(tokens_accum.clone());
    }

    ancestors
}
